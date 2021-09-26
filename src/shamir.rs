const SECURITY_LEVEL: usize = 128;

use std::marker::PhantomData;

use crate::{math::ShamirInteger, shard::SsssShard};

pub struct ShamirScheme<T: ShamirInteger> {
    pub num_shards: u8,
    pub threshold: u8,
    _phantom: PhantomData<T>,
}

impl<T: ShamirInteger> ShamirScheme<T> {
    pub fn new(threshold: u8, num: u8) -> Self {
        ShamirScheme {
            threshold,
            num_shards: num,
            _phantom: PhantomData,
        }
    }

    pub fn create_shards(&self, secret: &[u8]) -> Vec<SsssShard> {
        let mut rng = rand::thread_rng();
        let num_bits = SECURITY_LEVEL as u64;

        let mut le_polynomial = Vec::with_capacity(self.threshold.into());
        le_polynomial.push(T::from_bytes(secret));
        for _ in 1..self.threshold {
            le_polynomial.push(T::get_random(&mut rng, num_bits));
        }

        (0..self.num_shards)
            .map(|x| {
                let y = self.apply_x(x, &le_polynomial);
                SsssShard::new(self.num_shards, x, y.get_data())
            })
            .collect()
    }

    pub fn merge_shards(&self, shards: &[SsssShard]) -> Vec<u8> {
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

    fn apply_x(&self, x: u8, poly: &Vec<T>) -> T {
        let mut val = T::new();

        for (i, p) in poly.iter().enumerate() {
            val += T::new_int(x).pow(i as u32).mul(p);
        }

        val
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::UnsafeInteger;

    #[test]
    fn test_apply_x() {
        test_apply_x_polynomial::<UnsafeInteger>();
    }

    fn test_apply_x_polynomial<T: ShamirInteger>() {
        // 5 + x + 3x^2
        let poly: Vec<T> = vec![5u8, 1u8, 3u8].iter().map(|b| T::new_int(*b)).collect();
        let scheme = ShamirScheme::<T>::new(0, 0);

        let apply = |x| scheme.apply_x(x, &poly).get_data()[0];

        assert_eq!(35, apply(3));
        assert_eq!(57, apply(4));
        assert_eq!(85, apply(5));
    }

    #[test]
    fn test_end_to_end() {
        let implementations = vec![ShamirScheme::<UnsafeInteger>::new(3, 8)];

        for shamir in implementations {
            let mut rng = rand::thread_rng();
            let secret = UnsafeInteger::get_random(&mut rng, SECURITY_LEVEL.pow(2) as u64);

            let bytes: Vec<u8> = secret.get_data();

            let shards = shamir.create_shards(&bytes);

            assert_eq!(bytes, shamir.merge_shards(&shards[0..3]));
            assert_eq!(bytes, shamir.merge_shards(&shards[2..5]));
            assert_eq!(bytes, shamir.merge_shards(&shards[3..6]));
            assert_eq!(bytes, shamir.merge_shards(&shards[5..8]));
        }
    }
}
