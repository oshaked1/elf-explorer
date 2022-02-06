pub fn raw_to_hex(raw: &[u8]) -> String {
    let mut s = String::with_capacity(raw.len()*3);
    for byte in raw {
        s.push_str(&format!("{:02X} ", byte));
    }
    s.pop();
    s
}