mod encoding;
use crate::encoding::{base64_decode, base64_encode};

use std::fmt;
use std::str;

#[derive(Debug)]
struct SsssShard {
    shard_number: u8,
    data: Vec<u8>,
}

impl fmt::Display for SsssShard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_formatted = base64_encode(self.data.as_slice());
        write!(f, "{}-{}", self.shard_number, data_formatted)
    }
}

impl str::FromStr for SsssShard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('-').collect();
        Ok(SsssShard {
            shard_number: split[0].parse().unwrap(),
            data: base64_decode(split[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMATTED_STRING: &str = "13-QUJDQQ==";
    fn example_shard() -> SsssShard {
        SsssShard {
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
    }
}
