mod cli;
mod processor;

pub use cli::{
    B64Command, Base64Format, Opts, OutputFormat, SignatureFormat, SubCommand, TextCommand,
};
pub use processor::{
    generate, genpass, process_base64_decode, process_base64_encode, process_csv, sign, verify,
};
