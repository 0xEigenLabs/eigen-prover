use crate::database::Database;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use num_traits::ToPrimitive;
use plonky::field_gl::Fr;
use plonky::Field;
use starky::linearhash::LinearHash;
use std::collections::HashMap;
use utils::errors::Result;
use utils::{fea2string, fea82scalar, scalar2fe, scalar2fea};

pub struct SmtSetResult {
    old_root: [Fr; 4],
    new_root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<i64, Vec<Fr>>,
    ins_key: [Fr; 4],
    ins_value: BigUint,
    is_old0: bool,
    old_value: BigUint,
    new_value: BigUint,
    mode: String,
    proof_hash_counter: u64,
}

pub struct SmtGetResult {
    root: [Fr; 4],
    key: [Fr; 4],
    siblings: HashMap<i64, Vec<Fr>>,
    ins_key: [Fr; 4],
    ins_value: BigUint,
    is_old0: bool,
    value: BigUint,
    proof_hash_counter: u64,
}

// https://github.com/iden3/circomlibjs/blob/main/src/smt.js#L12
// https://github.com/0xPolygonHermez/zkevm-prover/blob/v1.1.6-RC2-fork.4/src/statedb/smt.cpp
pub struct SMT {
    db: Database,
}

impl SMT {
    pub fn new(db: Database) -> Self {
        SMT {
            db: db
        }
    }
    pub fn set(
        &mut self,
        old_root: &[Fr; 4],
        key: &[Fr; 4],
        value: BigUint,
        persistent: bool,
    ) -> Result<SmtSetResult> {
        let mut r = old_root.clone();
        let mut new_root = old_root.clone();

        let keys = self.split_key(key);
        let mut level: i64 = 0;
        let mut proof_hash_counter = 0;

        let mut found_key = [Fr::ZERO; 4];
        let mut found_rkey = [Fr::ZERO; 4];
        let mut ins_key = [Fr::ZERO; 4];

        let mut siblings: HashMap<i64, Vec<Fr>> = HashMap::new();

        let mut ins_value = BigUint::zero();
        let mut old_value = BigUint::zero();
        let mut found_value = BigUint::zero();
        let mut found_old_val_h = [Fr::ZERO; 4];

        let mut is_old0 = false;
        let mut b_found_key = false;

        let mut acc_key: Vec<u64> = Vec::new();

        while Self::not_all_zero(&r) && !b_found_key {
            let str_root = fea2string(&r);
            let db_value = self.db.read(&str_root, level)?;
            siblings.insert(level, db_value.clone());
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
                found_value = fea82scalar(&value_fea).unwrap();
                found_rkey[0] = siblings[&level][0];
                found_rkey[1] = siblings[&level][1];
                found_rkey[2] = siblings[&level][2];
                found_rkey[3] = siblings[&level][3];
                self.join_key(&acc_key, &found_rkey, &mut found_key);
                b_found_key = true;
            } else {
                // Take either the first 4 (keys[level]=0) or the second 4 (keys[level]=1) siblings as the hash of the next level
                let _idx = keys[level as usize] as usize * 4;
                r[0] = siblings[&level][_idx];
                r[1] = siblings[&level][_idx + 1];
                r[2] = siblings[&level][_idx + 2];
                r[3] = siblings[&level][_idx + 3];
                acc_key.push(keys[level as usize]);

                level += 1;
            }
        }
        level -= 1;
        acc_key.pop();

        if Self::not_all_zero(old_root) {
            proof_hash_counter = std::cmp::min(siblings.len() as i64, level + 1);
            if found_value != BigUint::zero() {
                proof_hash_counter += 2;
            }
        }

