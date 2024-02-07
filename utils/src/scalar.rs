use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::Num;
use plonky::ff::from_hex;
use plonky::field_gl::Fr;

pub fn remove_0x(key: &str) -> String {
    key.trim_start_matches("0x").to_string()
}

pub fn str_to_biguint(val: &str) -> BigUint {
    BigUint::from_str_radix(&remove_0x(val), 16).unwrap()
}

pub fn prepend_zeros(s: &str, n: usize) -> String {
    assert!(n <= 64);
    let sz = s.len();
    assert!(sz <= n && sz <= 64);
    format!("{:0>n$}", s)
}

pub fn normalize_to_n_format(s: &str, n: usize) -> String {
    prepend_zeros(&remove_0x(s), n)
}

pub fn normalize_to_0xn_format(s: &str, n: usize) -> String {
    format!("0x{}", prepend_zeros(&remove_0x(s), n))
}

/// Convert array of 4 Scalars of 64 bits into a unique 256 bits scalar
///
/// # Arguments
/// * fea: [Fr; 4] - Array of 4 Scalars of 64 bits
///
/// # Returns
/// * {Scalar} 256 bit number representation
///
pub fn h4_to_scalar(fea: &[Fr; 4]) -> BigUint {
    let biga = fea
        .iter()
        .map(|e| BigUint::from(e.as_int()))
        .collect::<Vec<BigUint>>();
    let mut scalar = BigUint::from(0u32);

    for (k, shift) in biga.iter().zip(vec![0u32, 64, 128, 192]) {
        scalar += k << shift
    }
    scalar
}

pub fn scalar_to_h4(sca: &BigUint) -> [Fr; 4] {
    let mut h4 = [Fr::ZERO; 4];
    let mask = BigUint::from_str_radix("FFFFFFFFFFFFFFFF", 16).unwrap();
    for (k, shift) in h4.iter_mut().zip(vec![0u32, 64, 128, 192]) {
        let tmp = (sca >> shift) & mask.clone();
        *k = Fr::from(tmp.to_u64().unwrap());
    }
    h4
}

pub fn h4_to_string(h4: &[Fr; 4]) -> String {
    let sc = h4_to_scalar(h4);
    format!("0x{:0>64}", sc.to_str_radix(16))
}

/// Field element array to Scalar
///
/// result = arr[0] + arr[1]*(2^32) + arr[2]*(2^64) + arr[3]*(2^96) + arr[3]*(2^128) + arr[3]*(2^160) + arr[3]*(2^192) + arr[3]*(2^224)
pub fn fea2scalar(fea: &[Fr; 8]) -> BigUint {
    let biga = fea
        .iter()
        .map(|e| BigUint::from(e.as_int()))
        .collect::<Vec<BigUint>>();
    let mut scalar = BigUint::from(0u32);

    for (k, shift) in biga.iter().zip(vec![0u32, 32, 64, 96, 128, 160, 192, 224]) {
        scalar += k << shift
    }

    scalar
}

#[inline(always)]
pub fn scalar2fea(scalar: &BigUint) -> [Fr; 8] {
    let mut res = [Fr::ZERO; 8];
    let mask = BigUint::from(0xFFFFFFFFu64);
    for (k, shift) in res
        .iter_mut()
        .zip(vec![0u32, 32, 64, 96, 128, 160, 192, 224])
    {
        let aux = (scalar >> shift) & mask.clone();
        *k = Fr::from(aux.to_u64().unwrap());
    }
    res
}

pub fn scalar2fe(scalar: u64) -> Fr {
    Fr::from(scalar)
}

/* Hexa string to/from field element (array) conversion */
pub fn string2fea(os: &str) -> Vec<Fr> {
    let scalar = BigUint::from_str_radix(&remove_0x(os), 16).unwrap();
    scalar2fea(&scalar).to_vec()
}

// `0x${Scalar.toString(sc, 16).padStart(64, '0')}`;
pub fn fea2string(fea: &[Fr; 8]) -> String {
    let f1 = fea2scalar(fea);
    format!("0x{:0>64}", f1.to_str_radix(16))
}

pub fn string2fe(os: &str) -> Fr {
    let os = remove_0x(os);
    from_hex(&os).unwrap()
}

#[cfg(test)]
mod test {
    use crate::scalar::*;
    use plonky::field_gl::Fr;
    #[test]
    fn test_prepend_zeros() {
        assert_eq!(prepend_zeros("abc", 10), "0000000abc".to_string());
    }

    #[test]
    fn test_h4_to_scalar() {
        let a = [Fr::from(32), Fr::from(3), Fr::from(2), Fr::from(1)];
        let out = h4_to_scalar(&a);
        let aa = scalar_to_h4(&out);
        assert_eq!(a, aa);
    }

    #[test]
    fn test_fea2scalar() {
        let a = [
            Fr::from(32),
            Fr::from(3),
            Fr::from(2),
            Fr::from(1),
            Fr::from(32),
            Fr::from(3),
            Fr::from(2),
            Fr::from(1),
        ];
        let out = fea2scalar(&a);
        let aa = scalar2fea(&out);
        assert_eq!(a, aa);
    }
}
