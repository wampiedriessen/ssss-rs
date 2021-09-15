const PAD_CHAR: char = '=';

/// Translates a byte-array to its corresponding base64 encoding
pub fn base64_encode(x: &[u8]) -> String {
    let mut output = String::with_capacity(x.len() * 2); // TODO: times 1.4 would be more accurate

    let chunks = x.chunks(3);

    for chunk in chunks {
        let chunk_bytes = encode_triplet_chunk(chunk, |b| u6_to_b64_char(b));
        output += String::from_utf8(chunk_bytes).unwrap().as_str();
    }

    output
}

/// Translates a base64 encoded string to its corresponding byte-array
pub fn base64_decode(x: &str) -> Vec<u8> {
    if x.len() == 0 {
        return vec![];
    }
    debug_assert!(x.len() % 4 == 0);

    let bytes = x.as_bytes();

    let output: Vec<u8> = bytes
        .chunks_exact(4)
        .flat_map(|chunk| decode_quartet_chunk(chunk, |b| b64_char_to_u6(b)))
        .collect();

    output
}

fn encode_triplet_chunk<F>(chunk: &[u8], u6_mapper: F) -> Vec<u8>
where
    F: Fn(&u8) -> u8,
{
    let chunk_len = chunk.len();

    let u6bytes = match chunk_len {
        3 => bytes_to_u6(chunk),
        2 => bytes_to_u6(&[chunk[0], chunk[1], 0]),
        1 => bytes_to_u6(&[chunk[0], 0, 0]),
        _ => panic!("Impossible chunk length {}", chunk_len),
    };

    let mut chars: Vec<u8> = u6bytes.iter().map(u6_mapper).collect();

    if chunk_len < 3 {
        chars[3] = PAD_CHAR as u8;
    }
    if chunk_len < 2 {
        chars[2] = PAD_CHAR as u8;
    }

    chars
}

fn decode_quartet_chunk<F>(chars: &[u8], u6_demapper: F) -> Vec<u8>
where
    F: Fn(&u8) -> u8,
{
    debug_assert!(chars.len() == 4);

    let mut cutoff = 0;
    if chars[2] == PAD_CHAR as u8 {
        cutoff = 2;
    } else if chars[3] == PAD_CHAR as u8 {
        cutoff = 1;
    }

    let u6bytes: Vec<u8> = chars.iter().map(u6_demapper).collect();

    let mut bytes = u6_to_bytes(u6bytes.as_slice());

    for _ in 0..cutoff {
        bytes.pop();
    }

    bytes
}

fn bytes_to_u6(x: &[u8]) -> Vec<u8> {
    debug_assert!(x.len() == 3);

    let m = ((x[0] as u32) << 16) | ((x[1] as u32) << 8) | (x[2] as u32);

    let a = ((m & 0xFC0000) >> 18) as u8;
    let b = ((m & 0x03F000) >> 12) as u8;
    let c = ((m & 0x000FC0) >> 6) as u8;
    let d = ((m & 0x00003F) >> 0) as u8;

    vec![a, b, c, d]
}

fn u6_to_bytes(x: &[u8]) -> Vec<u8> {
    debug_assert!(x.len() == 4);

    let m = ((x[0] as u32) << 18) | ((x[1] as u32) << 12) | ((x[2] as u32) << 6) | (x[3] as u32);

    let a = ((m & 0xFF0000) >> 16) as u8;
    let b = ((m & 0x00FF00) >> 8) as u8;
    let c = (m & 0x0000FF) as u8;

    vec![a, b, c]
}

fn u6_to_b64_char(x: &u8) -> u8 {
    match x {
        0..=25 => 'A' as u8 + x,
        26..=51 => 'a' as u8 + (x - 26),
        52..=61 => '0' as u8 + (x - 52),
        62 => '+' as u8,
        63 => '/' as u8,
        _ => panic!("TODO: do not panic, bring your towel"),
    }
}

fn b64_char_to_u6(x: &u8) -> u8 {
    match *x as char {
        'A'..='Z' => x - ('A' as u8),
        'a'..='z' => (x + 26) - ('a' as u8),
        '0'..='9' => (x + 52) - ('0' as u8),
        '+' => 62,
        '/' => 63,
        PAD_CHAR => 0,
        _ => panic!("TODO: do not panic, bring your towel"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("A", "QQ==")]
    #[test_case("AB", "QUI=")]
    #[test_case("ABC", "QUJD")]
    fn test_base64_encode(decoded: &str, encoded: &str) {
        let encoded_output = base64_encode(decoded.as_bytes());

        assert_eq!(encoded, encoded_output);

        let decoded_output = base64_decode(encoded);

        assert_eq!(decoded.as_bytes(), decoded_output);
    }
}
