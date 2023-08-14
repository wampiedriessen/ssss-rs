mod encoding;
mod err;
mod math;
mod shard;

use crate::math::{ShamirData, UnsafeInteger};
pub use shard::SsssShard;

// TODO: Give a different implementation a chance
type ShamirImpl = UnsafeInteger;

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
pub fn encode(options: ShamirScheme, secret: &[u8]) -> Vec<SsssShard> {
    let data_len = secret.len();
    let num_chunks = secret.len() / ShamirImpl::CHUNK_BYTE_COUNT + (if secret.len() % ShamirImpl::CHUNK_BYTE_COUNT == 0 { 0 } else { 1 });
    println!("Chunks: {num_chunks}");
    println!("Data len: {data_len}");
    (0..num_chunks)
        .into_iter()
        .map(|chunknum| {
            let rstart = chunknum * ShamirImpl::CHUNK_BYTE_COUNT;
            let rstop = (chunknum + 1 ) * ShamirImpl::CHUNK_BYTE_COUNT;
            let mut secret_data = [0u8; ShamirImpl::CHUNK_BYTE_COUNT];
            if secret.len() <= rstop {
                secret_data[..(secret.len()-rstart)].copy_from_slice(&secret[rstart..]);
            } else {
                secret_data.copy_from_slice(&secret[rstart..rstop])
            };

            encode_internal::<ShamirImpl>(&options, secret)
        }).reduce(|mut accumulator, newchunks| {
            for (x, mut data) in newchunks {
                accumulator[(x-1) as usize].1.append(&mut data);
            }
            accumulator
        }).unwrap()
        .into_iter().map(|(x, data)| {
            SsssShard::new(options.num_shards, x, data)
        }).collect()
}

#[must_use]
pub fn decode(shards: &[SsssShard]) -> Vec<u8> {
    let num_shards = shards.len();
    let num_chunks = (shards[0].data().len() / ShamirImpl::CHUNK_BYTE_COUNT) + 1;
    println!("Chunks: {num_chunks}");

    (0..num_chunks)
        .into_iter()
        .flat_map(|chunknum| {
            let short_shards: Vec<SsssShard> = shards
                .iter()
                .map(|s| {
                    let rstart = chunknum * ShamirImpl::CHUNK_BYTE_COUNT;
                    let rstop = (chunknum + 1 ) * ShamirImpl::CHUNK_BYTE_COUNT;
                    let data = s.data();
                    let data = if data.len() <= rstop { &data[rstart..] } else { &data[rstart..rstop] };

                    SsssShard::new(
                        num_shards as u8,
                        s.num(),
                        data.to_vec(),
                    )
                })
                .collect();
            decode_internal::<ShamirImpl>(short_shards.as_slice())
        }).collect()
}

#[must_use]
fn encode_internal<T: ShamirData>(options: &ShamirScheme, secret: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut rng = rand::thread_rng();

    let mut le_polynomial = Vec::with_capacity(options.threshold.into());
    le_polynomial.push(T::from_bytes(secret));
    for _ in 1..options.threshold {
        le_polynomial.push(T::get_random(&mut rng));
    }

    (1..options.num_shards+1)
        .map(|x| {
            let y = apply_x(x, &le_polynomial);
            (x, y.get_data())
        })
        .collect()
}

#[must_use]
fn decode_internal<T: ShamirData>(shards: &[SsssShard]) -> Vec<u8> {
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

    sum.get_data().to_vec()
}

fn apply_x<T: ShamirData>(x: u8, poly: &Vec<T>) -> T {
    let mut val = T::new();

    for (i, p) in poly.iter().enumerate() {
        val += T::new_int(x).pow(i as u32).mul(p);
    }

    val
}

#[cfg(test)]
mod test {
    use crate::math::{ShamirData, UnsafeInteger};
    use rand::RngCore;

    #[test]
    fn test_apply_x() {
        apply_x_polynomial::<UnsafeInteger>();
    }

    fn apply_x_polynomial<T: ShamirData>() {
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

    fn end_to_end<T: ShamirData>() {
        let mut rng = rand::thread_rng();
        let mut secret_bytes = vec![0; 128];
        rng.fill_bytes(&mut secret_bytes);

        let shards = super::encode(super::ShamirScheme::new(3, 8), &secret_bytes);

        assert_eq!(secret_bytes, super::decode(&shards[0..3]));
        assert_eq!(secret_bytes, super::decode(&shards[2..5]));
        assert_eq!(secret_bytes, super::decode(&shards[3..6]));
        assert_eq!(secret_bytes, super::decode(&shards[5..8]));
    }

    #[test]
    #[should_panic]
    fn test_all_unencrypted_data() {
        todo!();
        let t = 2;
        let n = 2;

        test_unencrypted_data::<UnsafeInteger>(super::ShamirScheme::new(t, n));
        // test_unencrypted_data::<Galois>(ShamirScheme::new(t, n));
    }

    fn test_unencrypted_data<T: ShamirData>(shamir: super::ShamirScheme) {
        let secret = "a".repeat(100);
        let secret_bytes = secret.as_bytes();

        let shards = super::encode(shamir, &secret_bytes);

        for s in &shards {
            println!("{s}");
        }

        assert_ne!(&secret_bytes[0..50], &shards[0].data()[0..50]);
        assert_ne!(&secret_bytes[0..50], &shards[1].data()[0..50]);
    }
}
