const PAD_CHAR: char = '=';

/// Translates a byte-array to its corresponding base64 encoding
pub fn base64_encode(x: &[u8]) -> crate::err::Result<String> {
    let mut output = String::with_capacity((x.len() as f64 * 1.4).ceil() as usize );

    let chunks = x.chunks_exact(3);

    let rem = chunks.remainder();

    for chunk in chunks {
        let a = chunk[0] >> 2;
        let b = ((chunk[0] & 0x3) << 4) | ((chunk[1] & 0xF0) >> 4);
        let c = ((chunk[1] & 0x0F) << 2) | ((chunk[2] & 0xC0) >> 6);
        let d = chunk[2] & 0x3F;

        output.push(ENC_LOOKUP_TABLE[a as usize]);
        output.push(ENC_LOOKUP_TABLE[b as usize]);
        output.push(ENC_LOOKUP_TABLE[c as usize]);
        output.push(ENC_LOOKUP_TABLE[d as usize]);
    }

    if rem.len() == 2 {
        output.push(ENC_LOOKUP_TABLE[ (rem[0] >> 2) as usize]);
        output.push(ENC_LOOKUP_TABLE[ (((rem[0] & 0x3) << 4) | ((rem[1] & 0xF0) >> 4)) as usize ]);
        output.push(ENC_LOOKUP_TABLE[ ((rem[1] & 0x0F) << 2) as usize]);
        output.push(PAD_CHAR);
    }
    if rem.len() == 1 {
        output.push(ENC_LOOKUP_TABLE[ (rem[0] >> 2) as usize]);
        output.push(ENC_LOOKUP_TABLE[ ((rem[0] & 0x3) << 4) as usize ]);
        output.push(PAD_CHAR);
        output.push(PAD_CHAR);
    }

    Ok(output)
}

/// Translates a base64 encoded string to its corresponding byte-array
pub fn base64_decode(x: &str) -> crate::err::Result<Vec<u8>> {
    if x.len() == 0 {
        return Ok(vec![]);
    }
    if x.len() % 4 != 0 {
        return Err(crate::err::SsssErr);
    }
    let mut output = Vec::with_capacity((x.len() as f64 * 0.8).ceil() as usize );

    let chunks = x.trim_end_matches('=').as_bytes().chunks_exact(4);

    let rem = chunks.remainder();

    for chunk in chunks {
        let a: u8 = DEC_LOOKUP_TABLE[chunk[0] as usize];
        let b: u8 = DEC_LOOKUP_TABLE[chunk[1] as usize];
        let c: u8 = DEC_LOOKUP_TABLE[chunk[2] as usize];
        let d: u8 = DEC_LOOKUP_TABLE[chunk[3] as usize];

        output.push((a << 2) | ((b & 0xF0) >> 4));
        output.push(((b & 0x0F) << 4) | ((c & 0x3C) >> 2) );
        output.push(((c & 0x03) << 6) | (d & 0x3F) );
    }

    // 3 padding characters cannot happen:
    // if rem.len() == 1 { ... }
    if rem.len() == 2 {
        let a: u8 = DEC_LOOKUP_TABLE[rem[0] as usize];
        let b: u8 = DEC_LOOKUP_TABLE[rem[1] as usize];

        output.push((a << 2) | ((b & 0xF0) >> 4));
    }
    if rem.len() == 3 {
        let a: u8 = DEC_LOOKUP_TABLE[rem[0] as usize];
        let b: u8 = DEC_LOOKUP_TABLE[rem[1] as usize];
        let c: u8 = DEC_LOOKUP_TABLE[rem[2] as usize];

        output.push((a << 2) | ((b & 0xF0) >> 4));
        output.push(((b & 0x0F) << 4) | ((c & 0x3C) >> 2) );
    }

    Ok(output)
}

const ENC_LOOKUP_TABLE: [char; 64] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9','+','/'
];
const DEC_LOOKUP_TABLE: [u8; 128] = [
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, /* 0 - 15 */
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, /* 16 - 31 */
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 62, 80, 80, 80, 63, /* 32 - 47 */
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 80, 80, 80, 64, 80, 80, /* 48 - 63 */
    80,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, /* 64 - 79 */
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 80, 80, 80, 80, 80, /* 80 - 96 */
    80, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, /* 87 - 111 */
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 80, 80, 80, 80, 80 /* 112 - 127 */
];

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
