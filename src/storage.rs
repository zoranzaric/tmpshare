//! Abstracting away `Metadata` storage and file access.
use chrono::prelude::*;
use chrono::Duration;

use std::fmt;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

extern crate checksums;

extern crate serde;
extern crate serde_json;

use glob::glob;

use uuid::Uuid;

use failure::{Error, ResultExt};

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

/// A collection of served files.
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub hash: String,
    pub entries: Vec<String>,
    #[serde(with = "my_date_format")]
    create_date: NaiveDateTime,
    #[serde(with = "my_date_format")]
    last_access_date: NaiveDateTime,
}

impl Collection {
    pub fn new(entries: Vec<Metadata>) -> Self {
        let my_uuid = Uuid::new_v4();
        Collection {
            hash: format!("{}", my_uuid).to_string(),
            entries: entries.iter().map(|e| e.hash.clone()).collect(),
            create_date: Utc::now().naive_local(),
            last_access_date: Utc::now().naive_local(),
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.hash, self.entries.join(", "))
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
pub fn hash_file(path: &Path) -> Result<String, Error> {
    if !path.exists() {
        return Err(format_err!("File not found"));
    }
    Ok(checksums::hash_file(path, checksums::Algorithm::SHA2256))
}

/// Retrieves the `Metadata` for a given hash.
pub fn get_metadata(hash: &str) -> Result<Metadata, Error> {
    let meta_path_filename = format!("{}.meta.json", hash);
    let meta_path = Path::new(&meta_path_filename);
    if !meta_path.exists() {
        return Err(format_err!("File not found"));
    }

    let mut meta = read_metadata(meta_path)?;
    meta.last_access_date = Utc::now().naive_local();

    write_metadata(meta_path, meta)
}

/// Retrieves the `Metadata` for a given file.
pub fn read_metadata(meta_path: &Path) -> Result<Metadata, Error> {
    let meta_file = File::open(meta_path).context("Could not open meta file")?;
    match serde_json::from_reader::<_, Metadata>(meta_file) {
        Ok(meta) => Ok(meta),
        Err(e) => Err(format_err!("Could not parse metadata: {}", e)),
    }
}

fn write_metadata(path: &Path, metadata: Metadata) -> Result<Metadata, Error> {
    let mut parent = match path.parent() {
        Some(parent) => PathBuf::from(parent),
        None => {
            return Err(format_err!("File not found"));
        }
    };

    let meta_file_name = format!("{}.meta.json", metadata.hash);
    parent.push(&meta_file_name);
    match File::create(&meta_file_name) {
        Ok(meta_file) => match serde_json::to_writer(meta_file, &metadata) {
            Ok(_) => Ok(metadata),
            Err(e) => Err(format_err!(
                "An error occured while serializing the metadata \"{:?}\": {}",
                metadata,
                e
            )),
        },
        Err(e) => Err(format_err!(
            "An error occured while opening the file \"{}\": {}",
            meta_file_name,
            e
        )),
    }
}

/// Constructs the `Metadata` for a given file and writes it to the filesystem.
pub fn add(path: &Path) -> Result<Metadata, Error> {
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
                return Err(format_err!("File not found"));
            }
        },
        None => {
            return Err(format_err!("File not found"));
        }
    };
    let metadata = Metadata::new(String::from(file_name), hash);

    write_metadata(path, metadata)
}

/// Constructs the `Collection` for given `Metadata`s and writes it to the filesystem.
pub fn add_collection(path: &Path, metadata: Vec<Metadata>) -> Result<Collection, Error> {
    let collection = Collection::new(metadata);

    write_collection(path, collection)
}

fn write_collection(path: &Path, collection: Collection) -> Result<Collection, Error> {
    let mut parent = match path.parent() {
        Some(parent) => PathBuf::from(parent),
        None => {
            return Err(format_err!("File not found"));
        }
    };

    let meta_file_name = format!("{}.collection.json", collection.hash);
    parent.push(&meta_file_name);
    match File::create(&meta_file_name) {
        Ok(meta_file) => match serde_json::to_writer(meta_file, &collection) {
            Ok(_) => Ok(collection),
            Err(e) => Err(format_err!(
                "An error occured while serializing the collectio \"{:?}\": {}",
                collection,
                e
            )),
        },
        Err(e) => Err(format_err!(
            "An error occured while opening the file \"{}\": {}",
            meta_file_name,
            e
        )),
    }
}

/// Retrieves the `Collection` for a given hash.
pub fn get_collection(hash: &str) -> Result<Collection, Error> {
    let meta_path_filename = format!("{}.collection.json", hash);
    let meta_path = Path::new(&meta_path_filename);
    if !meta_path.exists() {
        return Err(format_err!("File not found"));
    }

    let mut meta = read_collection(meta_path)?;
    meta.last_access_date = Utc::now().naive_local();

    write_collection(meta_path, meta)
}

/// Retrieves the metadata for a given collection file.
pub fn read_collection(meta_path: &Path) -> Result<Collection, Error> {
    let meta_file = File::open(meta_path).context("Could not open collection meta file")?;
    match serde_json::from_reader::<_, Collection>(meta_file) {
        Ok(meta) => Ok(meta),
        Err(e) => Err(format_err!("Could not parse collection metadata: {}", e)),
    }
}

/// Lists all the Metadata.
pub fn list() -> Vec<Metadata> {
    glob("*.meta.json")
        .unwrap()
        .filter(|entry| entry.is_ok())
        .map(|entry| match read_metadata(entry.unwrap().as_path()) {
            Ok(meta) => Some(meta),
            Err(_) => None,
        })
        .filter(|entry| entry.is_some())
        .map(|entry| entry.unwrap())
        .collect()
}

/// Remove files older than the supplied days.
pub fn cleanup(days: u16) {
    let duration = Duration::days(days as i64);

    let threshold = Utc::now().naive_local() - duration;

    for meta in list() {
        if meta.create_date < threshold {
            let meta_path_filename = format!("{}.meta.json", meta.hash);
            let meta_path = Path::new(&meta_path_filename);
            if !meta_path.exists() {
                continue;
            }

            fs::remove_file(&Path::new(meta.file_name.as_str())).unwrap();
            fs::remove_file(meta_path).unwrap();
            println!("deleted {}", meta);
        }
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
