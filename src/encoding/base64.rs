const PAD_CHAR: char = '=';

use super::util;

/// Translates a byte-array to its corresponding base64 encoding
#[must_use]
pub fn base64_encode(x: &[u8]) -> crate::err::Result<String> {
    let mut output = String::with_capacity((x.len() as f64 * 1.4).ceil() as usize );

    let chunks = x.chunks(3);

    for chunk in chunks {
        let chunk_bytes = util::encode_triplet_chunk(chunk, |b| u6_to_b64_char(b), PAD_CHAR as u8)?;

        for byte in chunk_bytes {
            output.push(byte as char);
        }
    }

    Ok(output)
}

/// Translates a base64 encoded string to its corresponding byte-array
#[must_use]
pub fn base64_decode(x: &str) -> crate::err::Result<Vec<u8>> {
    if x.len() == 0 {
        return Ok(vec![]);
    }
    if x.len() % 4 != 0 {
        return Err(crate::err::SsssErr);
    }

    let bytes = x.as_bytes();

    let output: Vec<u8> = bytes
        .chunks_exact(4)
        .flat_map(|chunk| util::decode_quartet_chunk(chunk, |b| b64_char_to_u6(b), PAD_CHAR as u8))
        .flatten()
        .collect();

    Ok(output)
}

fn u6_to_b64_char(x: &u8) -> crate::err::Result<u8> {
    match x {
        0..=25 => Ok('A' as u8 + x),
        26..=51 => Ok('a' as u8 + (x - 26)),
        52..=61 => Ok('0' as u8 + (x - 52)),
        62 => Ok('+' as u8),
        63 => Ok('/' as u8),
        _ => Err(crate::err::SsssErr),
    }
}

fn b64_char_to_u6(x: &u8) -> crate::err::Result<u8> {
    match *x as char {
        'A'..='Z' => Ok(x - ('A' as u8)),
        'a'..='z' => Ok((x + 26) - ('a' as u8)),
        '0'..='9' => Ok((x + 52) - ('0' as u8)),
        '+' => Ok(62),
        '/' => Ok(63),
        PAD_CHAR => Ok(0),
        _ => Err(crate::err::SsssErr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn empty_is_empty() -> crate::err::Result<()> {
        let encoded_output = base64_encode("".as_bytes())?;
        assert_eq!("", encoded_output);

        let decoded_output = base64_decode("").unwrap();
        assert_eq!("".as_bytes(), decoded_output);

        Ok(())
    }

    #[test_case("A", "QQ==")]
    #[test_case("AB", "QUI=")]
    #[test_case("ABC", "QUJD")]
    fn test_base64_encode(decoded: &str, encoded: &str) -> crate::err::Result<()>  {
        let encoded_output = base64_encode(decoded.as_bytes())?;
        assert_eq!(encoded, encoded_output);

        let decoded_output = base64_decode(encoded).unwrap();
        assert_eq!(decoded.as_bytes(), decoded_output);
        
        Ok(())
    }
}
