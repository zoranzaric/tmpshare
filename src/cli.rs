//! Abstracting away cli argument parsing.
use std::path::PathBuf;

/// Arguments for the `tmpshare` tool.
#[derive(StructOpt, Debug)]
#[structopt(name = "tmpshare", about = "A tool to share files")]
pub enum TmpShareOpt {
    #[structopt(name = "add", about = "Adds files to tmpshare")]
    Add {
        #[structopt(help = "The file to add", parse(from_os_str))]
        filename: PathBuf,
    },
    #[structopt(name = "serve", about = "Serves file via HTTP")]
    Serve {
        #[structopt(long = "address", help = "Sets the address to bind the HTTP server to",
                    default_value = "127.0.0.1")]
        address: String,
        #[structopt(long = "port", help = "Sets the port to bind the HTTP server to",
                    default_value = "8080")]
        port: u16,
    },
    #[structopt(name = "list", about = "Lists files served by tmpshare")]
    List {},
    #[structopt(name = "cleanup", about = "Purges old files")]
    Cleanup {
        #[structopt(help = "Maximum age of a file")]
        days: u16,
    },
}
