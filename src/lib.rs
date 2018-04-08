//! `tmpshare` is a tool to share files.
//! 
//! ## Usage
//! 
//! ```text
//! $ echo "Hello World" > hello-world
//! 
//! $ tmpshare add hello-world
//! D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26
//! 
//! $ tmpshare list
//! D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26: hello-world
//! 
//! $ tmpshare serve
//! Serving from http://127.0.0.1:8080
//! 
//! $ curl http://127.0.0.1:8080/get/D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26
//! Hello World
//! ```
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate chrono;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate structopt;

pub mod cli;
pub mod http;
pub mod storage;
