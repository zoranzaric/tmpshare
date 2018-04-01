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

    let path = tmpshare::get_path(&hash).unwrap();
    println!("{:?}", path.as_path())
}