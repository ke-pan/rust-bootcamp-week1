use std::{fs, io::Read};

use crate::cli::Base64Format;
use anyhow::Result;
use base64::prelude::*;

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let content = match input {
        "-" => {
            let mut buffer = Vec::new();
            std::io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        _ => fs::read(input)?,
    };
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(content)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(content)?,
    };
    Ok(decoded)
}

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<String> {
    let content = match input {
        "-" => {
            let mut buffer = Vec::new();
            std::io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        _ => fs::read(input)?,
    };
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(content),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(content),
    };
    Ok(encoded)
}
