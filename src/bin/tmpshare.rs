//! `tmpshare` is a tool to share files for a short time via HTTP.
extern crate tmpshare;
use tmpshare::cli::TmpShareOpt;

extern crate structopt;
use structopt::StructOpt;

extern crate glob;
use glob::glob;

pub fn main() {
    match TmpShareOpt::from_args() {
        TmpShareOpt::Add { filename } => {
            match tmpshare::storage::add(filename.as_path()) {
                Ok(metadata) => println!("{}", metadata.hash),
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        },
        TmpShareOpt::Serve { address, port } => {
            tmpshare::http::serve(&address, port)
        },
        TmpShareOpt::List { }=> {
            for entry in glob("*.meta.json").unwrap() {
                match entry {
                    Ok(path) => {
                        match tmpshare::storage::read_metadata(path.as_path()) {
                            Ok(meta) => println!("{}", meta),
                            Err(e) => eprintln!("{}", e),
                        }
                    },
                    Err(_) => { },
                }
            }
        }
    }
}
