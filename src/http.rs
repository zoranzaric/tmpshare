//! Abstracting away HTTP serving.
use std::path::Path;

use rocket;
use rocket::Request;
use rocket::config::{Config, Environment};
use rocket::response;
use rocket::response::{NamedFile, Response};

/// A served file.
pub struct TmpShareFile {
    file: NamedFile,
    file_name: String,
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
                .mount("/", routes![get])
                .catch(errors![not_found])
                .launch();
        }
        Err(e) => {
            eprintln!("Error while configuring the web server: {}", e);
        }
    }
}
