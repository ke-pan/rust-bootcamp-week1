use crate::{sign_jwt, verify_jwt, Claims, Executor};
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;
use hifitime::Duration;

#[derive(Debug, Subcommand)]
#[enum_dispatch(Executor)]
pub enum JwtCommand {
    #[command(name = "sign", about = "Encode JWT token")]
    Sign(JwtEncodeArgs),
    #[command(name = "verify", about = "Decode JWT token")]
    Verify(JwtDecodeArgs),
}

#[derive(Debug, Args)]
pub struct JwtEncodeArgs {
    #[arg(long, help = "Subject of the token, optional")]
    pub sub: Option<String>,
    #[arg(long, value_parser = parse_duration, default_value = "7 days", help = "Token expiration time, example: 1 hour, 1 s, 1 d, 2 minutes")]
    pub exp: Duration,
    #[arg(long, help = "Audience of the token, optional")]
    pub iss: Option<String>,
    #[arg(long, help = "Issuer of the token, optional")]
    pub aud: Option<String>,
}

#[derive(Debug, Args)]
pub struct JwtDecodeArgs {
    #[arg(long)]
    pub token: String,
}

impl Executor for JwtEncodeArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let token = sign_jwt(Claims::try_new(
            self.sub.clone(),
            self.exp,
            self.iss.clone(),
            self.aud.clone(),
        )?)?;
        print!("{}", token);
        Ok(())
    }
}

fn parse_duration(s: &str) -> Result<Duration, &'static str> {
    s.parse().map_err(|_| "Invalid duration")
}

impl Executor for JwtDecodeArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        verify_jwt(&self.token)?;
        print!("Token is valid");
        Ok(())
    }
}
