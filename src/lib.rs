//! `tmpshare` is a tool to share files.
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate chrono;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod http;
pub mod storage;
