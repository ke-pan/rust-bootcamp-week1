mod b64;
mod csv;
mod genpass;
mod http;
mod text;
pub use self::b64::*;
use self::csv::CsvArgs;
pub use self::csv::OutputFormat;
use self::genpass::GenPassArgs;
use self::http::HttpArgs;
pub use self::text::*;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Executor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Process CSV file")]
    Csv(CsvArgs),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassArgs),
    #[command(name = "base64", about = "Base64 encode/decode", subcommand)]
    Base64(B64Command),
    #[command(name = "text", about = "Sign text", subcommand)]
    Text(TextCommand),
    #[command(name = "http", about = "Serve local files and dir over HTTP")]
    Http(HttpArgs),
}

pub fn validate_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait Executor {
    async fn execute(&self) -> anyhow::Result<()>;
}