        let mut mode: String;
        // If value!=0, it means we want to update an existing leaf node value, or create a new leaf node with the new value, in case keys are different
        if value != BigUint::zero() {
            if b_found_key {
                if key[0] == found_key[0]
                    && key[1] == found_key[1]
                    && key[2] == found_key[2]
                    && key[3] == found_key[3]
                {
                    mode = "update".to_string();

                    old_value = found_value;
                    // First, we create the db entry for the new VALUE, and store the calculated hash in newValH
                    let mut v = scalar2fea(&value);
                    let mut c = [Fr::ZERO; 4];
                    let mut new_val_h = [Fr::ZERO; 4];
                    self.hash_save_u64(&v, &c, &mut new_val_h)?;
                    // Second, we create the db entry for the new leaf node = RKEY + HASH, and store the calculated hash in new_leaf_hash
                    for i in 0..4 {
                        v[i] = found_key[i].as_int();
                    }
                    for i in 0..4 {
                        v[4 + i] = new_val_h[i].as_int();
                    }
                    c[0] = Fr::ONE;

                    let mut new_leaf_hash = [Fr::ZERO; 4];
                    self.hash_save_u64(&v, &c, &mut new_leaf_hash)?;
                    proof_hash_counter += 2;

                    if level >= 0 {
                        for jj in 0..4 {
                            let cur_v = siblings.get_mut(&level).unwrap();
                            cur_v[keys[level as usize] as usize * 4 + jj] = new_leaf_hash[jj];
                        }
                    } else {
                        new_root[0] = new_leaf_hash[0];
                        new_root[1] = new_leaf_hash[1];
                        new_root[2] = new_leaf_hash[2];
                        new_root[3] = new_leaf_hash[3];
                    }
                } else {
                    mode = "insertFound".to_string();
                    let mut level2 = level + 1;
                    let found_keys = self.split_key(&found_key);
                    while keys[level2 as usize] == found_keys[level2 as usize] {
                        level2 += 1;
                    }
                    let old_key = self.remove_key_bits(&found_key, level2 + 1);

                    // Insert a new leaf node for the old value, and store the hash in oldLeafHash

                    // Prepare the vector of field elements
                    let mut v = [Fr::ZERO; 8];
                    v[0..4].copy_from_slice(&old_key);
                    v[4..].copy_from_slice(&found_old_val_h);
                    let mut c = [Fr::ZERO; 4];
                    let mut old_leaf_hash = [Fr::ZERO; 4];
                    self.hash_save(&v, &c, &mut old_leaf_hash)?;
                    ins_key[0] = found_key[0];
                    ins_key[1] = found_key[1];
                    ins_key[2] = found_key[2];
                    ins_key[3] = found_key[3];
                    ins_value = found_value;
                    is_old0 = false;

                    let new_key = self.remove_key_bits(&key, level2 + 1);
                    let value_fea = scalar2fea(&value);

                    c[0] = Fr::ZERO;
                    let mut new_val_h = [Fr::ZERO; 4];
                    self.hash_save_u64(&value_fea, &c, &mut new_val_h)?;
                    v[0..4].copy_from_slice(&new_key);
                    v[4..].copy_from_slice(&new_val_h);

                    c[0] = Fr::ONE;
                    let mut new_leaf_hash = [Fr::ZERO; 4];
                    self.hash_save(&v, &c, &mut new_leaf_hash)?;

                    // Insert a new bifurcation intermediate node with both hashes (old and new) in the right position based on the bit

                    // Prepare the 2 hashes: new|old or old|new, based on the bit
                    let mut node = [Fr::ZERO; 8];
                    for j in 0..4usize {
                        node[keys[level2 as usize] as usize * 4 + j] = new_leaf_hash[j];
                        node[found_keys[level2 as usize] as usize * 4 + j] = old_leaf_hash[j];
                    }

                    c[0] = Fr::ZERO;
                    let mut r2 = [Fr::ZERO; 4];
                    self.hash_save(&node, &c, &mut r2)?;

                    proof_hash_counter += 4;
                    level2 -= 1;

                    while level2 != level {
                        node.fill(Fr::ZERO);
                        for j in 0..4usize {
                            node[keys[level2 as usize] as usize * 4 + j] = r2[j];
                        }

                        c[0] = Fr::ZERO;
                        self.hash_save(&node, &c, &mut r2)?;
                        proof_hash_counter += 1;
                        level2 -= 1;
                    }

                    if level >= 0 {
                        for jj in 0..4 {
                            let cur_v = siblings.get_mut(&level).unwrap();
                            cur_v[keys[level as usize] as usize * 4 + jj] = r2[jj];
                        }
                    } else {
                        new_root[0] = r2[0];
                        new_root[1] = r2[1];
                        new_root[2] = r2[2];
                        new_root[3] = r2[3];
                    }
                }
            } else {
                // insert without foundKey
                mode = "insertNotFound".to_string();
                // Build the new remaining key
                let new_key = self.remove_key_bits(&key, level + 1);
                let value_fea = scalar2fea(&value);

                let mut c = [Fr::ZERO; 4];
                let mut new_val_h = [Fr::ZERO; 4];
                self.hash_save_u64(&value_fea, &c, &mut new_val_h)?;
                let mut key_val_vec = [Fr::ZERO; 8];
                key_val_vec[0..4].copy_from_slice(&new_key);
                key_val_vec[4..].copy_from_slice(&new_val_h);

                // Capacity marks the node as leaf
                c[0] = Fr::ONE;
                let mut new_leaf_hash = [Fr::ZERO; 4];
                self.hash_save(&key_val_vec, &c, &mut new_leaf_hash)?;

                proof_hash_counter += 2;
                if level >= 0 {
                    for jj in 0..4 {
                        let cur_v = siblings.get_mut(&level).unwrap();
                        cur_v[keys[level as usize] as usize * 4 + jj] = new_leaf_hash[jj];
                    }
                } else {
                    new_root[0] = new_leaf_hash[0];
                    new_root[1] = new_leaf_hash[1];
                    new_root[2] = new_leaf_hash[2];
                    new_root[3] = new_leaf_hash[3];
                }
            }
        } else {
            // If value=0, we are possibly going to delete an existing node
            //
            // Setting a value=0 in an existing key, i.e. deleting
            if b_found_key
                && key[0] == found_key[0]
                && key[1] == found_key[1]
                && key[2] == found_key[2]
                && key[3] == found_key[3]
            {
                old_value = found_value;
                if level > 0 {
                    // If level > 0, we are going to delete and existing node (not the root node)
                    //
                    // Set the hash of the deleted node to zero
                    for jj in 0..4 {
                        let cur_v = siblings.get_mut(&level).unwrap();
                        cur_v[keys[level as usize] as usize * 4 + jj] = Fr::ZERO;
                    }

                    // Find if there is only one non-zero hash in the siblings list for this level
                    let mut ukey = Self::get_unique_sibling(&siblings[&level]);
                    if ukey >= 0 {
                        mode = "deleteFound".to_string();
                        let mut aux_fea = [Fr::ZERO; 4];
                        for i in 0..4 {
                            aux_fea[i] = siblings[&level][ukey as usize * 4 + 1];
                        }
                        let str_aux = fea2string(&aux_fea);
                        let db_value = self.db.read(&str_aux, level)?;
                        siblings.insert(level + 1, db_value.clone());

                        if siblings[&(level + 1)].len() > 8 && siblings[&(level + 1)][8] == Fr::ONE
                        {
                            let mut val_h = [Fr::ZERO; 4];
                            for i in 0..4 {
                                val_h[i] = siblings[&(level + 1)][4 + i];
                            }

                            let str_val_h = fea2string(&val_h);
                            if str_val_h.len() < 8 {
                                panic!("Smt::set() dbValue.size()<8 root: {}", str_val_h);
                            }
                            let db_value = self.db.read(&str_val_h, 0)?;

                            let mut val_a = [Fr::ZERO; 8];
                            for i in 0..8 {
                                val_a[i] = db_value[i];
                            }

                            let val = fea82scalar(&val_a).unwrap();

                            proof_hash_counter += 2;

                            let mut rkey = [Fr::ZERO; 4];
                            for i in 0..4 {
                                rkey[i] = siblings[&(level + 1)][i];
                            }

                            // Calculate the insKey
                            let mut aux_bits = acc_key.clone();
                            aux_bits.push(ukey as u64);
                            self.join_key(&aux_bits, &rkey, &mut ins_key);
                            ins_value = val;
                            is_old0 = false;

                            // Climb the branch until there are two siblings
                            while ukey >= 0 && level >= 0 {
                                level -= 1;
                                if level >= 0 {
                                    ukey = Self::get_unique_sibling(&siblings[&level]);
                                }
                            }

                            let old_key = self.remove_key_bits(&ins_key, level + 1);

                            let mut a = [Fr::ZERO; 8];
                            a[0..4].copy_from_slice(&old_key);
                            a[4..].copy_from_slice(&val_h);

                            let c = [Fr::ONE, Fr::ZERO, Fr::ZERO, Fr::ZERO];
                            let mut old_leaf_hash = [Fr::ZERO; 4];
                            self.hash_save(&a, &c, &mut old_leaf_hash)?;
                            proof_hash_counter += 1;
                            if level >= 0 {
                                for jj in 0..4 {
                                    let cur_v = siblings.get_mut(&level).unwrap();
                                    cur_v[keys[level as usize] as usize * 4 + jj] =
                                        old_leaf_hash[jj];
                                }
                            } else {
                                new_root[0] = old_leaf_hash[0];
                                new_root[1] = old_leaf_hash[1];
                                new_root[2] = old_leaf_hash[2];
                                new_root[3] = old_leaf_hash[3];
                            }
                        } else {
                            mode = "deleteNotFound".to_string();
                        }
                    } else {
                        // 2 siblings found
                        mode = "deleteNotFound".to_string()
                    }
                } else {
                    // If level=0, this means we are deleting the root node
                    mode = "deleteLast".to_string();
                    new_root[0] = Fr::ZERO;
                    new_root[1] = Fr::ZERO;
                    new_root[2] = Fr::ZERO;
                    new_root[3] = Fr::ZERO;
                }
            } else {
                // Setting to zero a node that does not exist, so nothing to do
                mode = "zeroToZero".to_string();
                if b_found_key {
                    ins_key.copy_from_slice(&found_key);
                    ins_value = found_value;
                    is_old0 = false;
                }
            }
        }

