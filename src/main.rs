use clap::Parser;
use rcli::{
    genpass, process_base64_decode, process_base64_encode, process_csv, B64Command, Opts,
    SubCommand,
};
// rcli csv -i input.csv -o output.json --header --pretty -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv(cmd) => {
            if let Some(output) = cmd.output {
                process_csv(&cmd.input, &output, cmd.format)?;
            } else {
                let output = format!("output.{}", cmd.format);
                process_csv(&cmd.input, &output, cmd.format)?;
            }
        }
        SubCommand::GenPass(cmd) => {
            let password = genpass(cmd.length, cmd.uppercase, cmd.symbol, cmd.number);
            println!("{}", password);
        }
        SubCommand::Base64(cmd) => match cmd {
            B64Command::Encode(cmd) => {
                let encoded = process_base64_encode(&cmd.input, cmd.format)?;
                print!("{}", encoded);
            }
            B64Command::Decode(cmd) => {
                let decoded = process_base64_decode(&cmd.input, cmd.format)?;
                print!("{}", String::from_utf8_lossy(&decoded));
            }
        },
    }
    Ok(())
}
