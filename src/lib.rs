//! `tmpshare` is a tool to share files.
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
