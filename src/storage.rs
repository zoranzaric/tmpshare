use chrono::prelude::*;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::Read;

extern crate checksums;

extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub file_name: String,
    pub hash: String,
    #[serde(with = "my_date_format")] create_date: NaiveDateTime,
    #[serde(with = "my_date_format")] last_access_date: NaiveDateTime,
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

pub fn hash_file(path: &Path) -> Result<String, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    Ok(checksums::hash_file(path, checksums::Algorithm::SHA2256))
}

pub fn get_path(hash: &str) -> Result<PathBuf, io::Error> {
    let meta_path_filename = format!("{}.meta.json", hash);
    let meta_path = Path::new(&meta_path_filename);
    if !meta_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    match File::open(meta_path) {
        Ok(mut meta_file) => {
            let mut meta_contents = String::new();
            let _ = meta_file.read_to_string(&mut meta_contents);
            match serde_json::from_str::<Metadata>(meta_contents.as_str()) {
                Ok(meta) => Ok(PathBuf::from(meta.file_name.as_str())),
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Could not parse metadata: {}", e)))
            }
        },
        Err(e) => Err(e)
    }
}

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
    Ok(Metadata::new(String::from(file_name), hash))
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
