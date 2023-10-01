use log::{debug, warn};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use plonky::ff::from_hex;
use plonky::field_gl::Fr;

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


/// Convert array of 4 Scalars of 64 bits into a unique 256 bits scalar
///
/// # Arguments
/// * fea: [Fr; 4] - Array of 4 Scalars of 64 bits
///
/// # Returns
/// * {Scalar} 256 bit number representation
///
pub fn fea42scalar(fea: &[Fr; 4]) -> BigUint {
    let biga = fea.iter().map(|e|  BigUint::from(e.as_int())).collect::<Vec<BigUint>>();
    let mut scalar = BigUint::from(0u32);

    for (k, shift) in biga.iter().zip(vec![0u32, 64, 128, 192]) {
        scalar = scalar + (k << shift)
    }
    scalar
}

/// Field element array to Scalar
///
/// result = arr[0] + arr[1]*(2^32) + arr[2]*(2^64) + arr[3]*(2^96) + arr[3]*(2^128) + arr[3]*(2^160) + arr[3]*(2^192) + arr[3]*(2^224)
pub fn fea82scalar(fea: &[Fr; 8]) -> BigUint {
    let biga = fea.iter().map(|e|  BigUint::from(e.as_int())).collect::<Vec<BigUint>>();
    let mut scalar = BigUint::from(0u32);

    for (k, shift) in biga.iter().zip(vec![0u32, 32, 64, 96, 128, 160, 192, 224]) {
        scalar = scalar + (k << shift)
    }

    scalar
}

pub fn scalar2fe(scalar: u64) -> Fr {
    Fr::from(scalar)
}

#[inline(always)]
pub fn scalar2fea(scalar: &BigUint) -> [Fr; 4] {
    let mut fea = [Fr::ZERO; 4];
    let mask = BigUint::from(0xFFFFFFFFFFFFFFFFu64);
    for (k, shift) in fea.iter_mut().zip(vec![0u32, 64, 128, 192]) {
        let aux = (scalar >> shift) & mask.clone();
        *k = Fr::from(aux.to_u64().unwrap());
    }
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

/* Hexa string to/from field element (array) conversion */
pub fn string2fea(os: &String) -> Vec<Fr> {
    assert_eq!(os.len(), 66);

    let mut res = vec![0u64; 4];
    let mut j = 3;
    for i in vec![0usize, 16, 32, 48] {
        res[j] = u64::from_str_radix(&os[(2+j*16)..(2+(j+1)*16)], 16).unwrap();
        j -= 1;
    }
    res.reverse();
    res.iter().map(|e| { Fr::from(*e) }).collect::<Vec<Fr>>()
}

// `0x${Scalar.toString(sc, 16).padStart(64, '0')}`;
pub fn fea2string(fea: &[Fr; 4]) -> String {
    let f1 = fea42scalar(fea);
    format!("0x{:0>64}", f1.to_str_radix(16))
}

pub fn string2fe(os: &String) -> Fr {
    let os = remove_0x(os);
    from_hex(&os).unwrap()
}

#[cfg(test)]
mod test {
    use crate::scalar::*;
    use plonky::field_gl::Fr;
    #[test]
    fn test_prepend_zeros() {
        assert_eq!(
            prepend_zeros(&"abc".to_string(), 10),
            "0000000abc".to_string()
        );
    }

    #[test]
    fn test_fea2string() {
        let a = [
            Fr::from(32),
            Fr::from(3),
            Fr::from(2),
            Fr::from(1),
        ];
        let out = fea2string(&a);
        let aa = string2fea(&out);
        assert_eq!(a[0..4], aa);
    }

    #[test]
    fn test_fea2scalar() {
        let a = [
            Fr::from(32),
            Fr::from(3),
            Fr::from(2),
            Fr::from(1),
        ];
        let out = fea42scalar(&a);
        let aa = scalar2fea(&out);
        assert_eq!(a, aa);
    }
}
