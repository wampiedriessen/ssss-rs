mod inputoutput;

use inputoutput::InputOutput;
use structopt::StructOpt;

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

    #[structopt(flatten)]
    io: InputOutput,

    #[structopt(subcommand, name = "action", help = "Want to create, or combine shards?")]
    action: Action,
}

fn main() -> Result<(), String> {
    let opt = SsssRsOpt::from_args();

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
    let mut shards = Vec::<ssss_rs::SsssShard>::new();
    let mut input = io.get_input()?;

    let mut input_buffer = String::new();
    let mut line = 1;
    while input.read_to_string(&mut input_buffer).map_err::<String, _>(|_| "Could not read input!".into())? != 0 {
        shards.push(input_buffer.trim().parse().map_err(|x| format!("{} on line {}", x, line))?);
        input_buffer.clear();
        line += 1;
    }

    let secret = ssss_rs::decode(shards.as_slice());

    let mut out = io.get_output()?;

    writeln!(out, "").map_err::<String, _>(|_| "Could not write output!".into())?;

    out.write_all(secret.as_slice()).map_err(|_| "Could not write output!".into())
}

fn create_shards(thresh: u8, num: u8, io: &InputOutput) -> Result<(), String> {
let mut input_buffer = Vec::new();

    io.get_input()?.read_to_end(&mut input_buffer).map_err::<String, _>(|_| "Could not read input!".into())?;

    let options = ssss_rs::ShamirScheme::new(thresh, num);
    let shards = ssss_rs::encode(&options, input_buffer.as_slice());
    let mut out = io.get_output()?;

    for shard in shards {
        writeln!(out, "{}", shard).map_err::<String, _>(|_| "Could not write output!".into())?;
    }

    Ok(())
}
