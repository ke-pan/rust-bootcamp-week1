use super::Executor;
use crate::genpass;
use clap::Args;

#[derive(Debug, Args)]
pub struct GenPassArgs {
    #[arg(short, long, default_value_t = 14)]
    pub length: u8,
    #[arg(short, long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(short, long, default_value_t = false)]
    pub symbol: bool,
    #[arg(short, long, default_value_t = false)]
    pub number: bool,
}

impl Executor for GenPassArgs {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = genpass(self.length, self.uppercase, self.symbol, self.number);
        println!("{}", password);
        Ok(())
    }
}
