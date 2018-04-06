extern crate tmpshare;

use std::env::args;
use std::path::Path;

pub fn main() {
    let path = match args().nth(1) {
        Some(path) => path,
        None => {
            println!("usage: get <hash>");
            std::process::exit(1);
        }
    };

    let path = Path::new(&path);

    match tmpshare::storage::hash_file(path) {
        Ok(hash) => println!("{}", hash),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
