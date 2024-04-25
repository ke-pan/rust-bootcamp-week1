use anyhow::Ok;
use hifitime::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Option<String>,
    pub exp: f64,
    pub iss: Option<String>,
    pub aud: Option<String>,
}

impl Claims {
    pub fn try_new(
        sub: Option<String>,
        exp: Duration,
        iss: Option<String>,
        aud: Option<String>,
    ) -> anyhow::Result<Self> {
        let now = Epoch::now()?;
        let exp = now + exp;
        let exp = exp.to_unix_seconds();
        Ok(Self { sub, exp, iss, aud })
    }
}

const SECRET: &str = "secret";

pub fn sign_jwt(claims: Claims) -> anyhow::Result<String> {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> anyhow::Result<()> {
    let _ = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )?;

    Ok(())
}
