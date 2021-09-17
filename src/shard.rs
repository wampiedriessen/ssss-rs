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
    pub fn new(total_shards: u8, n: u8, data: Vec<u8>) -> Self {
        SsssShard {
            shard_poolsize: Some(total_shards),
            shard_number: n,
            data,
        }
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl fmt::Display for SsssShard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_formatted = base64_encode(self.data.as_slice());

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

impl str::FromStr for SsssShard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('-').collect();
        Ok(SsssShard {
            shard_poolsize: None,
            shard_number: split[0].parse().unwrap(),
            data: base64_decode(split[1]),
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
