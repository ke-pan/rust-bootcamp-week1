use std::str::FromStr;

use clap::{Args, Subcommand};

use super::validate_file;

#[derive(Subcommand, Debug)]
pub enum TextCommand {
    #[command(name = "sign", about = "Sign the message")]
    Sign(SignArgs),
    #[command(name = "verify", about = "Verify the signature")]
    Verify(VerifyArgs),
    #[command(name = "generate", about = "Generate a new key pair")]
    Generate,
}

#[derive(Debug, Args)]
pub struct SignArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=parse_format, default_value = "blake")]
    pub format: SignatureFormat,
    #[arg(short, long, value_parser=validate_file)]
    pub key: String,
}

#[derive(Debug, Args)]
pub struct VerifyArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(short, long, value_parser=parse_format, default_value = "blake")]
    pub format: SignatureFormat,
    #[arg(short, long, value_parser=validate_file)]
    pub key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum SignatureFormat {
    Blake,
    Ed25519,
}

impl FromStr for SignatureFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake" => Ok(SignatureFormat::Blake),
            "ed25519" => Ok(SignatureFormat::Ed25519),
            _ => Err("Invalid format"),
        }
    }
}

fn parse_format(format: &str) -> Result<SignatureFormat, &'static str> {
    format.parse()
}
