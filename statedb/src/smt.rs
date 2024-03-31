use crate::database::Database;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use plonky::field_gl::Fr;
use plonky::from_hex;
use plonky::Field;
use starky::linearhash::LinearHash;
use starky::traits::MTNodeType;
use std::collections::HashMap;
use utils::errors::Result;
use utils::{fea2scalar, h4_to_scalar, h4_to_string, scalar2fe, scalar2fea};

#[derive(Debug)]
pub struct SmtSetResult {
    pub old_root: [Fr; 4],
    pub new_root: [Fr; 4],
    pub key: [Fr; 4],
    pub siblings: HashMap<i64, Vec<Fr>>,
    pub ins_key: [Fr; 4],
    pub ins_value: BigUint,
    pub is_old0: bool,
    pub old_value: BigUint,
    pub new_value: BigUint,
    pub mode: String,
    pub proof_hash_counter: u64,
}

#[derive(Debug)]
pub struct SmtGetResult {
    pub root: [Fr; 4],
    pub key: [Fr; 4],
    pub siblings: HashMap<i64, Vec<Fr>>,
    pub ins_key: [Fr; 4], // found key, not equal to key when is_old0 = false
    pub ins_value: BigUint,
    pub is_old0: bool,
    pub value: BigUint,
    pub proof_hash_counter: u64,
}

// https://github.com/0xPolygonHermez/zkevm-commonjs/blob/v0.6.0.0/src/smt.js
#[derive(Default, Debug)]
pub struct SMT {
    db: Database,
}

