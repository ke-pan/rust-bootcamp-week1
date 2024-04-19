use clap::{Parser, Subcommand};
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Csv {
        #[arg(short, long, value_parser = validate_input_file)]
        input: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long, value_parser = parse_format, default_value = "json")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err("Invalid format"),
        }
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    format.parse()
}
