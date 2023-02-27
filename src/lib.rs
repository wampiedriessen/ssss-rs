mod err;
mod encoding;
mod math;
mod shard;

use crate::math::{ShamirInteger, UnsafeInteger};
pub use shard::SsssShard;

// TODO: Give a different implementation a chance
type ShamirImpl = UnsafeInteger;

pub struct ShamirScheme {
    pub (crate) num_shards: u8,
    pub (crate) threshold: u8,
}

impl ShamirScheme {
    #[must_use]
    pub fn new(threshold: u8, num_shards: u8) -> Self {
        ShamirScheme {
            threshold,
            num_shards,
        }
    }
}

#[must_use]
pub fn encode(options: ShamirScheme, secret: &[u8]) -> Vec<SsssShard> {
    encode_internal::<ShamirImpl>(options, secret)
}

#[must_use]
pub fn decode(shards: &[SsssShard]) -> Vec<u8> {
    decode_internal::<ShamirImpl>(shards)
}

#[must_use]
fn encode_internal<T: ShamirInteger>(options: ShamirScheme, secret: &[u8]) -> Vec<SsssShard> {
    let mut rng = rand::thread_rng();
    let num_bits = T::get_max_chunksize();

    let mut le_polynomial = Vec::with_capacity(options.threshold.into());
    le_polynomial.push(T::from_bytes(secret));
    for _ in 1..options.threshold {
        le_polynomial.push(T::get_random(&mut rng, num_bits));
    }

    (1..options.num_shards+1)
        .map(|x| {
            let y = apply_x(x, &le_polynomial);
            SsssShard::new(options.num_shards, x, y.get_data())
        })
        .collect()
}

#[must_use]
fn decode_internal<T: ShamirInteger>(shards: &[SsssShard]) -> Vec<u8> {
    let mut sum = T::new();
    for shard_i in shards {
        let i = shard_i.num() as i64;
        let mut accum = T::from_bytes(shard_i.data());

        for j in shards.iter().map(|s| s.num() as i64) {
            if i == j {
                continue;
            }
            accum *= T::new_fraction(j, j - i);
        }
        sum += accum;
    }

    sum.get_data()
}

fn apply_x<T: ShamirInteger>(x: u8, poly: &Vec<T>) -> T {
    let mut val = T::new();

    for (i, p) in poly.iter().enumerate() {
        val += T::new_int(x).pow(i as u32).mul(p);
    }

    val
}


#[cfg(test)]
mod test {
    use crate::math::{ShamirInteger, UnsafeInteger};

    #[test]
    fn test_apply_x() {
        apply_x_polynomial::<UnsafeInteger>();
    }

    fn apply_x_polynomial<T: ShamirInteger>() {
        // 5 + x + 3x^2
        let poly: Vec<T> = vec![5u8, 1u8, 3u8].iter().map(|b| T::new_int(*b)).collect();

        let apply = |x| super::apply_x(x, &poly).get_data()[0];

        assert_eq!(35, apply(3));
        assert_eq!(57, apply(4));
        assert_eq!(85, apply(5));
    }

    #[test]
    fn test_end_to_end() {
        end_to_end::<UnsafeInteger>();
    }

    fn end_to_end<T: ShamirInteger>() {
        let mut rng = rand::thread_rng();
        let secret = T::get_random(&mut rng, 128);

        let bytes: Vec<u8> = secret.get_data();
        let shards = super::encode( super::ShamirScheme::new(3, 8), &bytes);

        assert_eq!(bytes, super::decode(&shards[0..3]));
        assert_eq!(bytes, super::decode(&shards[2..5]));
        assert_eq!(bytes, super::decode(&shards[3..6]));
        assert_eq!(bytes, super::decode(&shards[5..8]));
    }
}
