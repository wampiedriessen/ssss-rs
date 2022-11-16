pub fn encode_triplet_chunk<F>(chunk: &[u8], u6_mapper: F, pad_char: u8) -> crate::err::Result<Vec<u8>>
where
    F: Fn(&u8) -> crate::err::Result<u8>,
{
    let chunk_len = chunk.len();

    let u6bytes = match chunk_len {
        3 => bytes_to_u6(chunk),
        2 => bytes_to_u6(&[chunk[0], chunk[1], 0]),
        1 => bytes_to_u6(&[chunk[0], 0, 0]),
        _ => panic!("Impossible chunk length {}", chunk_len),
    };

    let mut chars: Vec<u8> = u6bytes.iter().map(u6_mapper).collect::<Result<_,_>>()?;

    if chunk_len < 3 {
        chars[3] = pad_char;
    }
    if chunk_len < 2 {
        chars[2] = pad_char;
    }

    Ok(chars)
}

pub fn decode_quartet_chunk<F>(chars: &[u8], u6_demapper: F, pad_char: u8) -> crate::err::Result<Vec<u8>>
where
    F: Fn(&u8) -> crate::err::Result<u8>,
{
    assert_eq!(chars.len(), 4);

    let mut cutoff = 0;
    if chars[2] == pad_char {
        cutoff = 2;
    } else if chars[3] == pad_char {
        cutoff = 1;
    }

    let u6bytes: Vec<u8> = chars.iter().map(u6_demapper).collect::<Result<_,_>>()?;

    let mut bytes = u6_to_bytes(u6bytes.as_slice());

    for _ in 0..cutoff {
        bytes.pop();
    }

    Ok(bytes)
}

fn bytes_to_u6(x: &[u8]) -> Vec<u8> {
    assert_eq!(x.len(), 3);

    let m = ((x[0] as u32) << 16) | ((x[1] as u32) << 8) | (x[2] as u32);

    let a = ((m & 0xFC0000) >> 18) as u8;
    let b = ((m & 0x03F000) >> 12) as u8;
    let c = ((m & 0x000FC0) >> 6) as u8;
    let d = ((m & 0x00003F) >> 0) as u8;

    vec![a, b, c, d]
}

fn u6_to_bytes(x: &[u8]) -> Vec<u8> {
    assert_eq!(x.len(), 4);

    let m = ((x[0] as u32) << 18) | ((x[1] as u32) << 12) | ((x[2] as u32) << 6) | (x[3] as u32);

    let a = ((m & 0xFF0000) >> 16) as u8;
    let b = ((m & 0x00FF00) >> 8) as u8;
    let c = (m & 0x0000FF) as u8;

    vec![a, b, c]
}
