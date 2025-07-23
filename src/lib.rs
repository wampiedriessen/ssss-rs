// Allow dead code and unused imports when testing
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod encoding;
mod err;
mod math;
mod shard;

use rand::Rng;
use math::{GF, GfPoly};
pub use shard::SsssShard;

pub struct ShamirScheme {
    pub(crate) num_shards: u8,
    pub(crate) threshold: u8,
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
pub fn encode(options: &ShamirScheme, secret: &[u8]) -> Vec<SsssShard> {
    let data_len = secret.len();
    let rawchunks: Vec<Vec<(u8, u8)>> = (0..data_len)
        .into_iter()
        .map(|i| {
            encode_byte(options, secret[i])
        }).collect();

    (0..rawchunks[0].len())
        .map(|i| {
            let data = rawchunks.iter().map(|d| d[i].1).collect::<Vec<_>>();
            SsssShard::new(options.num_shards, rawchunks[0][i].0, data)
        }).collect()
}

#[must_use]
fn encode_byte(options: &ShamirScheme, secret: u8) -> Vec<(u8, u8)> {
    let mut rng = rand::thread_rng();
    let mut poly = vec![secret];
    for _ in 1..options.threshold {
        poly.push(rng.gen());
    }

    let poly = GfPoly::new(&poly);

    (1..=options.num_shards)
        .map(|x| {
            let y = poly.apply_x(x);
            (x, y.value() as u8)
        })
        .collect()
}

#[must_use]
pub fn decode(shards: &[SsssShard]) -> Vec<u8> {
    let num_bytes = shards[0].data().len();

    let mut data = Vec::new();
    let xvec: Vec<u8> = shards.iter().map(|s| s.num()).collect();
    let xslice = xvec.as_slice();

    for i in 0..num_bytes {
        let yvec = shards.iter().map(|s| s.data()[i]).collect::<Vec<_>>();

        data.push(decode_byte(xslice, yvec.as_slice()));
    }

    data
}

#[must_use]
fn decode_byte(x: &[u8], y: &[u8]) -> u8 {
    assert_eq!(x.len(), y.len());

    let k = x.len();
    let mut sum = GF::new(0);

    for j in 0..k {
        let mut mult = GF::new(y[j]);
        for m in 0..k {
            if j == m { continue; }
            mult *= GF::new(x[m]) / (GF::new(x[m]) - GF::new(x[j]));
        }

        sum += mult;
    }

    sum.value() as u8
}

#[cfg(test)]
mod test {
    use rand::RngCore;
    use crate::decode_byte;

    #[test]
    fn test_end_to_end() {
        let options = super::ShamirScheme::new(3, 8);
        let mut rng = rand::thread_rng();
        let mut secret_bytes = vec![0; 128];
        rng.fill_bytes(&mut secret_bytes);

        let shards = super::encode(&options, &secret_bytes);

        assert_eq!(secret_bytes, super::decode(&shards[0..3]));
        assert_eq!(secret_bytes, super::decode(&shards[2..5]));
        assert_eq!(secret_bytes, super::decode(&shards[3..6]));
        assert_eq!(secret_bytes, super::decode(&shards[5..8]));
    }

    #[test]
    fn test_threshold_of_one_is_plain_data() {
        let options = super::ShamirScheme::new(1, 8);
        let secret_bytes = [42, 32];

        let shards = super::encode(&options, &secret_bytes);

        for s in &shards {
            assert_eq!([42, 32], s.data());
        }
    }

    #[test]
    fn test_single_byte() {
        let secret_byte = 42u8;
        let options = super::ShamirScheme::new(2, 2);

        let encoded_bytes = super::encode_byte(&options, secret_byte);

        println!("{:?}", encoded_bytes);

        let decoded_poly = decode_byte(&[1, 2], &[encoded_bytes[0].1, encoded_bytes[1].1]);
        assert_eq!(42, decoded_poly);
    }

    #[test]
    fn test_all_unencrypted_data() {
        let options = super::ShamirScheme::new(2, 2);

        let secret = "a".repeat(100);
        let secret_bytes = secret.as_bytes();

        let shards = super::encode(&options, &secret_bytes);

        for s in &shards {
            println!("{s}");
        }

        assert_ne!(&secret_bytes[0..50], &shards[0].data()[0..50]);
        assert_ne!(&secret_bytes[0..50], &shards[1].data()[0..50]);
        assert_ne!(&shards[0].data()[0..50], &shards[1].data()[0..50]);

        assert_ne!(&secret_bytes[50..], &shards[0].data()[50..]);
        assert_ne!(&secret_bytes[50..], &shards[1].data()[50..]);
        assert_ne!(&shards[0].data()[50..], &shards[1].data()[50..]);
    }
}
