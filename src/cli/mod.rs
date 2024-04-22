mod b64;
mod csv;
mod genpass;
mod text;

use clap::{Parser, Subcommand};
use std::path::Path;

pub use self::b64::{B64Command, Base64Format};
use self::csv::CsvCommand;
pub use self::csv::OutputFormat;
use self::genpass::GenPassCommand;
pub use self::text::{SignatureFormat, TextCommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Process CSV file")]
    Csv(CsvCommand),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassCommand),
    #[command(name = "base64", about = "Base64 encode/decode", subcommand)]
    Base64(B64Command),
    #[command(name = "text", about = "Sign text", subcommand)]
    Text(TextCommand),
}

pub fn validate_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
