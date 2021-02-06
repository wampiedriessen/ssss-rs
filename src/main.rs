use num_bigint::BigUint;
use std::fmt;
use std::io::{self, BufRead};

const MAXDEGREE: usize = 1024;
const MAXTOKENLEN: usize = 128;
const MAXLINELEN: usize = MAXTOKENLEN + 1 + 10 + 1 + MAXDEGREE / 4 + 10;

/* coefficients of some irreducible polynomials over GF(2) */
const irred_coeff: &'static [u8] = &[
    4, 3, 1, 5, 3, 1, 4, 3, 1, 7, 3, 2, 5, 4, 3, 5, 3, 2, 7, 4, 2, 4, 3, 1, 10, 9, 3, 9, 4, 2, 7,
    6, 2, 10, 9, 6, 4, 3, 1, 5, 4, 3, 4, 3, 1, 7, 2, 1, 5, 3, 2, 7, 4, 2, 6, 3, 2, 5, 3, 2, 15, 3,
    2, 11, 3, 2, 9, 8, 7, 7, 2, 1, 5, 3, 2, 9, 3, 1, 7, 3, 1, 9, 8, 3, 9, 4, 2, 8, 5, 3, 15, 14,
    10, 10, 5, 2, 9, 6, 2, 9, 3, 2, 9, 5, 2, 11, 10, 1, 7, 3, 2, 11, 2, 1, 9, 7, 4, 4, 3, 1, 8, 3,
    1, 7, 4, 1, 7, 2, 1, 13, 11, 6, 5, 3, 2, 7, 3, 2, 8, 7, 5, 12, 3, 2, 13, 10, 6, 5, 3, 2, 5, 3,
    2, 9, 5, 2, 9, 7, 2, 13, 4, 3, 4, 3, 1, 11, 6, 4, 18, 9, 6, 19, 18, 13, 11, 3, 2, 15, 9, 6, 4,
    3, 1, 16, 5, 2, 15, 14, 6, 8, 5, 2, 15, 11, 2, 11, 6, 2, 7, 5, 3, 8, 3, 1, 19, 16, 9, 11, 9, 6,
    15, 7, 6, 13, 4, 3, 14, 13, 3, 13, 6, 3, 9, 5, 2, 19, 13, 6, 19, 10, 3, 11, 6, 5, 9, 2, 1, 14,
    3, 2, 13, 3, 1, 7, 5, 4, 11, 9, 8, 11, 6, 5, 23, 16, 9, 19, 14, 6, 23, 10, 2, 8, 3, 2, 5, 4, 3,
    9, 6, 4, 4, 3, 2, 13, 8, 6, 13, 11, 1, 13, 10, 3, 11, 6, 5, 19, 17, 4, 15, 14, 7, 13, 9, 6, 9,
    7, 3, 9, 7, 1, 14, 3, 2, 11, 8, 2, 11, 6, 4, 13, 5, 2, 11, 5, 1, 11, 4, 1, 19, 10, 3, 21, 10,
    6, 13, 3, 1, 15, 7, 5, 19, 18, 10, 7, 5, 3, 12, 7, 2, 7, 5, 1, 14, 9, 6, 10, 3, 2, 15, 13, 12,
    12, 11, 9, 16, 9, 7, 12, 9, 3, 9, 5, 2, 17, 10, 6, 24, 9, 3, 17, 15, 13, 5, 4, 3, 19, 17, 8,
    15, 6, 3, 19, 6, 1,
];

struct Params {
    threshold: i32,
    number: i32,
    token: String,
}

struct Opts {
    showversion: bool,
    help: bool,
    quiet: bool,
    hex: bool,
    diffusion: bool,
    security: bool,
    params: Params,
}

struct Shard {
    shards_magnitude: u8,
    shard_number: u8,
    val: BigUint,
}

impl fmt::Display for Shard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hexval = self.val.to_str_radix(16);
        write!(
            f,
            "{:0width$}-{}",
            self.shard_number,
            hexval,
            width = self.shards_magnitude as usize
        )
    }
}

impl Opts {
    fn new() -> Opts {
        Opts {
            showversion: false,
            help: false,
            quiet: false,
            hex: false,
            diffusion: false,
            security: false,
            params: Params {
                threshold: -1,
                number: -1,
                token: "".to_string(),
            },
        }
    }
}

fn main() {
    let mut opts = Opts::new();

    opts.params.threshold = 2;
    opts.params.threshold = 10;
    opts.params.token = "hunter2".to_string();

    let shards = Split(opts).unwrap();

    for shard in shards {
        println!("{}", shard);
    }
}

fn validate(params: &Params) -> Result<(), &'static str> {
    if params.threshold < 2 {
        return Err("Invalid parameters: invalid threshold value");
    }
    if params.number < params.threshold {
        return Err("Invalid parameters: number of shares smaller than threshold");
    }
    if params.token.len() > MAXTOKENLEN {
        return Err("Invalid parameters: token too long");
    }
    return Ok(());
}

fn Split(opts: Opts) -> Result<Vec<Shard>, &'static str> {
    validate(&opts.params)?;

    let fmt_len = (opts.params.number as f32).log(10.0).floor() as u8;

    let mut input = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut input)
        .expect("Could not read from stdin");

    if input.len() > MAXTOKENLEN {
        return Err("Invalid input: Token too long to process");
    }

    return Ok(vec![
        Shard {
            val: BigUint::from(8 as u128),
            shard_number: 1,
            shards_magnitude: fmt_len,
        },
        Shard {
            val: BigUint::from(5 as u128),
            shard_number: 2,
            shards_magnitude: fmt_len,
        },
    ]);
}
