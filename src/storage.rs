//! Abstracting away `Metadata` storage and file access.
use chrono::prelude::*;

use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

extern crate checksums;

extern crate serde;
extern crate serde_json;

/// All the metadata for a served file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub file_name: String,
    pub hash: String,
    #[serde(with = "my_date_format")]
    create_date: NaiveDateTime,
    #[serde(with = "my_date_format")]
    last_access_date: NaiveDateTime,
}

impl Metadata {
    pub fn new(file_name: String, hash: String) -> Self {
        Metadata {
            file_name,
            hash,
            create_date: Utc::now().naive_local(),
            last_access_date: Utc::now().naive_local(),
        }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.hash, self.file_name)
    }
}

mod my_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Calculates the Sha256 hash for a given file.
pub fn hash_file(path: &Path) -> Result<String, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    Ok(checksums::hash_file(path, checksums::Algorithm::SHA2256))
}

/// Retrieves the `Metadata` for a given hash.
pub fn get_metadata(hash: &str) -> Result<Metadata, io::Error> {
    let meta_path_filename = format!("{}.meta.json", hash);
    let meta_path = Path::new(&meta_path_filename);
    if !meta_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let mut meta = read_metadata(meta_path)?;
    meta.last_access_date = Utc::now().naive_local();

    write_metadata(meta_path, meta)
}

/// Retrieves the `Metadata` for a given file.
pub fn read_metadata(meta_path: &Path) -> Result<Metadata, io::Error> {
    match File::open(meta_path) {
        Ok(meta_file) => {
            match serde_json::from_reader::<_, Metadata>(meta_file) {
                Ok(meta) => Ok(meta),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not parse metadata: {}", e),
                )),
            }
        }
        Err(e) => Err(e),
    }
}

/// Constructs the `Metadata` for a given file and writes it to the filesystem.
pub fn add(path: &Path) -> Result<Metadata, io::Error> {
    let hash = match hash_file(path) {
        Ok(hash) => hash,
        Err(err) => {
            return Err(err);
        }
    };
    let file_name = match path.file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(file_name) => file_name,
            None => {
                return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
            }
        },
        None => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    };
    let metadata = Metadata::new(String::from(file_name), hash);

    write_metadata(path, metadata)
}

fn write_metadata(path: &Path, metadata: Metadata) -> Result<Metadata, io::Error> {
    let mut parent = match path.parent() {
        Some(parent) => PathBuf::from(parent),
        None => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    };

    let meta_file_name = format!("{}.meta.json", metadata.hash);
    parent.push(&meta_file_name);
    match File::create(&meta_file_name) {
        Ok(meta_file) => match serde_json::to_writer(meta_file, &metadata) {
            Ok(_) => {
                Ok(metadata)
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "An error occured while serializing the metadata \"{:?}\": {}",
                    metadata, e
                ),
            )),
        },
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "An error occured while opening the file \"{}\": {}",
                meta_file_name, e
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn parsing_metadata_json_works() {
        let metadata_json_str = r#"{
            "file_name": "TODO.md",
            "hash": "D76A099F5201CBD3C6DADDBDB56C1CF5FF8210198B862AFB92E919D492DC3751",
            "create_date": "2018-04-01 20:40:00",
            "last_access_date": "2018-04-01 20:40:00"
        }"#;

        let metadata: super::Metadata = serde_json::from_str(metadata_json_str).unwrap();

        let file_name = metadata.file_name;

        assert_eq!("TODO.md", file_name);
    }
}
