mod b64;
mod csv;
mod http;
mod passgen;
mod text;

use std::{fs, io, io::Read};

pub use b64::*;
pub use csv::process_csv;
pub use http::serve;
pub use passgen::genpass;
pub use text::*;

fn get_content_from(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let input_content = match input {
        "-" => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        _ => fs::read(input)?,
    };
    Ok(input_content)
}
