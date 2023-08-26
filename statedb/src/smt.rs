use crate::database::Database;
use num_bigint::BigUint;
use plonky::field_gl::Fr;
use plonky::Field;
use starky::linearhash::LinearHash;
use std::collections::HashMap;
use utils::errors::Result;
use utils::{fea2scalar, fea2string, scalar2fe};

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
    pub fn set(
        &mut self,
        old_root: &[Fr; 4],
        key: &[Fr; 4],
        value: u64,
        persistent: bool,
    ) -> Result<SmtSetResult> {
        let mut r = [Fr::ZERO; 4];
        for i in 0..4 {
            r[i] = old_root[i];
        }
        let mut new_root = [Fr::ZERO; 4];
        for i in 0..4 {
            new_root[i] = old_root[i];
        }

        let keys = self.split_key(key);
        let mut level = 0;
        let proof_hash_counter = 0;

        let mut found_key = [Fr::ZERO; 4];
        let mut found_rkey = [Fr::ZERO; 4];
        let mut ins_key = [Fr::ZERO; 4];

        let siblings: HashMap<u64, Vec<Fr>> = HashMap::new();

        let mut ins_value = BigUint::zero();
        let mut old_value = BigUint::zero();
        let mut found_value = BigUint::zero();
        let mut found_old_val_h = [Fr::ZERO; 4];

        let mut is_old0 = false;
        let mut b_found_key = false;

        let mut acc_key: Vec<u64>;
        let mut last_acc_key: BigUint = 0;

        while not_all_zero(&r) && !b_found_key {
            let str_root = fea2string(&r);
            let db_value = self.db.read(&str_root, level)?;
            siblings.insert(level, db_value);
            if db_value.len() > 8 && db_value[8] == Fr::ONE {
                found_old_val_h[0] = db_value[4];
                found_old_val_h[1] = db_value[5];
                found_old_val_h[2] = db_value[6];
                found_old_val_h[3] = db_value[7];
                let s_value_hash = fea2string(&found_old_val_h);
                let db_value = self.db.read(&s_value_hash, 0)?;
                let mut value_fea = [Fr::ZERO; 8];
                for i in 0..8 {
                    value_fea[i] = db_value[i];
                }
                found_value = fea2scalar(&value_fea);
                found_rkey[0] = siblings[level][0];
                found_rkey[1] = siblings[level][1];
                found_rkey[2] = siblings[level][2];
                found_rkey[3] = siblings[level][3];
                self.join_key(acc_key, found_rkey, &mut found_key);
                b_found_key = true;
            } else {
                // Take either the first 4 (keys[level]=0) or the second 4 (keys[level]=1) siblings as the hash of the next level
                r[0] = siblings[level][keys[level] * 4];
                r[1] = siblings[level][keys[level] * 4 + 1];
                r[2] = siblings[level][keys[level] * 4 + 2];
                r[3] = siblings[level][keys[level] * 4 + 3];
                acc_key.push(keys[level]);

                level += 1;
            }
        }
        level -= 1;
        acc_key.pop();

        if not_all_zero(old_root) {
            proof_hash_counter = std::cmp::min(siblings.len(), level + 1);
            if found_value != 0 {
                proof_hash_counter += 2;
            }
        }

        // update the existing node
        if value != 0 {}
    }

    #[inline(always)]
    fn not_all_zero(r: &[Fr; 4]) -> bool {
        !Fr::is_zero(r[0]) || !Fr::is_zero(r[1]) || !Fr::is_zero(r[2]) || !Fr::is_zero(r[3])
    }

    pub fn get(&mut self, root: &[Fr; 4], key: &[Fr; 4]) -> Result<SmtGetResult> {
        let mut r = [Fr::ZERO; 4];
        for i in 0..4 {
            r[i] = key[i];
        }
    }

    fn split_key(&mut self, key: &[Fr; 4]) -> [u64; 4] {
        let mut ru = [0u64; 4];
        for i in 0..4 {
            ru[i] = key[i].as_int();
        }
        ru
    }

    fn join_key(&mut self, bits: &Vec<u64>, rkey: &[Fr; 4], &mut auxk: [Fr; 4]) {
        let n = [0u64, 0, 0, 0];
        let accs = [0u64, 0, 0, 0];
        for i in 0..bits.len() {
            if b > 0 {
                accs[i % 4] = accs[i % 4] | (1u64 << n[i % 4])
            }
            n[i % 4] += 1;
        }
        for i in 0..4 {
            auxk[i] = rkey[i];
        }
        let mut aux: BigUint;
        for i in 0..4 {
            BigUint = BigUint::from(auxk.as_int());
            aux = (aux << n[i]) | BigUint::from(accs[i]);
            auxk[i] = Fr::from(aux.to_u64());
        }
    }

    fn get_unique_sibling(a: &Vec<Fr>) -> i32 {
        let mut n_found = 0;
        let mut fnd: i32 = 0;
        for i in (0..a.len()).step_by(4) {
            if not_all_zero(&a[i..(i + 4)]) {
                n_found += 1;
                fnd = (i as i32) / 4;
            }
        }
        if n_found == 1 {
            return fnd;
        }
        -1
    }

    fn save_state_root(&mut self, state_root: &[Fr; 4]) -> Result<usize> {
        let mut db_value: Vec<Fr> = Vec::new();
        for i in 0..4 {
            db_value.push(state_root[i]);
        }
        for i in 0..8 {
            db_value.push(Fr::ZERO);
        }
        self.db
            .write(&self.db.db_state_root_key.to_string(), &db_value, true)
    }

    fn hash_save(&mut self, a: &[Fr; 8], c: &[Fr; 4], hash: &[Fr; 4]) -> Result<usize> {
        let mut db_value = [Fr::ZERO; 12];
        for i in 0..8 {
            db_value[i] = a[i];
        }
        for i in 0..4 {
            db_value[i + 8] = c[i];
        }

        let p = LinearHash::new();
        let digest = p.hash(&db_value, 0).unwrap();
        let digest = digest.as_elements();

        let str_digest = fea2string(digest);

        let mut db_value: Vec<Fr> = Vec::new();
        for i in 0..8 {
            db_value.push(a[i]);
        }
        for i in 0..4 {
            db_value.push(c[i]);
        }
        self.db.write(&str_digest, &db_value, true)
    }

    fn remove_key_bits(&mut self, key: &[Fr; 4], u64: nbits) -> [Fr; 4] {
        let full_lvl: u64 = nbits / 4;
        let auxk: [u64; 4];

        for i in 0..4 {
            auxk[i] = key[i].as_int();
        }

        for i in 0..4 {
            let n = full_lvl;
            if full_lvl * 4 + i < nbits {
                n += 1;
            }
            auxk[i] = auxk[i] >> n;
        }

        let mut r = [Fr::ZERO; 4];
        for i in 0..4 {
            r[i] = scalar2fe(auxk[i]);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