impl SMT {
    pub const EMPTY: [Fr; 4] = [Fr::ZERO, Fr::ZERO, Fr::ZERO, Fr::ZERO];
    pub const ONE: [Fr; 4] = [Fr::ONE, Fr::ZERO, Fr::ZERO, Fr::ZERO];
    pub fn new(db: Database) -> Self {
        SMT { db }
    }
    pub fn db_mut(&mut self) -> &mut Database {
        &mut self.db
    }
    pub fn set(
        &mut self,
        old_root: &[Fr; 4],
        key: &[Fr; 4],
        value: BigUint,
        persistent: bool,
    ) -> Result<SmtSetResult> {
        let mut r = *old_root;
        let mut new_root = *old_root;

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

        log::debug!(
            "Set check: root {:?}, node is zero: {}, !b_found_key: {}",
            r,
            Self::node_is_zero(&r),
            !b_found_key
        );
        while !Self::node_is_zero(&r) && !b_found_key {
            let db_value = self.db.read(&r)?;
            siblings.insert(level, db_value.clone());
            if db_value.len() > 8 && db_value[8] == Fr::ONE {
                found_old_val_h.copy_from_slice(&db_value[4..8]);
                let db_value = self.db.read(&found_old_val_h)?;
                let mut value_fea = [Fr::ZERO; 8];
                value_fea[..8].copy_from_slice(&db_value[..8]);
                found_value = fea2scalar(&value_fea);
                found_rkey.copy_from_slice(&siblings[&level][0..4]);
                self.join_key(&acc_key, &found_rkey, &mut found_key);
                b_found_key = true;
                log::debug!("Smt::set found at level:{}, found_val: {:?}, found_key: {:?}, found_rkey: {:?}", level, found_value, found_key, found_rkey);
            } else {
                // Take either the first 4 (keys[level]=0) or the second 4 (keys[level]=1) siblings as the hash of the next level
                let idx = keys[level as usize] as usize * 4;
                r.copy_from_slice(&siblings[&level][idx..(idx + 4)]);
                acc_key.push(keys[level as usize]);

                log::debug!(
                    "Smt::set down 1 from level:{}, key[level]: {:?}, root/hash: {:?}",
                    level,
                    keys[level as usize],
                    h4_to_scalar(&r)
                );

                level += 1;
            }
        }
        log::debug!("One step back: {level}");
        // one step back
        level -= 1;
        acc_key.pop();

        // If value!=0, it means we want to update an existing leaf node value, or create a new leaf node with the new value, in case keys are different
        if !Self::node_is_zero(old_root) {
            proof_hash_counter = std::cmp::min(siblings.len() as i64, level + 1);
            if found_value != BigUint::zero() {
                proof_hash_counter += 2;
            }
        }

        let mut mode: String;
        // If value!=0, it means we want to update an existing leaf node value, or create a new leaf node with the new value, in case keys are different
        if !value.is_zero() {
            log::debug!(
                "Insert or update {:?} to {}, b_found_key: {}, found_key: {:?}",
                key,
                value,
                b_found_key,
                found_key
            );
            if b_found_key {
                if Self::node_is_eq(key, &found_key) {
                    mode = "update".to_string();
                    log::debug!("Smt::set(): mode: {}", mode);

                    old_value = found_value;
                    // First, we create the db entry for the new VALUE, and store the calculated hash in newValH
                    let mut v = scalar2fea(&value);
                    let mut c = [Fr::ZERO; 4];
                    let new_val_h = self.hash_save(&v, &c)?;
                    // Second, we create the db entry for the new leaf node = RKEY + HASH, and store the calculated hash in new_leaf_hash
                    v[..4].copy_from_slice(&found_key[..4]);
                    v[4..(4 + 4)].copy_from_slice(&new_val_h[..4]);

                    // Prepare the capacity = 1, 0, 0, 0
                    c[0] = Fr::ONE;

                    // Save and get the hash
                    let new_leaf_hash = self.hash_save(&v, &c)?;
                    proof_hash_counter += 2;

                    // If we are not at the top, the new leaf hash will become part of the higher level content, based on the keys[level] bit
                    if level >= 0 {
                        for jj in 0..4 {
                            let cur_v = siblings.get_mut(&level).unwrap();
                            cur_v[keys[level as usize] as usize * 4 + jj] = new_leaf_hash[jj];
                        }
                    } else {
                        // If this is the top, then this is the new root
                        new_root.copy_from_slice(&new_leaf_hash);
                    }
                    log::debug!("Smt::set() updated an existing node at level={level} leaf node hash={}, value hash = {}", h4_to_scalar(&new_leaf_hash), h4_to_scalar(&new_val_h));
                } else {
                    mode = "insertFound".to_string();
                    log::debug!("Smt3::set() mode: {}", mode);

                    // Increase the level since we need to create a new leaf node
                    let mut level2 = level + 1;
                    // Split the found key in bits
                    let found_keys = self.split_key(&found_key);
                    // While the key bits are the same, increase the level; we want to find the first bit when the keys differ
                    while keys[level2 as usize] == found_keys[level2 as usize] {
                        level2 += 1;
                    }
                    // Store the key of the old value at the new level
                    let old_key = self.remove_key_bits(&found_key, level2 + 1);

                    // Insert a new leaf node for the old value, and store the hash in oldLeafHash

                    // Prepare the vector of field elements
                    let mut v = [Fr::ZERO; 8];
                    v[0..4].copy_from_slice(&old_key);
                    v[4..].copy_from_slice(&found_old_val_h);
                    // Prepare the capacity = 1, 0, 0, 0

                    // Save and get the hash
                    let old_leaf_hash = self.hash_save(&v, &Self::ONE.clone())?;
                    // Record the inserted key for the reallocated old value
                    ins_key.copy_from_slice(&found_key);
                    ins_value = found_value;
                    is_old0 = false;

                    log::debug!(
                        "Smt::set() stored leaf node insValue={}, insKey={}",
                        ins_value,
                        h4_to_scalar(&ins_key)
                    );

                    // Insert a new value node for the new value, and store the calculated hash in newValH

                    // Calculate the key of the new leaf node of the new value
                    let new_key = self.remove_key_bits(key, level2 + 1);
                    // Convert the value scalar to an array of field elements
                    let value_fea = scalar2fea(&value);

                    // Capacity is marking the node as intermediate
                    // Create the intermediate node
                    let new_val_h = self.hash_save(&value_fea, &Self::EMPTY.clone())?;

                    // Insert a new leaf node for the new key-value hash pair

                    // Calculate the key-value hash content
                    v[0..4].copy_from_slice(&new_key);
                    v[4..].copy_from_slice(&new_val_h);

                    let new_leaf_hash = self.hash_save(&v, &Self::ONE.clone())?;

                    // Insert a new bifurcation intermediate node with both hashes (old and new) in the right position based on the bit

                    // Prepare the 2 hashes: new|old or old|new, based on the bit
                    let mut node = [Fr::ZERO; 8];
                    for j in 0..4usize {
                        node[keys[level2 as usize] as usize * 4 + j] = new_leaf_hash[j];
                        node[found_keys[level2 as usize] as usize * 4 + j] = old_leaf_hash[j];
                    }

                    // Capacity is marking the node as intermediate
                    let mut r2 = self.hash_save(&node, &Self::EMPTY.clone())?;

                    proof_hash_counter += 4;
                    level2 -= 1;
                    log::info!(
                        "Smt::set() inserted a new intermediate node level= {}, leaf node hash= {}",
                        level2,
                        h4_to_scalar(&r2)
                    );

                    while level2 != level {
                        node.fill(Fr::ZERO);
                        for j in 0..4usize {
                            node[keys[level2 as usize] as usize * 4 + j] = r2[j];
                        }

                        // Capacity is marking the node as intermediate

                        // Create the intermediate node and store the calculated hash in r2
                        r2 = self.hash_save(&node, &Self::EMPTY.clone())?;
                        proof_hash_counter += 1;
                        log::info!(
                            "Smt::set() inserted a new intermediate level= {}, leaf node hash={}",
                            level2,
                            h4_to_scalar(&r2)
                        );
                        // Climb the branch
                        level2 -= 1;
                    }

                    // If not at the top of the tree, update the stored siblings for the root of the branch
                    if level >= 0 {
                        for jj in 0..4 {
                            let cur_v = siblings.get_mut(&level).unwrap();
                            cur_v[keys[level as usize] as usize * 4 + jj] = r2[jj];
                        }
                    } else {
                        // If at the top of the tree, update newRoot
                        new_root.copy_from_slice(&r2);
                    }
                }
            } else {
                // insert without foundKey
                mode = "insertNotFound".to_string();
                // We could not find any key with any bit in common, so we need to create a new intermediate node, and a new leaf node

                // Value node creation

                // Build the new remaining key
                let new_key = self.remove_key_bits(key, level + 1);
                // Convert the scalar value to an array of 8 field elements
                let value_fea = scalar2fea(&value);
                log::debug!(
                    "mode: {}, new_key: {:?}, value_fea: {:?}",
                    mode,
                    new_key,
                    value_fea.iter().map(|e| e.as_int()).collect::<Vec<u64>>()
                );

                // Capacity mars the node as intermediate/value
                // Create the node and store the calculated hash in newValH
                let new_val_h = self.hash_save(&value_fea, &Self::EMPTY.clone())?;
                // Insert the new key-value hash leaf node

                // Calculate the node content: key|hash
                let mut key_val_vec = [Fr::ZERO; 8];
                key_val_vec[0..4].copy_from_slice(&new_key);
                key_val_vec[4..].copy_from_slice(&new_val_h);

                // Capacity marks the node as leaf
                // Create the new leaf node and store the calculated hash in newLeafHash
                let new_leaf_hash = self.hash_save(&key_val_vec, &Self::ONE.clone())?;

                proof_hash_counter += 2;
                // If not at the top of the tree, update siblings with the new leaf node hash
                if level >= 0 {
                    for jj in 0..4 {
                        let cur_v = siblings.get_mut(&level).unwrap();
                        cur_v[keys[level as usize] as usize * 4 + jj] = new_leaf_hash[jj];
                    }
                } else {
                    // If at the top of the tree, update the new root
                    new_root.copy_from_slice(&new_leaf_hash);
                }
            }
        } else {
            // If value=0, we are possibly going to delete an existing node
            // Setting a value=0 in an existing key, i.e. deleting
            log::debug!(
                "b_found_key: {}, key: {:?}, found_key: {:?}",
                b_found_key,
                key,
                found_key
            );
            if b_found_key && Self::node_is_eq(key, &found_key) {
                old_value = found_value;
                if level >= 0 {
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
                        log::debug!("Smt::set(), mode deleteFound");
                        let mut aux_fea = [Fr::ZERO; 4];
                        for (i, aux) in aux_fea.iter_mut().enumerate() {
                            *aux = siblings[&level][ukey as usize * 4 + i];
                        }
                        let db_value = self.db.read(&aux_fea)?;
                        siblings.insert(level + 1, db_value);

                        if siblings[&(level + 1)].len() > 8 && siblings[&(level + 1)][8] == Fr::ONE
                        {
                            let mut val_h = [Fr::ZERO; 4];
                            val_h[..4].copy_from_slice(&siblings[&(level + 1)][4..(4 + 4)]);

                            let db_value = self.db.read(&val_h)?;

                            let mut val_a = [Fr::ZERO; 8];
                            val_a[..8].copy_from_slice(&db_value[..8]);

                            let val = fea2scalar(&val_a);

                            proof_hash_counter += 2;

                            let mut rkey = [Fr::ZERO; 4];
                            rkey[..4].copy_from_slice(&siblings[&(level + 1)][..4]);

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

                            let old_leaf_hash = self.hash_save(&a, &Self::ONE.clone())?;
                            proof_hash_counter += 1;
                            if level >= 0 {
                                for jj in 0..4 {
                                    let cur_v = siblings.get_mut(&level).unwrap();
                                    cur_v[keys[level as usize] as usize * 4 + jj] =
                                        old_leaf_hash[jj];
                                }
                            } else {
                                new_root.copy_from_slice(&old_leaf_hash);
                            }
                        } else {
                            mode = "deleteNotFound".to_string();
                            log::debug!("Smt::set(), mode deleteNotFound");
                        }
                    } else {
                        // 2 siblings found
                        log::debug!("Smt::set(), mode deleteNotFound buf siblings found");
                        mode = "deleteNotFound".to_string()
                    }
                } else {
                    // If level=0, this means we are deleting the root node
                    mode = "deleteLast".to_string();
                    log::debug!("Smt::set(), mode deleteLast");
                    new_root.copy_from_slice(&Self::EMPTY.clone());
                }
            } else {
                // Setting to zero a node that does not exist, so nothing to do
                mode = "zeroToZero".to_string();
                log::debug!("Smt::set(), zeroToZero, nothing to do");
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
        log::debug!("level: {}, {:?}", level, siblings);
        siblings.retain(|k, _| *k <= level);
        log::debug!("retain {:?}", siblings);
        while level >= 0 {
            let mut a = [Fr::ZERO; 8];
            let mut c = [Fr::ZERO; 4];
            a.copy_from_slice(&siblings[&level][0..8]);
            c.copy_from_slice(&siblings[&level][8..12]);

            new_root = self.hash_save(&a, &c)?;
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
            new_root,
            key: *key,
            siblings,
            ins_key,
            ins_value,
            is_old0,
            old_value,
            new_value: value,
            mode,
            proof_hash_counter: proof_hash_counter as u64,
        })
    }

