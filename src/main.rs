use clap::Parser;
use rcli::{process, Opts, SubCommand};
// rcli csv -i input.csv -o output.json --header --pretty -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv {
            input,
            output,
            format,
        } => {
            if let Some(output) = output {
                process(&input, &output, format)?;
            } else {
                let output = format!("output.{}", format);
                process(&input, &output, format)?;
            }
        }
    }
    Ok(())
}
