use crate::cli::Base64Format;
use anyhow::Result;
use base64::prelude::*;

use super::get_content_from;

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let content = get_content_from(input)?;
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(content)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(content)?,
    };
    Ok(decoded)
}

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<String> {
    let content = get_content_from(input)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(content),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(content),
    };
    Ok(encoded)
}
