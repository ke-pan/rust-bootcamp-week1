use clap::Parser;
use rcli::{genpass, process_csv, Opts, SubCommand};
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
                process_csv(&input, &output, format)?;
            } else {
                let output = format!("output.{}", format);
                process_csv(&input, &output, format)?;
            }
        }
        SubCommand::GenPass {
            length,
            uppercase,
            symbol,
            number,
        } => {
            let password = genpass(length, uppercase, symbol, number);
            println!("{}", password);
        }
    }
    Ok(())
}
