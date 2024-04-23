mod cli;
mod processor;

pub use cli::{
    B64Command, Base64Format, Opts, OutputFormat, SignatureFormat, SubCommand, TextCommand,
};
pub use processor::*;
