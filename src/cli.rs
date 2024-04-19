use clap::{Parser, Subcommand};
use std::path::Path;

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
        #[arg(short, long, default_value = "output.json")]
        output: String,
        #[arg(long, default_value_t = true)]
        header: bool,
        #[arg(short, long, default_value_t = true)]
        pretty: bool,
        #[arg(short, long, default_value_t = ',')]
        delimiter: char,
    },
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
