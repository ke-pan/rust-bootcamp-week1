use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassCommand {
    #[arg(short, long, default_value_t = 14)]
    pub length: u8,
    #[arg(short, long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(short, long, default_value_t = false)]
    pub symbol: bool,
    #[arg(short, long, default_value_t = false)]
    pub number: bool,
}
