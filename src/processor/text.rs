use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey};
use std::{
    fs,
    io::{self, Read},
    str::FromStr,
};

use crate::{genpass, SignatureFormat};

pub fn generate() -> String {
    genpass(32, true, true, true)
}

pub fn verify(
    input: &str,
    signature: &str,
    keyfile: &str,
    format: SignatureFormat,
) -> anyhow::Result<bool> {
    let input_content = match input {
        "-" => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        _ => fs::read(input)?,
    };
    let key = fs::read(keyfile)?;
    match format {
        SignatureFormat::Blake => {
            Ok(blake3::keyed_hash(key[..32].try_into()?, &input_content).to_string() == signature)
        }
        _ => {
            let key = SigningKey::from_bytes(key[..32].try_into()?);
            let signature = Signature::from_str(signature)?;
            match key.verify(&input_content, &signature) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }
}

pub fn sign(input: &str, keyfile: &str, format: SignatureFormat) -> anyhow::Result<String> {
    let input_content = match input {
        "-" => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        _ => fs::read(input)?,
    };
    let key = fs::read(keyfile)?;
    match format {
        SignatureFormat::Blake => {
            Ok(blake3::keyed_hash(key[..32].try_into()?, &input_content).to_string())
        }
        _ => {
            let mut key = SigningKey::from_bytes(key[..32].try_into()?);
            let signature = key.sign(&input_content);
            Ok(signature.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake() {
        let key = "fixtures/blake.txt";
        let input = "Cargo.toml";
        let signature = sign(input, key, SignatureFormat::Blake).unwrap();
        assert!(verify(input, &signature, key, SignatureFormat::Blake).unwrap());
    }

    #[test]
    fn test_ed25519() {
        let key = "fixtures/blake.txt";
        let input = "Cargo.toml";
        let signature = sign(input, key, SignatureFormat::Ed25519).unwrap();
        assert!(verify(input, &signature, key, SignatureFormat::Ed25519).unwrap());
    }
}
