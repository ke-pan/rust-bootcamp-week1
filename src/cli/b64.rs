use std::str::FromStr;

use clap::{Args, Subcommand};

use super::validate_file;

#[derive(Subcommand, Debug)]
pub enum B64Command {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(EncodeCommand),
    #[command(name = "decode", about = "Base64 decode")]
    Decode(DecodeCommand),
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err("Invalid format"),
        }
    }
}

fn parse_format(format: &str) -> Result<Base64Format, &'static str> {
    format.parse()
}
