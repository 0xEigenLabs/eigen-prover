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

#[cfg(test)]
mod test {
    use crate::scalar::prepend_zeros;
    #[test]
    fn test_prepend_zeros() {
        assert_eq!(prepend_zeros(&"abc".to_string(), 10), "0000000abc".to_string());
    }
}