    pub fn get(&mut self, root: &[Fr; 4], key: &[Fr; 4]) -> Result<SmtGetResult> {
        let mut r = *root;
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
        while !Self::node_is_zero(&r) && !b_found_key {
            // Read the content of db for entry r: siblings[&level] = db.read(r)
            let db_value = self.db.read(&r)?;
            // Get a copy of the content of this database entry, at the corresponding level: 0, 1...
            siblings.insert(level, db_value);

            // if siblings[&level][8]=1 then this is a leaf: [X, X, X, X, X, X, X, X, 1, 0, 0, 0]
            if siblings[&level].len() > 8 && siblings[&level][8] == Fr::ONE {
                log::info!("Node is a leaf");
                // Second 4 elements are the hash of the value, so we can get value=db(valueHash)
                let mut value_hash_fea = [Fr::ZERO; 4];
                value_hash_fea.copy_from_slice(&siblings[&level][4..8]);
                let db_value = self.db.read(&value_hash_fea)?;

                // First 4 elements are the remaining key
                let mut found_r_key = [Fr::ZERO; 4];
                found_r_key.copy_from_slice(&siblings[&level][..4]);

                // We convert the 8 found value elements to a scalar called foundVal
                let mut fea = [Fr::ZERO; 8];
                fea.copy_from_slice(&db_value[0..8]);
                found_val = fea2scalar(&fea);

                // We construct the whole key of that value in the database, and we call it foundKey
                self.join_key(&acc_key, &found_r_key, &mut found_key);
                b_found_key = true;
            }
            // If this is an intermediate node
            else {
                // Take either the first 4 (keys[level]=0) or the second 4 (keys[level]=1) siblings as the hash of the next level
                let idx = (keys[level as usize] * 4) as usize;
                r.copy_from_slice(&siblings[&level][idx..(idx + 4)]);

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
            if Self::node_is_eq(key, &found_key) {
                value = found_val;
            }
            // if foundKey!=key, then the requested value was not found
            else {
                ins_key.copy_from_slice(&found_key);
                ins_value = found_val;
                is_old0 = false;
            }
        }

        // We leave the siblings only up to the leaf node level, deleting items where key equal or
        // are bigger than level + 1, e.g. siblings = siblings.slice(0, level + 1);
        siblings.retain(|k, _| *k <= level);

        let mut ret = SmtGetResult {
            root: *root,
            key: *key,
            value: value.clone(),
            ins_key,
            ins_value,
            is_old0,
            proof_hash_counter: 0,
            siblings: siblings.clone(),
        };

        if !Self::node_is_zero(root) {
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
    fn node_is_zero(r: &[Fr; 4]) -> bool {
        Fr::is_zero(&r[0]) && Fr::is_zero(&r[1]) && Fr::is_zero(&r[2]) && Fr::is_zero(&r[3])
    }

    #[inline(always)]
    fn node_is_eq(r: &[Fr; 4], r2: &[Fr; 4]) -> bool {
        Fr::eq(&r[0], &r2[0])
            && Fr::eq(&r[1], &r2[1])
            && Fr::eq(&r[2], &r2[2])
            && Fr::eq(&r[3], &r2[3])
    }

    fn split_key(&mut self, key: &[Fr; 4]) -> Vec<u64> {
        let mut ru = [0u64; 4];
        for i in 0..4 {
            ru[i] = key[i].as_int();
        }
        log::debug!("key [u64]: {:?}", ru);
        // Split the key in bits, taking one bit from a different scalar every time
        let mut result = vec![];
        for _i in 0..64 {
            for ruj in &mut ru {
                let aux = *ruj & 1;
                result.push(aux);
                *ruj >>= 1;
            }
        }
        result
    }

    fn join_key(&mut self, bits: &[u64], rkey: &[Fr; 4], auxk: &mut [Fr; 4]) {
        let mut n = [0u64, 0, 0, 0];
        let mut accs = [0u64, 0, 0, 0];
        for i in 0..bits.len() {
            if bits[i] > 0 {
                accs[i % 4] |= 1u64 << n[i % 4]
            }
            n[i % 4] += 1;
        }
        auxk[..4].copy_from_slice(&rkey[..4]);
        for i in 0..4 {
            let mut aux = BigUint::from(auxk[i].as_int());
            aux = (aux << n[i]) | BigUint::from(accs[i]);
            log::debug!("aux: {}", aux.to_string());

            let mut str_aux = aux.to_str_radix(16);
            if str_aux.len() % 2 != 0 {
                str_aux = format!("0{}", str_aux);
            }
            auxk[i] = from_hex(&str_aux).unwrap();
        }
    }

    fn get_unique_sibling(a: &[Fr]) -> i32 {
        let mut n_found = 0;
        let mut fnd: i32 = 0;
        for i in (0..a.len()).step_by(4) {
            if !Self::node_is_zero(&a[i..(i + 4)].try_into().unwrap()) {
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
        for sr in state_root.iter().take(4) {
            db_value.push(*sr);
        }
        db_value.append(&mut [Fr::ZERO; 8].to_vec());
        self.db
            .write(&self.db.db_state_root_key.to_string(), &db_value, true)
    }

    fn hash_save(&mut self, a: &[Fr; 8], c: &[Fr; 4]) -> Result<[Fr; 4]> {
        let mut db_value = [Fr::ZERO; 12];
        db_value[..8].copy_from_slice(a);
        db_value[8..].copy_from_slice(c);

        let p = LinearHash::new();
        let digest = p.hash(&db_value, 0).unwrap();
        let digest = digest.as_elements();
        log::debug!("hash_save: {:?} => {:?}", db_value, digest);

        let str_digest = h4_to_string(digest.try_into().unwrap());

        self.db.write(&str_digest, &db_value.to_vec(), true)?;
        Ok([digest[0], digest[1], digest[2], digest[3]])
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
            auxk[i as usize] >>= n;
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
    use num_traits::Num;
    use utils::*;

    fn setup() -> SMT {
        // export DATABASE_URL="postgresql://root:password@127.0.0.1:5432/state"
        env_logger::try_init().unwrap_or_default();
        let db = Database::new(None);
        SMT::new(db)
    }

    #[test]
    #[ignore]
    fn test_smt_join_and_split_key() {
        let mut smt = setup();
        let key = BigUint::from_str_radix(
            "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000",
            16,
        )
        .unwrap(); // bn254::prime - 1
        log::debug!("key string {:?}", key);
        let h4key = scalar_to_h4(&key);
        log::debug!("key {:?}", h4key);
        let result = smt.split_key(&h4key);
        log::debug!("result {:?}", result);

        let mut kea_r = [Fr::ZERO; 4];
        let t = [Fr::ZERO; 4];
        smt.join_key(&result, &t, &mut kea_r);
        let key_r_str = h4_to_scalar(&kea_r);
        assert_eq!(key, key_r_str);
    }

    #[test]
    #[ignore]
    fn test_join_key_overflow() {
        let mut smt = setup();

        let rkey = [
            Fr::from(0xf3efdf4a91164c36),
            Fr::from(0xcacc86e67aa7ad4d),
            Fr::from(0x772e5272aa6c70be),
            Fr::from(0xe6f4433edeb5f8be),
        ];
        let bits = vec![0, 1, 0, 0, 1, 0];
        let mut _key = [Fr::ZERO; 4];
        smt.join_key(&bits, &rkey, &mut _key);
    }

    #[test]
    #[ignore]
    fn test_add_and_remove() {
        let mut smt = setup();
        let sca = scalar_to_h4(&BigUint::from(123u64));
        let val = BigUint::from(123u64);
        let r1 = smt.set(&SMT::EMPTY, &sca, val.clone(), true).unwrap();
        let r_get = smt.get(&r1.new_root, &sca).unwrap();
        assert_eq!(val, r_get.value);
        let r0 = smt
            .set(&SMT::EMPTY, &sca, BigUint::from(0u64), true)
            .unwrap();
        assert!(SMT::node_is_zero(&r0.new_root));
    }

    #[test]
    #[ignore]
    fn test_mult_update() {
        let mut smt = setup();
        let sca = scalar_to_h4(&BigUint::from(123u64));
        let val = BigUint::from(123u64);
        let val2 = BigUint::from(1234u64);
        let r1 = smt.set(&SMT::EMPTY, &sca, val.clone(), true).unwrap();
        let r2 = smt.set(&r1.new_root, &sca, val2, true).unwrap();
        let r3 = smt.set(&r2.new_root, &sca, val, true).unwrap();
        assert!(SMT::node_is_eq(&r1.new_root, &r3.new_root));
    }

    #[test]
    #[ignore]
    fn test_shared_element_2() {
        let mut smt = setup();
        let sca = scalar_to_h4(&BigUint::from(7u64));
        let val = BigUint::from(2u64);
        let r1 = smt.set(&SMT::EMPTY, &sca, val, true).unwrap();

        let sca2 = scalar_to_h4(&BigUint::from(15u64));
        let val2 = BigUint::from(3u64);
        let r2 = smt.set(&r1.new_root, &sca2, val2, true).unwrap();

        let r3 = smt
            .set(&r2.new_root, &sca, BigUint::from(0u64), true)
            .unwrap();
        let r4 = smt
            .set(&r3.new_root, &sca2, BigUint::from(0u64), true)
            .unwrap();
        assert!(SMT::node_is_zero(&r4.new_root));
    }

    #[test]
    #[ignore]
    fn test_shared_element_3() {
        let mut smt = setup();
        let sca = scalar_to_h4(&BigUint::from(7u64));
        let val = BigUint::from(123u64);
        let r1 = smt.set(&SMT::EMPTY, &sca, val, true).unwrap();

        let sca2 = scalar_to_h4(&BigUint::from(15u64));
        let val2 = BigUint::from(1235u64);
        let r2 = smt.set(&r1.new_root, &sca2, val2, true).unwrap();

        let sca3 = scalar_to_h4(&BigUint::from(9u64));
        let val3 = BigUint::from(1236u64);
        let r3 = smt.set(&r2.new_root, &sca3, val3, true).unwrap();

        let r4 = smt
            .set(&r3.new_root, &sca, BigUint::from(0u64), true)
            .unwrap();
        let r5 = smt
            .set(&r4.new_root, &sca2, BigUint::from(0u64), true)
            .unwrap();
        let r6 = smt
            .set(&r5.new_root, &sca3, BigUint::from(0u64), true)
            .unwrap();
        assert!(SMT::node_is_zero(&r6.new_root));
    }

    #[test]
    #[ignore]
    fn test_add_and_remove_n() {
        let mut smt = setup();
        let n = 128;
        // dummy result
        let mut r = SmtSetResult {
            new_root: SMT::EMPTY,
            siblings: HashMap::new(),
            old_root: SMT::EMPTY,
            key: SMT::EMPTY,
            ins_value: BigUint::from(0u64),
            ins_key: SMT::EMPTY,
            is_old0: false,
            old_value: BigUint::from(0u64),
            new_value: BigUint::from(0u64),
            mode: "".to_string(),
            proof_hash_counter: 0,
        };
        let sca = scalar_to_h4(&BigUint::from(123u64));
        let val = BigUint::from(123u64);
        for _i in 0..n {
            r = smt.set(&r.new_root, &sca, val.clone(), true).unwrap();
        }

        for _i in 0..n {
            r = smt
                .set(&r.new_root, &sca, BigUint::from(0u64), true)
                .unwrap();
        }
        assert!(SMT::node_is_zero(&r.new_root));
    }

    #[test]
    #[ignore]
    fn test_smt_set_and_get() {
        let mut smt = setup();

        let old_root = [Fr::ZERO; 4];
        let key = [Fr::ONE; 4];
        let value = BigUint::from(12u64);
        // insert not found
        let sr = smt.set(&old_root, &key, value, true);
        log::debug!("insert not found: sr: {:?}", sr);
        assert!(sr.is_ok());

        let sr = sr.unwrap();

        let mut key = [Fr::ONE; 4];
        key[0] = Fr::from(10);
        let value = BigUint::from(13u64);
        // insert found
        let sr = smt.set(&sr.new_root, &key, value, true);
        log::debug!("insert found, sr: {:?}", sr);
        assert!(sr.is_ok());

        let sr = sr.unwrap();

        // delete not found
        let mut key = [Fr::ONE; 4];
        key[0] = Fr::from(11);
        let value = BigUint::from(0u64);
        let sr = smt.set(&sr.new_root, &key, value, true);
        assert!(sr.is_ok());
        log::debug!("delete not found, sr: {:?}", sr);
        let sr = sr.unwrap();

        // get found
        let mut key = [Fr::ONE; 4];
        key[0] = Fr::from(10);
        let gr = smt.get(&sr.new_root, &key);
        assert!(gr.is_ok());
        let gr = gr.unwrap();
        let value = BigUint::from(13u64);
        log::debug!("get found, gr: {:?}", gr);
        assert_eq!(gr.value, value);
        assert!(gr.is_old0);

        // delete found
        let mut key = [Fr::ONE; 4];
        key[0] = Fr::from(10);
        let value = BigUint::from(0u64);
        let sr = smt.set(&sr.new_root, &key, value, true);
        assert!(sr.is_ok());
        log::debug!("delete found, sr: {:?}", sr);
        let sr = sr.unwrap();

        // get found
        let key = [Fr::ONE; 4];
        let gr = smt.get(&sr.new_root, &key);
        assert!(gr.is_ok());
        let gr = gr.unwrap();
        let value = BigUint::from(12u64);
        log::debug!("get found, gr: {:?}", gr);
        assert_eq!(gr.value, value);
        assert!(gr.is_old0);

        // get not found
        let mut key = [Fr::ONE; 4];
        key[0] = Fr::from(10);
        let gr = smt.get(&sr.new_root, &key);
        assert!(gr.is_ok());
        let gr = gr.unwrap();
        log::debug!("get not found, gr: {:?}", gr);
        assert!(!gr.is_old0);
    }
}
