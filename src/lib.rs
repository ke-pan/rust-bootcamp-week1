mod cli;
mod processor;

pub use cli::{B64Command, Base64Format, Opts, OutputFormat, SubCommand};
pub use processor::{genpass, process_base64_decode, process_base64_encode, process_csv};
