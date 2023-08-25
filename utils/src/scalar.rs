pub fn remove_0x(key: &String) -> String {
    key.trim_start_matches("0x").to_string()
}

pub fn prepend_zeros(s: &String, n: usize) -> String {
    assert_eq!(n <= 64, true);
    let sz = s.len();
    assert_eq!(sz <= n && sz <= 64, true);
    format!("{:0>n$}", s)
}

pub fn normalize_to_n_format(s: &String, n: usize) -> String {
    format!("{}", prepend_zeros(&remove_0x(s), n))
}

pub fn normalize_to_0xn_format(s: &String, n: usize) -> String {
    format!("0x{}", prepend_zeros(&remove_0x(s), n))
}

pub fn byte2char(b: u8) -> char {
    std::char::from_u32(b as u32).unwrap()
}

pub fn byte2string(b: u8) -> String {
    let mut result = String::from("");
    result.push(byte2char(b >> 4));
    result.push(byte2char(b & 0x0F));
    result
}

/// Byte to/from char conversion
pub fn char2byte(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - '0' as u8,
        'A'..='F' => c as u8 - 'A' as u8 + 10,
        'a'..='f' => c as u8 - 'a' as u8 + 10,
        _ => panic!("Invalud conversion, non-hex char: {}", c),
    }
}

/// Strint to/from byte array conversion
/// s must be even sized, and must not include the leading "0x"
/// pData buffer must be big enough to store converted data
pub fn string2ba(os: &String) -> Vec<u8> {
    let mut s = remove_0x(os);

    if s.len() % 2 != 0 {
        // 0 + s
        s = prepend_zeros(&s, s.len() + 1);
    }
    let dsize = s.len() * 2;
    let chars = &mut s.chars();
    let mut result: Vec<u8> = Vec::new();
    for _i in 0..dsize {
        result.push(char2byte(chars.next().unwrap()));
        result.push(char2byte(chars.next().unwrap()));
    }
    result
}

#[cfg(test)]
mod test {
    use crate::scalar::prepend_zeros;
    #[test]
    fn test_prepend_zeros() {
        assert_eq!(
            prepend_zeros(&"abc".to_string(), 10),
            "0000000abc".to_string()
        );
    }
}
