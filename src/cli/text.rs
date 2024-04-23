use super::{validate_file, Executor};
use crate::{decrypt, encrypt, generate, sign, verify};
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[derive(Subcommand, Debug)]
#[enum_dispatch(Executor)]
pub enum TextCommand {
    #[command(name = "sign", about = "Sign the message")]
    Sign(SignArgs),
    #[command(name = "verify", about = "Verify the signature")]
    Verify(VerifyArgs),
    #[command(name = "generate", about = "Generate a new key pair")]
    Generate(GenerateArgs),
    #[command(name = "encrypt", about = "Encrypt the message")]
    Encrypt(EncryptArgs),
    #[command(name = "decrypt", about = "Decrypt the message")]
    Decrypt(DecryptArgs),
}

#[derive(Debug, Args)]
pub struct GenerateArgs {}

impl Executor for GenerateArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        print!("{}", generate());
        Ok(())
    }
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

impl Executor for SignArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        print!("{}", sign(&self.input, &self.key, self.format)?);
        Ok(())
    }
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

impl Executor for VerifyArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let result = verify(&self.input, &self.signature, &self.key, self.format)?;
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct EncryptArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=validate_file)]
    pub key: String,
}

impl Executor for EncryptArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let encrypted = encrypt(&self.input, &self.key)?;
        print!("{}", encrypted);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct DecryptArgs {
    #[arg(short, long, value_parser=validate_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser=validate_file)]
    pub key: String,
}

impl Executor for DecryptArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let decrypted = decrypt(&self.input, &self.key)?;
        print!("{}", String::from_utf8_lossy(&decrypted));
        Ok(())
    }
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
