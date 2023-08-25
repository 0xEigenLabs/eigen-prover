use plonky::field_gl::Fr;
use std::collections::HashMap;
use crate::database::Database;
use utils::errors::Result;
use plonky::Field;

pub struct SmtSetResult {
    old_root: [Fr; 4],
    new_root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<u64, Vec<Fr>>,
    ins_key: [Fr; 4],
    ins_value: u64,
    is_old0: bool,
    old_value: u64,
    new_value: u64,
    mode: String,
    proof_hash_counter: u64,
}

pub struct SmtGetResult {
    root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<u64, Vec<Fr>>,
    ins_key: [Fr; 4],
    is_old0: bool,
    value: u64,
    proof_hash_counter: u64,
}

pub struct SMT {
    db: Database,
}

impl SMT {
    pub fn set(&mut self, old_root: &[Fr; 4], key: &[Fr; 4], value: u64, persistent: bool) -> Result<SmtSetResult> {
        let keys = self.split_key(key);
        let level = 0;
        let proof_hash_counter = 0;
    }

    fn split_key(&mut self, key: &[Fr; 4]) -> [u64; 4] {
        let mut ru = [0u64; 4];
        for i in 0..4 {
            ru[i] = key[i].as_int();
        }
        ru
    }

    fn get_unique_sibling(a: &Vec<Fr>) -> i32 {
        let mut n_found = 0;
        let mut fnd: i32 = 0;
        for i in (0..a.len()).step_by(4) {
            if a[i] != Fr::zero() || a[i+1] != Fr::zero() || a[i+2] != Fr::zero() || a[i+3] != Fr::zero() {
                n_found += 1;
                fnd = (i as i32)/4;
            }
        }
        if n_found == 1 {
            return fnd;
        }
        -1
    }

    fn save_state_root(&mut self, state_root: &[Fr; 4]) {
        let mut db_value: Vec<Fr> = Vec::new();
        for i in 0..4 {
            db_value.push(state_root[i]);
        }
        for i in 0..8 {
            db_value.push(Fr::zero());
        }
        self.db.write()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
