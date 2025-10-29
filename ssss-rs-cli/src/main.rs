mod inputoutput;

use std::io::BufRead;
use inputoutput::InputOutput;

#[derive(clap::Parser, Debug)]
enum Action {
    /// Creates secret-shards of the given input
    Shard {
        #[arg(short, long)]
        threshold: u8,

        #[arg(short, long = "number")]
        number_of_shards: u8,
    },
    /// Merges shards back together. When the threshold is reached, sensible output is given.
    Merge
}

#[derive(clap::Parser, Debug)]
#[command(name = "ssss-rs", about = "Shards a secret, and merges the pieces back together")]
struct SsssRsOpt {
    #[command(subcommand, name = "action", help = "Want to create, or combine shards?")]
    action: Action,

    #[command(flatten)]
    io: InputOutput,
}

fn main() -> Result<(), String> {
    use clap::Parser;
    let opt = SsssRsOpt::parse();

    let result = match opt.action {
        Action::Shard { threshold, number_of_shards } => create_shards(threshold, number_of_shards, &opt.io),
        Action::Merge => merge_shards(&opt.io),
    };

    if let Err(message) = result {
        println!("Unsuccessful execution of program:");
        println!("{}", message);
    };

    Ok(())
}

fn merge_shards(io: &InputOutput) -> Result<(), String> {
    let mut shards = Vec::<ssss_rs_core::SsssShard>::new();
    let input = io.get_input()?;

    let mut reader = std::io::BufReader::new(input);
    let mut input_buffer = String::new();
    let mut line = 1;
    while reader.read_line(&mut input_buffer).is_ok() {
        if input_buffer.is_empty() { break; }
        shards.push(input_buffer.trim().parse().map_err(|x| format!("{} on line {}", x, line))?);
        input_buffer.clear();
        line += 1;
    }

    let secret = ssss_rs_core::decode(shards.as_slice());

    let mut out = io.get_output()?;

    out.write_all(secret.as_slice()).map_err(|_| "Could not write output!")?;
    writeln!(out, "").map_err::<String, _>(|_| "Could not write output!".into())?;
    Ok(())
}

fn create_shards(thresh: u8, num: u8, io: &InputOutput) -> Result<(), String> {
let mut input_buffer = Vec::new();

    io.get_input()?.read_to_end(&mut input_buffer).map_err::<String, _>(|_| "Could not read input!".into())?;

    let options = ssss_rs_core::ShamirScheme::new(thresh, num);
    let shards = ssss_rs_core::encode(&options, input_buffer.as_slice());
    let mut out = io.get_output()?;

    for shard in shards {
        writeln!(out, "{}", shard).map_err::<String, _>(|_| "Could not write output!".into())?;
    }

    Ok(())
}
