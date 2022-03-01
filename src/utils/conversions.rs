use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub fn raw_to_hex(raw: &[u8]) -> String {
    let mut s = String::with_capacity(raw.len() * 3);
    for byte in raw {
        s.push_str(&format!("{:02X} ", byte));
    }
    s.pop();
    s
}

pub fn u32_to_hex(val: u32, is_little_endian: bool) -> String {
    let mut buf = [0; 4];

    match is_little_endian {
        true => LittleEndian::write_u32(&mut buf, val),
        false => BigEndian::write_u32(&mut buf, val),
    }

    raw_to_hex(&mut buf)
}

pub fn u64_to_hex(val: u64, is_little_endian: bool) -> String {
    let mut buf = [0; 8];

    match is_little_endian {
        true => LittleEndian::write_u64(&mut buf, val),
        false => BigEndian::write_u64(&mut buf, val),
    }

    raw_to_hex(&mut buf)
}

pub fn raw_to_str(raw: &[u8]) -> (usize, Result<&str, std::str::Utf8Error>) {
    let nul_range_end = raw.iter().position(|&c| c == b'\0').unwrap_or(raw.len()); // default to length if no `\0` present
    (nul_range_end, ::std::str::from_utf8(&raw[0..nul_range_end]))
}
