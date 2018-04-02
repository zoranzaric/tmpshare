#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
use rocket::config::{Config, Environment};
use rocket::response::NamedFile;

extern crate tmpshare;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

#[get("/get/<hash>")]
fn get(hash: String) -> NamedFile {
    let path = tmpshare::get_path(&hash).unwrap();
    NamedFile::open(&path).unwrap()
}

pub fn main() {
    let matches = App::new("tmpshare")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a file to tmpshare")
                .version(crate_version!())
                .author(crate_authors!())
                .arg(
                    Arg::with_name("FILEPATH")
                        .help("The file to add")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Serves file via HTTP")
                .version(crate_version!())
                .author(crate_authors!())
                .arg(
                    Arg::with_name("address")
                        .long("address")
                        .help("Sets the addres to bind the HTTP server to")
                        .default_value("127.0.01")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .help("Sets the port to bind the HTTP server to")
                        .default_value("8080")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("add") => {
            let matches = matches.subcommand_matches("add").unwrap();

            let filepath = matches.value_of("FILEPATH").unwrap();

            let path = Path::new(filepath);

            let metadata = match tmpshare::add(&path) {
                Ok(metadata) => metadata,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };
            let mut parent = match path.parent() {
                Some(parent) => PathBuf::from(parent),
                None => {
                    std::process::exit(1);
                }
            };

            let meta_file_name = format!("{}.meta.json", metadata.hash);
            parent.push(&meta_file_name);
            let mut meta_file = File::create(meta_file_name).unwrap();
            let _ = meta_file.write_all(serde_json::to_string(&metadata).unwrap().as_bytes());

            println!("{}", metadata.hash);
        }
        Some("serve") => {
            let matches = matches.subcommand_matches("serve").unwrap();

            let address = matches.value_of("address").unwrap();
            let port: u16 = matches.value_of("port").unwrap().parse().unwrap();

            println!("{}:{}", address, port);

            let config = Config::build(Environment::Staging)
                .address(address)
                .port(port)
                .finalize()
                .unwrap();

            rocket::custom(config, true)
                .mount("/", routes![get])
                .launch();
        }
        _ => {
            eprintln!("Unknown subcommand");
        }
    }
}
