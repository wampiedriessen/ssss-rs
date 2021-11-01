use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use rs_ssss::shamir::ShamirStd;
use rs_ssss::shard::SsssShard;

mod encoding;
mod math;
pub mod shamir;
pub mod shard;

#[derive(Debug, StructOpt)]
enum Action {
    /// Creates secret-shards of the given input
    Shard {
        #[structopt(short, long)]
        threshold: u8,

        #[structopt(short, long = "number")]
        number_of_shards: u8,
    },
    /// Merges shards back together. When the threshold is reached, sensible output is given.
    Merge
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ssss-rs", about = "Shards a secret, and merges the pieces back together")]
struct SsssRsOpt {
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), short, long)]
    output: Option<PathBuf>,

    #[structopt(subcommand, name = "action", help = "Want to create, or combine shards?")]
    action: Action,
}

fn main() -> Result<(), String> {
    let opt = SsssRsOpt::from_args();

    let result = match opt.action {
        Action::Shard { threshold, number_of_shards } => create_shards(threshold, number_of_shards, opt.output),
        Action::Merge => merge_shards(opt.output),
    };

    if let Err(message) = result {
        println!("Unsuccessful execution of program:");
        println!("{}", message);
    };

    Ok(())
}

fn merge_shards(output: Option<PathBuf>) -> Result<(), String> {
    let input = std::io::stdin();
    let mut shards = Vec::<SsssShard>::new();

    let mut input_buffer = String::new();
    let mut line = 1;
    while input.read_line(&mut input_buffer).map_err::<String, _>(|_| "Could not read input!".into())? != 0 {
        shards.push(input_buffer.trim().parse().map_err(|x| format!("{} on line {}", x, line))?);
        input_buffer.clear();
        line += 1;
    }

    let secret = ShamirStd::merge_shards(&shards);

    let mut out = get_output(output)?;

    writeln!(out, "").map_err::<String, _>(|_| "Could not write output!".into())?;

    out.write_all(secret.as_slice()).map_err(|_| "Could not write output!".into())
}

fn create_shards(thresh: u8, num: u8, output: Option<PathBuf>) -> Result<(), String> {
    let mut input = std::io::stdin();
    let mut input_buffer = Vec::new();

    input.read_to_end(&mut input_buffer).map_err::<String, _>(|_| "Could not read input!".into())?;

    let shamir = ShamirStd::new(thresh, num);

    let shards = shamir.create_shards(input_buffer.as_slice());

    let mut out = get_output(output)?;

    for shard in shards {
        writeln!(out, "{}", shard).map_err::<String, _>(|_| "Could not write output!".into())?;
    }

    Ok(())
}

fn get_output(output: Option<PathBuf>) -> Result<Box<dyn Write>, String> {
    if let Some(file) = output {
        Ok(Box::new(std::fs::File::create(file).map_err::<String, _>(|_| "Could not open file!".into())?))
    }
    else { Ok(Box::new(std::io::stdout())) }
}
