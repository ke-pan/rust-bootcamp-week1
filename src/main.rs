use anyhow::Ok;
use clap::Parser;
use rcli::{
    decrypt, encrypt, generate, genpass, process_base64_decode, process_base64_encode, process_csv,
    sign, verify, B64Command, Opts, SubCommand, TextCommand,
};
// rcli csv -i input.csv -o output.json --header --pretty -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Csv(cmd) => {
            if let Some(output) = cmd.output {
                process_csv(&cmd.input, &output, cmd.format)
            } else {
                let output = format!("output.{}", cmd.format);
                process_csv(&cmd.input, &output, cmd.format)
            }
        }
        SubCommand::GenPass(cmd) => {
            let password = genpass(cmd.length, cmd.uppercase, cmd.symbol, cmd.number);
            println!("{}", password);
            Ok(())
        }
        SubCommand::Base64(cmd) => match cmd {
            B64Command::Encode(cmd) => {
                let encoded = process_base64_encode(&cmd.input, cmd.format)?;
                print!("{}", encoded);
                Ok(())
            }
            B64Command::Decode(cmd) => {
                let decoded = process_base64_decode(&cmd.input, cmd.format)?;
                print!("{}", String::from_utf8_lossy(&decoded));
                Ok(())
            }
        },
        SubCommand::Text(cmd) => match cmd {
            TextCommand::Sign(cmd) => {
                print!("{}", sign(&cmd.input, &cmd.key, cmd.format)?);
                Ok(())
            }
            TextCommand::Verify(cmd) => {
                let result = verify(&cmd.input, &cmd.signature, &cmd.key, cmd.format)?;
                println!("{}", result);
                Ok(())
            }
            TextCommand::Generate => {
                print!("{}", generate());
                Ok(())
            }
            TextCommand::Encrypt(cmd) => {
                let encrypted = encrypt(&cmd.input, &cmd.key)?;
                print!("{}", encrypted);
                Ok(())
            }
            TextCommand::Decrypt(cmd) => {
                let decrypted = decrypt(&cmd.input, &cmd.key)?;
                print!("{}", String::from_utf8_lossy(&decrypted));
                Ok(())
            }
        },
    }
}
