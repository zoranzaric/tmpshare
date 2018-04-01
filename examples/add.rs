extern crate serde_json;
extern crate tmpshare;

use std::env::args;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

pub fn main() {
    let path = match args().nth(1) {
        Some(path) => path,
        None => {
            println!("usage: get <hash>");
            std::process::exit(1);
        }
    };

    let path = Path::new(&path);

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
