//! Abstracting away cli argument parsing.
use std::path::PathBuf;

/// Arguments for the `tmpshare` tool.
#[derive(StructOpt, Debug)]
#[structopt(name = "tmpshare")]
pub enum TmpShareOpt {
    #[structopt(name = "add")]
    Add {
        #[structopt(help = "The file to add", parse(from_os_str))]
        filename: PathBuf
    },
    #[structopt(name = "serve")]
    Serve {
        #[structopt(long = "address", help = "Sets the address to bind the HTTP server to", default_value = "127.0.0.1")]
        address: String,
        #[structopt(long = "port", help = "Sets the port to bind the HTTP server to", default_value = "8080")]
        port: u16,
    },
    #[structopt(name = "list")]
    List {
    }
}