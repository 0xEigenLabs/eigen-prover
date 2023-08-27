use plonky::field_gl::Fr;
use num_bigint::BigUint;
use std::str::FromStr;

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

pub fn fea2scalar(fea: &[Fr; 4]) -> BigUint {
    let mut f1: BigUint = BigUint::from(fea[3].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[2].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[1].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[0].as_int());
    f1
}

pub fn fea2string(fea: &[Fr; 4]) -> String {
    let f1 = fea2scalar(fea);
    f1.to_str_radix(16)
}

pub fn scalar2fe(scalar: u64) -> Fr {
    Fr::from(scalar)
}

pub fn scalar2fea(s: &String) -> [u64; 8] {
    let mut fea = [0u64; 8];
    let mask = BigUint::from(0xFFFFFFFF);
    let scalar = BigUint::from_str(s).unwrap();
    let aux: BigUint = scalar & mask;
    fea[0] = aux.to_u64();
    let aux = scalar>>32 & mask;
    fea[1] = aux.to_u64();
    let aux = scalar>>64 & mask;
    fea[2] = aux.to_u64();
    let aux = scalar>>96 & mask;
    fea[3] = aux.to_u64();
    let aux = scalar>>128 & mask;
    fea[4] = aux.to_u64();
    let aux = scalar>>160 & mask;
    fea[5] = aux.to_u64();
    let aux = scalar>>192 & mask;
    fea[6] = aux.to_u64();
    let aux = scalar>>224 & mask;
    fea[7] = aux.to_u64();
    fea
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

pub fn string2fea(os: &String) -> Vec<Fr> {
    let mut r = [Fr::ZERO; 4];
    let mut fea = vec![];
    for i in (0..os.len()).step_by(16) {
        if i + 16 > os.len() {
            panic!("string2fea: invalid input: {}", os);
        }
        let fe = os.get(i .. (i+16)).unwrap();
        let cr = string2fe(&fe.to_string());
        fea.push(cr);
    }
    fea
}

pub fn string2fe(os: &String) -> Fr {
    Fr::from_str(os).unwrap()
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
