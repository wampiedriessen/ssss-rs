use fraction::GenericFraction;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use std::fmt;

type SecretType = i64;
type ShardType = Point;

struct Point {
    x: SecretType,
    y: SecretType,
}

struct Shard {
    shards_magnitude: u8,
    shard_number: u8,
    val: ShardType,
}

impl fmt::Display for Shard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:0width$}-({},{})",
            self.shard_number,
            self.val.x,
            self.val.y,
            width = self.shards_magnitude as usize
        )
    }
}

fn main() {
    let shards = split(1234, 3, 10).unwrap();

    for shard in &shards {
        println!("{}", shard);
    }

    // this rng is only for shits 'n giggles
    let mut rng = rand::thread_rng();

    let chosen_shards: Vec<_> = shards.choose_multiple(&mut rng, 3).collect();

    let secret = combine(&chosen_shards, 3).unwrap();

    println!("Secret: {}", secret);
}

fn split(secret: SecretType, threshold: u8, tokens: u8) -> Result<Vec<Shard>, &'static str> {
    let gf = Uniform::new(1, 256);
    let mut rng = rand::thread_rng();

    let fmt_len = (tokens as f32).log(10.0).floor() as u8 + 1;

    let poly: Vec<SecretType> = (&gf)
        .sample_iter(&mut rng)
        .take(threshold as usize - 1)
        // .map(|l| Box::new(l))
        .collect();

    let mut shards: Vec<Shard> = Vec::with_capacity(tokens as usize);

    for i in 0..tokens {
        let x = (&gf).sample(&mut rng);
        let mut y = 0;

        for (i, val) in poly.iter().enumerate() {
            y += val * x.pow(i as u32 + 1);
        }

        shards.push(Shard {
            shard_number: i + 1,
            shards_magnitude: fmt_len,
            val: Point {
                x,
                y: y + (&secret),
            },
        });
    }

    return Ok(shards);
}

fn combine(shards: &Vec<&Shard>, threshold: usize) -> Result<SecretType, &'static str> {
    if shards.len() < threshold as usize {
        return Err("Not enough shards provided");
    }

    let mut ans: GenericFraction<SecretType> = GenericFraction::new(0, 1);

    for i in 0..threshold {
        let mut l = GenericFraction::new(shards[i].val.y, 1);

        for j in 0..threshold {
            if i == j {
                continue;
            }

            l *= GenericFraction::new(-shards[j].val.x, shards[i].val.x - shards[j].val.x);
        }

        ans += l;
    }

    return Ok(*ans.numer().unwrap());
}
