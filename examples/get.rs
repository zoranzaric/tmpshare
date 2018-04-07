extern crate tmpshare;

use std::env::args;

pub fn main() {
    let hash = match args().nth(1) {
        Some(hash) => hash,
        None => {
            println!("usage: get <hash>");
            std::process::exit(1);
        }
    };

    let meta = tmpshare::storage::get_metadata(&hash).unwrap();
    println!("{:?}", meta)
}
