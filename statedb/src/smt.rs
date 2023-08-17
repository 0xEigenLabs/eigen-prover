use plonky::field_gl::Fr;
use std::collections::HashMap;

pub struct SmtSetResult {
    old_root: [Fr; 4],
    new_root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<u64, Vec<Fr>>,
    ins_key: [Fr; 4],
    ins_value: u128,
    is_old0: bool,
    old_value: u128,
    new_value: u128,
    mode: String,
    proof_hash_counter: u64,
}

pub struct SmtGetResult {
    root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<u64, Vec<Fr>>,
    ins_key: [Fr; 4],
    is_old0: bool,
    value: u128,
    proof_hash_counter: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
