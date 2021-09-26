const SECURITY_LEVEL: usize = 128;

use crate::shard::SsssShard;
use crate::util::Fraction;
use num_bigint::{BigInt, BigUint, RandomBits, Sign};
use rand::Rng;

pub struct ShamirScheme {
    pub num_shards: u8,
    pub threshold: u8,
}

impl ShamirScheme {
    pub fn new(threshold: u8, num: u8) -> Self {
        ShamirScheme {
            threshold,
            num_shards: num,
        }
    }

    pub fn create_shards(&self, secret: &[u8]) -> Vec<SsssShard> {
        let mut rng = rand::thread_rng();
        let random_bits = RandomBits::new(SECURITY_LEVEL as u64);

        let mut le_polynomial = Vec::with_capacity(self.threshold.into());
        le_polynomial.push(BigUint::from_bytes_be(secret));
        for _ in 1..self.threshold {
            le_polynomial.push(rng.sample(random_bits));
        }

        (0..self.num_shards)
            .map(|x| {
                let y = apply_x(x, &le_polynomial);
                SsssShard::new(self.num_shards, x, y.to_bytes_be())
            })
            .collect()
    }

    pub fn merge_shards(&self, shards: &[SsssShard]) -> Vec<u8> {
        let mut sum = Fraction::new(0.into(), 1.into());
        for shard_i in shards {
            let i = shard_i.num() as i64;
            let mut accum =
                Fraction::new(BigInt::from_bytes_be(Sign::Plus, shard_i.data()), 1.into());
            for j in shards.iter().map(|s| s.num() as i64) {
                if i == j {
                    continue;
                }
                accum *= Fraction {
                    num: j.into(),
                    denum: (j - i).into(),
                };
            }
            sum += accum;
        }

        let normalized = sum.num / sum.denum;

        let (_, bytes) = normalized.to_bytes_be();

        bytes
    }
}

fn apply_x(x: u8, poly: &Vec<BigUint>) -> BigUint {
    let mut val = BigUint::from(0u64);

    for (i, p) in poly.iter().enumerate() {
        val += BigUint::from(x).pow(i as u32) * p;
    }

    val
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_x() {
        // 5 + x + 3x^2
        let poly: Vec<BigUint> = vec![5u8.into(), 1u8.into(), 3u8.into()];

        assert_eq!(35, apply_x(3, &poly).to_u64_digits()[0]);
        assert_eq!(57, apply_x(4, &poly).to_u64_digits()[0]);
        assert_eq!(85, apply_x(5, &poly).to_u64_digits()[0]);
    }

    #[test]
    fn test_end_to_end() {
        let shamir = ShamirScheme::new(3, 8);

        let mut rng = rand::thread_rng();
        let random_bits = RandomBits::new(SECURITY_LEVEL.pow(2) as u64);

        let secret: BigUint = rng.sample(random_bits);
        let bytes: Vec<u8> = secret.to_bytes_be();

        let shards = shamir.create_shards(&bytes);

        assert_eq!(bytes, shamir.merge_shards(&shards[0..3]));
        assert_eq!(bytes, shamir.merge_shards(&shards[2..5]));
        assert_eq!(bytes, shamir.merge_shards(&shards[3..6]));
        assert_eq!(bytes, shamir.merge_shards(&shards[5..8]));
    }
}
