use crate::encoding::{base64_decode, base64_encode};

use std::fmt;
use std::str;

#[derive(Debug)]
pub struct SsssShard {
    shard_poolsize: Option<u8>,
    shard_number: u8,
    data: Vec<u8>,
}

impl SsssShard {
    pub (crate) fn new(total_shards: u8, n: u8, data: Vec<u8>) -> Self {
        SsssShard {
            shard_poolsize: Some(total_shards),
            shard_number: n,
            data: data.to_vec(),
        }
    }

    pub (crate) fn data(&self) -> &[u8] { self.data.as_slice() }
    pub (crate) fn num(&self) -> u8 { self.shard_number }
}

impl fmt::Display for SsssShard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_formatted = base64_encode(self.data.as_slice()).unwrap();

        let width = match self.shard_poolsize {
            Some(x) => (x as f64).log10().ceil(),
            None => (self.shard_number as f64).log10().ceil(),
        } as usize;

        write!(
            f,
            "{:0width$}-{}",
            self.shard_number,
            data_formatted,
            width = width
        )
    }
}

const PARSE_ERR: &'static str = "Cannot parse Shard";

impl str::FromStr for SsssShard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('-').collect();

        if split.len() != 2 { return Err(PARSE_ERR.into()); }

        Ok(SsssShard {
            shard_poolsize: None,
            shard_number: split[0].parse().map_err::<String, _>(|_| PARSE_ERR.into())?,
            data: base64_decode(split[1]).map_err(|x| x.to_string())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMATTED_STRING: &str = "013-QUJDQQ==";
    fn example_shard() -> SsssShard {
        SsssShard {
            shard_poolsize: Some(222),
            shard_number: 13,
            data: vec![65, 66, 67, 65],
        }
    }

    #[test]
    fn formatting() {
        let formatted = format!("{}", example_shard());

        assert_eq!(FORMATTED_STRING, formatted)
    }

    #[test]
    fn parsing() {
        let shard: SsssShard = FORMATTED_STRING.parse().unwrap();

        let s = example_shard();
        assert_eq!(s.shard_number, shard.shard_number);
        assert_eq!(s.data, shard.data);

        // No need to ascertain shard_poolsize magnitude during parse
        assert_eq!(None, shard.shard_poolsize);
    }
}