        // Delete the extra siblings
        // map< uint64_t, vector<Goldilocks::Element> >::iterator it;
        // it = siblings.find(level+1);
        // siblings.erase(it, siblings.end());

        siblings.remove(&(level + 1));
        while level >= 0 {
            let mut a = [Fr::ZERO; 8];
            let mut c = [Fr::ZERO; 4];
            a.copy_from_slice(&siblings[&level][0..8]);
            c.copy_from_slice(&siblings[&level][8..12]);

            self.hash_save(&a, &c, &mut new_root)?;
            proof_hash_counter += 1;

            level -= 1;
            if level >= 0 {
                // Overwrite the first or second 4 elements (based on keys[level] bit) with the new root hash from the lower level
                for jj in 0..4 {
                    let cur_v = siblings.get_mut(&level).unwrap();
                    cur_v[keys[level as usize] as usize * 4 + jj] = new_root[jj];
                }
            }
        }
        if persistent
            && (old_root[0] != new_root[0]
                || old_root[1] != new_root[1]
                || old_root[2] != new_root[2]
                || old_root[3] != new_root[3])
        {
            self.save_state_root(&new_root)?;
        }

        Ok(SmtSetResult {
            old_root: *old_root,
            new_root: new_root,
            key: *key,
            siblings: siblings,
            ins_key: ins_key,
            ins_value: ins_value,
            is_old0: is_old0,
            old_value: old_value,
            new_value: value.into(),
            mode: mode,
            proof_hash_counter: proof_hash_counter as u64,
        })
    }

    pub fn get(&mut self, root: &[Fr; 4], key: &[Fr; 4]) -> Result<SmtGetResult> {
        let mut r = root.clone();
        // Get a list of the bits of the key to navigate top-down through the tree
        let keys = self.split_key(key);
        let mut level: i64 = 0;
        let mut b_found_key = false;

        let mut acc_key: Vec<u64> = Vec::new();
        let mut found_key = [Fr::ZERO; 4];
        let mut ins_key = [Fr::ZERO; 4];
        let mut siblings: HashMap<i64, Vec<Fr>> = HashMap::new();
        let mut ins_value: BigUint = BigUint::zero();
        let mut value: BigUint = BigUint::zero();
        let mut found_val: BigUint = BigUint::zero();

        let mut is_old0 = true;

        // Start natigating the tree from the top: r = root
        // Go down while r!=0 (while there is branch) until we find the key
        while Self::not_all_zero(&r) && !b_found_key {
            // Read the content of db for entry r: siblings[&level] = db.read(r)
            let str_r = fea2string(&r);
            let db_value = self.db.read(&str_r, level)?;
            // Get a copy of the content of this database entry, at the corresponding level: 0, 1...
            siblings.insert(level, db_value.clone());

            // if siblings[&level][8]=1 then this is a leaf
            if siblings[&level].len() > 8 && siblings[&level][8] == Fr::ONE {
                // Second 4 elements are the hash of the value, so we can get value=db(valueHash)
                let mut value_hash_fea = [Fr::ZERO; 4];
                value_hash_fea[0] = siblings[&level][4];
                value_hash_fea[1] = siblings[&level][5];
                value_hash_fea[2] = siblings[&level][6];
                value_hash_fea[3] = siblings[&level][7];

                let str_value_hash = fea2string(&value_hash_fea);
                let db_value = self.db.read(&str_value_hash, 0)?;
                // dbres = db.read(valueHashString, dbValue, dbReadLog);

                // First 4 elements are the remaining key
                let mut found_r_key = [Fr::ZERO; 4];
                found_r_key[0] = siblings[&level][0];
                found_r_key[1] = siblings[&level][1];
                found_r_key[2] = siblings[&level][2];
                found_r_key[3] = siblings[&level][3];

                // We convert the 8 found value elements to a scalar called foundVal
                let mut fea = [Fr::ZERO; 8];
                fea.copy_from_slice(&db_value[0..8]);
                found_val = fea82scalar(&fea).unwrap();

                // We construct the whole key of that value in the database, and we call it foundKey
                self.join_key(&acc_key, &found_r_key, &mut found_key);
                b_found_key = true;
            }
            // If this is an intermediate node
            else {
                // Take either the first 4 (keys[level]=0) or the second 4 (keys[level]=1) siblings as the hash of the next level
                let _idx = (keys[level as usize] * 4) as usize;
                r[0] = siblings[&level][_idx];
                r[1] = siblings[&level][_idx + 1];
                r[2] = siblings[&level][_idx + 2];
                r[3] = siblings[&level][_idx + 3];

                // Store the used key bit in accKey
                acc_key.push(keys[level as usize]);

                // Increase the level
                level += 1;
            }
        }

        // One step back
        level -= 1;
        acc_key.pop();

        // if we found the key, then we reached a leaf node while going down the tree
        if b_found_key {
            // if foundKey==key, then foundVal is what we were looking for
            if key[0] == found_key[0]
                && key[1] == found_key[1]
                && key[2] == found_key[2]
                && key[3] == found_key[3]
            {
                value = found_val;
            }
            // if foundKey!=key, then the requested value was not found
            else {
                ins_key[0] = found_key[0];
                ins_key[1] = found_key[1];
                ins_key[2] = found_key[2];
                ins_key[3] = found_key[3];
                ins_value = found_val;
                is_old0 = false;
            }
        }

        // We leave the siblings only up to the leaf node level
        // map< uint64_t, vector<Goldilocks::Element> >::iterator it;
        // it = siblings.find(level+1);
        // siblings.erase(it, siblings.end());
        siblings.remove(&(level + 1));

        let mut ret = SmtGetResult {
            root: *root,
            key: *key,
            value: value.clone(),
            ins_key: ins_key,
            ins_value: ins_value,
            is_old0: is_old0,
            proof_hash_counter: 0,
            siblings: siblings.clone(),
        };

        if Self::not_all_zero(root) {
            ret.proof_hash_counter = siblings.len() as u64;
            if value != BigUint::zero() || !is_old0 {
                ret.proof_hash_counter += 2;
            }
        } else {
            ret.proof_hash_counter = 0;
        }
        Ok(ret)
    }

    #[inline(always)]
    fn not_all_zero(r: &[Fr; 4]) -> bool {
        !Fr::is_zero(&r[0]) || !Fr::is_zero(&r[1]) || !Fr::is_zero(&r[2]) || !Fr::is_zero(&r[3])
    }

    fn split_key(&mut self, key: &[Fr; 4]) -> Vec<u64> {
        let mut ru = [0u64; 4];
        for i in 0..4 {
            ru[i] = key[i].as_int();
        }
        // Split the key in bits, taking one bit from a different scalar every time
        let mut result = vec![];
        for i in 0..64 {
            for j in 0..4 {
                let aux = ru[j] & 1;
                result.push(aux);
                ru[j] = ru[j] >> 1;
            }
        }
        result
    }

    fn join_key(&mut self, bits: &Vec<u64>, rkey: &[Fr; 4], auxk: &mut [Fr; 4]) {
        let mut n = [0u64, 0, 0, 0];
        let mut accs = [0u64, 0, 0, 0];
        for i in 0..bits.len() {
            if bits[i] > 0 {
                accs[i % 4] = accs[i % 4] | (1u64 << n[i % 4])
            }
            n[i % 4] += 1;
        }
        for i in 0..4 {
            auxk[i] = rkey[i];
        }
        let mut aux = BigUint::zero();
        for i in 0..4 {
            // BigUint = BigUint::from(auxk.as_int());
            aux = (aux << n[i]) | BigUint::from(accs[i]);
            auxk[i] = Fr::from(aux.to_u64().unwrap());
        }
    }

    fn get_unique_sibling(a: &Vec<Fr>) -> i32 {
        let mut n_found = 0;
        let mut fnd: i32 = 0;
        for i in (0..a.len()).step_by(4) {
            if Self::not_all_zero(&a[i..(i + 4)].try_into().unwrap()) {
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

    fn hash_save_u64(&mut self, a: &[u64; 8], c: &[Fr; 4], hash: &mut [Fr; 4]) -> Result<()> {
        let fea: [Fr; 8] = a
            .into_iter()
            .map(|i| Fr::from(*i))
            .collect::<Vec<Fr>>()
            .try_into()
            .unwrap();
        self.hash_save(&fea, c, hash)
    }
    fn hash_save(&mut self, a: &[Fr; 8], c: &[Fr; 4], hash: &mut [Fr; 4]) -> Result<()> {
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

        let str_digest = fea2string(digest.try_into().unwrap());

        let mut db_value: Vec<Fr> = Vec::new();
        for i in 0..8 {
            db_value.push(a[i]);
        }
        for i in 0..4 {
            db_value.push(c[i]);
        }
        self.db.write(&str_digest, &db_value, true)?;
        Ok(())
    }

    fn remove_key_bits(&mut self, key: &[Fr; 4], nbits: i64) -> [Fr; 4] {
        let full_lvl: i64 = nbits / 4;
        let mut auxk = [0u64; 4];

        for i in 0..4 {
            auxk[i] = key[i].as_int();
        }

        for i in 0..4 {
            let mut n = full_lvl;
            if full_lvl * 4 + i < nbits {
                n += 1;
            }
            auxk[i as usize] = auxk[i as usize] >> n;
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
    use crate::database::Database;
    use utils::*;

    #[test]
    fn test_smt_split_key() {
        env_logger::init();
        let db = Database::new();
        let mut smt = SMT::new(db);
        let key = "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000".to_string(); // bn254::prime - 1
        let key = string2fea(&key);
        let result = smt.split_key(&[key[0], key[1], key[2], key[3]]);
        println!("{:?}", result);
    }
}
