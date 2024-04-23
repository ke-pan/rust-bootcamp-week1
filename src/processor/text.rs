use super::get_content_from;
use crate::{genpass, SignatureFormat};
use base64::prelude::*;
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey};
use std::{fs, str::FromStr};

pub fn generate() -> String {
    genpass(32, true, true, true)
}

pub fn verify(
    input: &str,
    signature: &str,
    keyfile: &str,
    format: SignatureFormat,
) -> anyhow::Result<bool> {
    let input_content = get_content_from(input)?;
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
    let input_content = get_content_from(input)?;
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

pub fn encrypt(input: &str, keyfile: &str) -> anyhow::Result<String> {
    let input_content = get_content_from(input)?;
    let key = fs::read(keyfile)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let mut ciphertext = cipher
        .encrypt(&nonce, input_content.as_ref())
        .map_err(anyhow::Error::msg)?;
    let mut nonce = nonce.to_vec();
    nonce.append(&mut ciphertext);
    Ok(BASE64_STANDARD.encode(&nonce))
}

pub fn decrypt(input: &str, keyfile: &str) -> anyhow::Result<Vec<u8>> {
    let input_content = get_content_from(input)?;
    let input_content: Vec<u8> = BASE64_STANDARD.decode(input_content)?;
    let nonce = GenericArray::from_slice(&input_content[..12]);
    let key = fs::read(keyfile)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)?;
    cipher
        .decrypt(nonce, &input_content[12..])
        .map_err(anyhow::Error::msg)
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
