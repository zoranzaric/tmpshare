//! Abstracting away HTTP serving.
use std::path::Path;

use rocket;
use rocket::config::{Config, Environment};
use rocket::http::uri::Segments;
use rocket::request::FromSegments;
use rocket::response::{self, content::Html, NamedFile, Response};
use rocket::Request;

use upspin;

use tempfile::tempdir;

use failure::Error;

/// A served file.
pub struct TmpShareFile {
    file: NamedFile,
    file_name: String,
}

#[derive(Debug)]
pub struct UpspinPath {
    path: String,
}

impl<'a> FromSegments<'a> for UpspinPath {
    type Error = Error;

    fn from_segments(segments: Segments<'a>) -> Result<Self, Self::Error> {
        Ok(UpspinPath {
            path: segments.collect::<Vec<_>>().join("/"),
        })
    }
}

impl<'r> response::Responder<'r> for TmpShareFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.file.respond_to(req)?)
            .raw_header(
                "Content-Disposition",
                format!("attachment; filename={}", self.file_name),
            )
            .ok()
    }
}

#[get("/get/<hash>")]
fn get(hash: String) -> Option<TmpShareFile> {
    match super::storage::get_metadata(&hash) {
        Ok(metadata) => match NamedFile::open(Path::new(&metadata.file_name)) {
            Ok(named_file) => Some(TmpShareFile {
                file: named_file,
                file_name: metadata.file_name.to_string(),
            }),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

#[get("/list/<hash>")]
fn list(hash: String) -> Option<Html<String>> {
    match super::storage::get_collection(&hash) {
        Ok(collection) => {
            let lis = collection
                .entries
                .iter()
                .map(|e| format!("<li><a href=\"/get/{}\">{}</a></li>", e, e))
                .collect::<Vec<_>>()
                .join("\n");
            let html = format!(
                "<html><body><h1>Collection: {}</h1><ul>{}</ul></body></html>",
                collection.hash, lis
            );
            Some(Html(html.to_string()))
        }
        Err(_) => None,
    }
}

#[get("/upspin/<upspin_path..>")]
fn upspin(upspin_path: UpspinPath) -> Option<TmpShareFile> {
    let upspin_path: upspin::UpspinPath = match upspin_path.path.as_str().parse() {
        Ok(upspin_path) => upspin_path,
        Err(_) => {
            return None;
        }
    };

    let tmpdir = match tempdir() {
        Ok(tmpdir) => tmpdir,
        Err(_) => {
            return None;
        }
    };

    let local_path = tmpdir.path().join(upspin_path.file_name());

    match upspin_path.get(&local_path) {
        Ok(()) => {}
        Err(_) => {
            return None;
        }
    };

    match NamedFile::open(local_path) {
        Ok(named_file) => Some(TmpShareFile {
            file: named_file,
            file_name: upspin_path.file_name().to_string(),
        }),
        Err(_) => None,
    }
}

#[error(404)]
fn not_found(_req: &Request) -> String {
    "🤷‍♂️".to_string()
}

/// Helper to start the HTTP server.
pub fn serve(address: &str, port: u16) {
    match Config::build(Environment::Production)
        .address(address)
        .port(port)
        .finalize()
    {
        Ok(config) => {
            println!("Serving from http://{}:{}", address, port);
            rocket::custom(config, false)
                .mount("/", routes![get, list, upspin])
                .catch(errors![not_found])
                .launch();
        }
        Err(e) => {
            eprintln!("Error while configuring the web server: {}", e);
        }
    }
}
