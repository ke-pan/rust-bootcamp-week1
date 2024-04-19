mod cli;
mod processor;

pub use cli::{Opts, OutputFormat, SubCommand};
pub use processor::process;
