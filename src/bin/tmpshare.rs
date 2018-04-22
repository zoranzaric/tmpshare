//! `tmpshare` is a tool to share files for a short time via HTTP.
extern crate tmpshare;
use tmpshare::cli::TmpShareOpt;

extern crate structopt;
use structopt::StructOpt;

pub fn main() {
    match TmpShareOpt::from_args() {
        TmpShareOpt::Add { filenames } => {
            let metadata: Vec<tmpshare::storage::Metadata> = filenames
                .clone()
                .iter()
                .map(
                    |filename| match tmpshare::storage::add(filename.as_path()) {
                        Ok(metadata) => metadata,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    },
                )
                .collect();

            match metadata.len() {
                0 => eprintln!("TODO: handle no argument"), // TODO hanlde no filename
                1 => println!("{}", metadata.first().unwrap().hash),
                _ => {
                    match tmpshare::storage::add_collection(filenames.first().unwrap(), metadata) {
                        Ok(collection) => println!("{}", collection.hash),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    };
                }
            }
        }
        TmpShareOpt::Serve { address, port } => tmpshare::http::serve(&address, port),
        TmpShareOpt::List {} => {
            for meta in tmpshare::storage::list() {
                println!("{}", meta);
            }
        }
        TmpShareOpt::Cleanup { days } => {
            tmpshare::storage::cleanup(days);
        }
    }
}
