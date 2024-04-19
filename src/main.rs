use clap::Parser;
use rcli::{process, Opts, SubCommand};
// rcli csv -i input.csv -o output.json --header --pretty -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv { input, output, .. } => process(&input, &output)?,
    }
    Ok(())
}
