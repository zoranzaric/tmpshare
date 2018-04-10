//! `tmpshare` is a tool to share files for a short time via HTTP.
extern crate tmpshare;
use tmpshare::cli::TmpShareOpt;

extern crate structopt;
use structopt::StructOpt;

pub fn main() {
    match TmpShareOpt::from_args() {
        TmpShareOpt::Add { filename } => match tmpshare::storage::add(filename.as_path()) {
            Ok(metadata) => println!("{}", metadata.hash),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
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
