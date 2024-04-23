use clap::Parser;
use rcli::{Executor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.subcmd.execute().await
}
