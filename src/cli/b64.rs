use super::{validate_file, Executor};
use crate::{process_base64_decode, process_base64_encode};
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[derive(Subcommand, Debug)]
#[enum_dispatch(Executor)]
pub enum B64Command {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(EncodeArgs),
    #[command(name = "decode", about = "Base64 decode")]
    Decode(DecodeArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=parse_format, default_value = "standard")]
    pub format: Base64Format,
}

impl Executor for EncodeArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let encoded = process_base64_encode(&self.input, self.format)?;
        print!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=parse_format, default_value = "standard")]
    pub format: Base64Format,
}

impl Executor for DecodeArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let decoded = process_base64_decode(&self.input, self.format)?;
        print!("{}", String::from_utf8_lossy(&decoded));
        Ok(())
    }
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
