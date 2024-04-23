use crate::process_csv;
use clap::Args;
use std::fmt::Display;
use std::str::FromStr;

use super::{validate_file, Executor};

#[derive(Debug, Args)]
pub struct CsvArgs {
    #[arg(short, long, value_parser = validate_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

impl Executor for CsvArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        if let Some(output) = &self.output {
            process_csv(&self.input, output, self.format)
        } else {
            let output = format!("output.{}", self.format);
            process_csv(&self.input, &output, self.format)
        }
    }
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
