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

pub fn fea42scalar(fea: &[Fr; 4]) -> BigUint {
    let mut f1: BigUint = BigUint::from(fea[3].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[2].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[1].as_int());
    f1 <<= 64;
    f1 |= BigUint::from(fea[0].as_int());
    f1
}

pub fn fea82scalar(fea: &[Fr; 8]) -> Option<BigUint> {
    // Add field element 7
    let aux_h = fea[7].as_int();
    if aux_h >= 0x100000000 {
        warn!(
            "fea2scalar() found element 7 has a too high value={}",
            fea[7]
        );
        return None;
    }

    // Add field element 6
    let aux_l = fea[6].as_int();
    if aux_l >= 0x100000000 {
        warn!(
            "fea2scalar() found element 6 has a too high value={}",
            fea[6]
        );
        return None;
    }

    let mut scalar: BigUint = (BigUint::from(aux_h) << 32) + BigUint::from(aux_l);
    scalar <<= 64;

    // Add field element 5
    let aux_h = fea[5].as_int();
    if aux_h >= 0x100000000 {
        warn!(
            "fea2scalar() found element 5 has a too high value={}",
            fea[5]
        );
        return None;
    }

    // Add field element 4
    let aux_l = fea[4].as_int();
    if aux_l >= 0x100000000 {
        warn!(
            "fea2scalar() found element 4 has a too high value={}",
            fea[4]
        );
        return None;
    }

    scalar += (BigUint::from(aux_h) << 32) + BigUint::from(aux_l);
    scalar <<= 64;

    // Add field element 3
    let aux_h = fea[3].as_int();
    if aux_h >= 0x100000000 {
        warn!(
            "fea2scalar() found element 3 has a too high value={}",
            fea[3]
        );
        return None;
    }

    // Add field element 2
    let aux_l = fea[2].as_int();
    if aux_l >= 0x100000000 {
        warn!(
            "fea2scalar() found element 2 has a too high value={}",
            fea[2]
        );
        return None;
    }

    scalar += (BigUint::from(aux_h) << 32) + BigUint::from(aux_l);
    scalar <<= 64;

    // Add field element 1
    let aux_h = fea[1].as_int();
    if aux_h >= 0x100000000 {
        warn!(
            "fea2scalar() found element 1 has a too high value={}",
            fea[1]
        );
        return None;
    }

    // Add field element 0
    let aux_l = fea[0].as_int();
    if aux_l >= 0x100000000 {
        warn!(
            "fea2scalar() found element 0 has a too high value={}",
            fea[0]
        );
        return None;
    }

    scalar += (BigUint::from(aux_h) << 32) + BigUint::from(aux_l);
    Some(scalar)
}

pub fn scalar2fe(scalar: u64) -> Fr {
    Fr::from(scalar)
}

#[inline(always)]
pub fn scalar2fea(scalar: &BigUint) -> [u64; 8] {
    let mut fea = [0u64; 8];
    let mask = BigUint::from(0xFFFFFFFFu64);
    // let scalar = BigUint::from_str(s).unwrap();
    let mut aux: BigUint = scalar.clone() & mask.clone();
    fea[0] = aux.to_u64().unwrap();
    aux = (scalar.clone() >> 32) & mask.clone();
    fea[1] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 64 & mask.clone();
    fea[2] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 96 & mask.clone();
    fea[3] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 128 & mask.clone();
    fea[4] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 160 & mask.clone();
    fea[5] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 192 & mask.clone();
    fea[6] = aux.to_u64().unwrap();
    aux = scalar.clone() >> 224 & mask.clone();
    fea[7] = aux.to_u64().unwrap();
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
    let os = remove_0x(os);
    let mut fea = vec![];
    debug!("string2fe: {}", os.len());
    for i in (0..os.len()).step_by(16) {
        if i + 16 > os.len() {
            panic!("string2fea: invalid input: {}", os);
        }
        let fe = os.get(i..(i + 16)).unwrap();
        debug!("string2fea fe: {}", fe);
        let cr = string2fe(&fe.to_string());
        fea.push(cr);
    }
    fea.reverse();
    fea
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
    use crate::scalar::prepend_zeros;
    use crate::scalar::{fea2string, string2fea};
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
}
