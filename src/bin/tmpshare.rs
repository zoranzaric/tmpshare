extern crate tmpshare;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

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
                        .help("Sets the address to bind the HTTP server to")
                        .default_value("127.0.0.1")
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
            // `unwrap`ing ok because we have the add command
            let matches = matches.subcommand_matches("add").unwrap();

            // `unwrap`ing ok because FILEPATH is required
            let filepath = matches.value_of("FILEPATH").unwrap();

            let path = Path::new(filepath);

            let metadata = match tmpshare::storage::add(&path) {
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
            match File::create(&meta_file_name) {
                Ok(mut meta_file) => {
                    match serde_json::to_string(&metadata) {
                        Ok(json_string) => {
                            let _ = meta_file.write_all(json_string.as_bytes());
                            println!("{}", metadata.hash);
                        },
                        Err(e) => {
                            eprintln!("An error occured while serializing the metadata \"{:?}\": {}", metadata, e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("An error occured while opening the file \"{}\": {}", meta_file_name, e);
                }
            }

        }
        Some("serve") => {
            // `unwrap`ing ok because we have the serve command
            let matches = matches.subcommand_matches("serve").unwrap();

            // `unwrap`ing ok because address has a default
            let address = matches.value_of("address").unwrap();
            // `unwrap`ing ok because port has a default
            match matches.value_of("port").unwrap().parse::<u16>() {
                Ok(port) => {
                    println!("{}:{}", address, port);

                    tmpshare::http::serve(address, port);
                },
                Err(e) => {
                    eprintln!("Error while parsing port \"{}\": {}", matches.value_of("port").unwrap(), e);
                }
            }
        }
        _ => {
            eprintln!("Unknown subcommand");
        }
    }
}
