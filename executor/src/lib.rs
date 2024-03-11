#![allow(clippy::redundant_closure)]
use anyhow::Result;
use ethers_core::types::BlockId;
use ethers_providers::{Http, Middleware, Provider};
use powdr_number::FieldElement;
use revm::primitives::HashSet;
use revm::{
    db::{CacheDB, EmptyDB, EthersDB, PlainAccount},
    inspector_handle_register,
    inspectors::TracerEip3155,
    primitives::{
        Address, Bytes, FixedBytes, HashMap, ResultAndState, Storage, TransactTo, B256, U256,
    },
    Database, DatabaseCommit, Evm,
};
use ruint::{uint, Uint};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io::Write};
use zkvm::zkvm_evm_generate_chunks;

use statedb::database::Database as StateDB;

type ExecResult = Result<Vec<(Vec<u8>, Bytes, Uint<256, 4>, ResultAndState)>>;
mod merkle_trie;
use merkle_trie::state_merkle_trie_root;

macro_rules! local_fill {
    ($left:expr, $right:expr, $fun:expr) => {
        if let Some(right) = $right {
            $left = $fun(right.0)
        }
    };
    ($left:expr, $right:expr) => {
        if let Some(right) = $right {
            $left = Address::from(right.as_fixed_bytes())
        }
    };
}

fn new_storage(storage: &Storage) -> HashMap<U256, U256> {
    storage.iter().map(|(k, v)| (*k, v.present_value)).collect()
}

pub async fn batch_process(
    client: Arc<Provider<Http>>,
    block_number: u64,
    chain_id: u64,
    task: &str,
    task_id: &str,
    base_dir: &str,
) -> (ExecResult, usize) {
    //let client = Provider::<Http>::try_from(url).unwrap();
    //let client = Arc::new(client);
    let block = match client.get_block_with_txs(block_number).await {
        Ok(Some(block)) => block,
        Ok(None) => panic!("Block not found"),
        Err(error) => panic!("Error: {:?}", error),
    };

    log::info!("Fetched block number: {:?}", block.number.unwrap());
    let previous_block_number = block_number - 1;

    let prev_id: BlockId = previous_block_number.into();
    // SAFETY: This cannot fail since this is in the top-level tokio runtime
    let mut ethersdb = EthersDB::new(Arc::clone(&client), Some(prev_id)).unwrap();

    let mut cache_db = CacheDB::new(EmptyDB::default());

    let mut db = StateDB::new(None);

    let mut test_pre = HashMap::new();
    for tx in &block.transactions {
        let from_acc = Address::from(tx.from.as_fixed_bytes());
        // query basic properties of an account incl bytecode
        let acc_info = ethersdb.basic(from_acc).unwrap().unwrap();
        log::info!("acc_info: {} => {:?}", from_acc, acc_info);
        let account_info = models::AccountInfo {
            balance: acc_info.balance,
            code: acc_info.code.clone().unwrap().bytecode,
            nonce: acc_info.nonce,
            // FIXME: fill in the storage
            storage: HashMap::new(),
        };
        test_pre.insert(from_acc, account_info);
        cache_db.insert_account_info(from_acc, acc_info);

        if tx.to.is_some() {
            let to_acc = Address::from(tx.to.unwrap().as_fixed_bytes());
            let acc_info = ethersdb.basic(to_acc).unwrap().unwrap();
            log::info!("to_info: {} => {:?}", to_acc, acc_info);
            // setup storage

            uint! {
                let account_slot_json = db.read_nodes(to_acc.to_string().as_str()).unwrap_or_default();
                let account_slot_json_str = account_slot_json.as_str();
                if !account_slot_json_str.is_empty() {
                    log::info!("not found slot in db, account_slot_json: {:?}", account_slot_json_str);
                }
                let account_slot: HashSet<Uint<256,4>>= serde_json::from_str(account_slot_json_str).unwrap_or_default();
                for slot in account_slot {
                    let slot = U256::from(slot);
                    if !acc_info.code.as_ref().unwrap().is_empty() {
                        // query value of storage slot at account address
                        let value = ethersdb.storage(to_acc, slot).unwrap();
                        log::info!("slot:{}, value: {:?}", slot, value);

                        cache_db
                            .insert_account_storage(to_acc, slot, value)
                            .unwrap();
                    }
                }
            }
            cache_db.insert_account_info(to_acc, acc_info);
        }
    }

    let mut evm = Evm::builder()
        .with_db(&mut cache_db)
        .with_external_context(TracerEip3155::new(Box::new(std::io::stdout()), true, true))
        .modify_block_env(|b| {
            if let Some(number) = block.number {
                let nn = number.0[0];
                b.number = U256::from(nn);
            }
            local_fill!(b.coinbase, block.author);
            local_fill!(b.timestamp, Some(block.timestamp), U256::from_limbs);
            local_fill!(b.difficulty, Some(block.difficulty), U256::from_limbs);
            local_fill!(b.gas_limit, Some(block.gas_limit), U256::from_limbs);
            if let Some(base_fee) = block.base_fee_per_gas {
                local_fill!(b.basefee, Some(base_fee), U256::from_limbs);
            }
        })
        .modify_cfg_env(|c| {
            c.chain_id = chain_id;
        })
        .append_handler_register(inspector_handle_register)
        .build();

    let txs = block.transactions.len();
    log::info!("Found {txs} transactions.");

    let _elapsed = std::time::Duration::ZERO;

    // Create the traces directory if it doesn't exist
    std::fs::create_dir_all("traces").unwrap_or_else(|_| panic!("Failed to create trace file"));

    let mut transaction_parts = models::TransactionParts {
        data: vec![],
        gas_limit: vec![],
        gas_price: None,
        nonce: U256::default(),
        secret_key: B256::default(),
        sender: Some(Address::default()),
        to: None,
        value: vec![],
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        access_lists: vec![],
        blob_versioned_hashes: vec![],
        max_fee_per_blob_gas: None,
    };

    // Fill in CfgEnv
    let mut all_result = vec![];
    for tx in block.transactions {
        evm = evm
            .modify()
            .modify_tx_env(|etx| {
                etx.caller = Address::from(tx.from.as_fixed_bytes());
                etx.gas_limit = tx.gas.as_u64();
                local_fill!(etx.gas_price, tx.gas_price, U256::from_limbs);
                local_fill!(etx.value, Some(tx.value), U256::from_limbs);
                etx.data = tx.input.0.clone().into();
                let mut gas_priority_fee = U256::ZERO;
                local_fill!(
                    gas_priority_fee,
                    tx.max_priority_fee_per_gas,
                    U256::from_limbs
                );
                etx.gas_priority_fee = Some(gas_priority_fee);
                etx.chain_id = Some(chain_id);
                etx.nonce = Some(tx.nonce.as_u64());
                if let Some(access_list) = tx.access_list.clone() {
                    etx.access_list = access_list
                        .0
                        .into_iter()
                        .map(|item| {
                            let new_keys: Vec<U256> = item
                                .storage_keys
                                .into_iter()
                                .map(|h256| U256::from_le_bytes(h256.0))
                                .collect();
                            (Address::from(item.address.as_fixed_bytes()), new_keys)
                        })
                        .collect();
                } else {
                    etx.access_list = Default::default();
                }

                etx.transact_to = match tx.to {
                    Some(to_address) => {
                        TransactTo::Call(Address::from(to_address.as_fixed_bytes()))
                    }
                    None => TransactTo::create(),
                };
            })
            .build();

        let mut gas_limit_uint = Uint::ZERO;
        local_fill!(gas_limit_uint, Some(block.gas_limit), U256::from_limbs);
        let tx_data = tx.input.0.clone();
        transaction_parts.data.push(tx_data.into());
        transaction_parts.gas_limit.push(gas_limit_uint);

        let mut tx_gas_price = Uint::ZERO;
        local_fill!(tx_gas_price, tx.gas_price, U256::from_limbs);
        transaction_parts.gas_price = Some(tx_gas_price);
        transaction_parts.nonce = U256::from(tx.nonce.as_u64());
        transaction_parts.secret_key = B256::default();
        transaction_parts.sender = Some(Address::from(tx.from.as_fixed_bytes()));
        transaction_parts.to = tx
            .to
            .map(|to_address| Address::from(to_address.as_fixed_bytes()));

        let mut tx_value = Uint::ZERO;
        local_fill!(tx_value, Some(tx.value), U256::from_limbs);
        transaction_parts.value.push(tx_value);
        transaction_parts.max_fee_per_gas = Some(U256::from(tx.max_fee_per_gas.unwrap().as_u64()));
        transaction_parts.max_priority_fee_per_gas =
            Some(U256::from(tx.max_priority_fee_per_gas.unwrap().as_u64()));

        let access_list_vec = tx.access_list.as_ref().map(|access_list| {
            access_list
                .0
                .iter()
                .map(|item| models::AccessListItem {
                    address: Address::from(item.address.as_fixed_bytes()),
                    storage_keys: item
                        .storage_keys
                        .iter()
                        .map(|h256| B256::from(h256.to_fixed_bytes()))
                        .collect(),
                })
                .collect()
        });

        transaction_parts.access_lists.push(access_list_vec);
        /*
        // Construct the file writer to write the trace to
        let tx_number = tx.transaction_index.unwrap().0[0];
        let file_name = format!("traces/{}.json", tx_number);
        let write = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name);
        let inner = Arc::new(Mutex::new(BufWriter::new(
            write.expect("Failed to open file"),
        )));
        let writer = FlushWriter::new(Arc::clone(&inner));

        // Inspect and commit the transaction to the EVM
        let inspector = TracerEip3155::new(Box::new(writer), true, true);
        if let Err(error) = evm.inspect_commit(inspector) {
            println!("Got error: {:?}", error);
        }

        // Flush the file writer
        inner.lock().unwrap().flush().expect("Failed to flush file");
        */
        //evm.transact_commit().unwrap();
        let result = evm.transact().unwrap();
        log::info!("evm transact result: {:?}", result.result);
        evm.context.evm.db.commit(result.state.clone());
        let env = evm.context.evm.env.clone();
        let txbytes = serde_json::to_vec(&env.tx).unwrap();
        all_result.push((txbytes, env.tx.data, env.tx.value, result));

        for (k, v) in &evm.context.evm.db.accounts {
            log::info!("state: {}=>{:?}", k, v);
            let account_slot_json = db.read_nodes(k.to_string().as_str()).unwrap_or_default();
            let account_slot_json_str = account_slot_json.as_str();
            let mut account_slot: HashSet<Uint<256, 4>> =
                serde_json::from_str(account_slot_json_str).unwrap_or_default();
            if !v.storage.is_empty() {
                for (k, v) in v.storage.iter() {
                    log::info!("slot => storage: {}=>{}", k, v);
                    account_slot.insert(*k);
                }
            }
            let new_account_slot_json =
                serde_json::to_string(&account_slot).expect("Failed to serialize");

            let write_res =
                db.write_nodes(k.to_string().as_str(), new_account_slot_json.as_str(), true);
            if write_res.is_err() {
                log::error!("Failed to write nodes: {:?}", write_res);
            }
        }
    }

    let mut test_post = BTreeMap::new();
    for (idx, res) in all_result.iter().enumerate() {
        let (txbytes, data, value, ResultAndState { result, state }) = res;
        {
            // 1. expect_exception: Option<String>,
            log::info!("expect_exception: {:?}", result.is_success());
            // indexes: TxPartIndices,
            log::info!(
                "indexes: data{:?}, value: {}, gas: {}",
                data,
                value,
                result.gas_used()
            );
            log::info!("output: {:?}", result.output());

            // post_state: HashMap<Address, AccountInfo>,
            log::info!("post_state: {:?}", state);
            // logs: B256,
            log::info!("logs: {:?}", result.logs());
            // txbytes: Option<Bytes>,
            log::info!("txbytes: {:?}", txbytes);

            let mut new_state: HashMap<Address, models::AccountInfo> = HashMap::new();

            let mut plain_accounts = vec![];
            for (address, account) in state.iter() {
                let account_info = models::AccountInfo {
                    balance: account.info.balance,
                    code: account
                        .info
                        .code
                        .clone()
                        .map(|code| code.bytecode)
                        .unwrap_or_default(),
                    nonce: account.info.nonce,
                    storage: new_storage(&account.storage),
                };

                new_state.insert(*address, account_info);
                plain_accounts.push((
                    *address,
                    PlainAccount {
                        info: account.info.clone(),
                        storage: new_storage(&account.storage),
                    },
                ));
            }

            let post_value = test_post
                .entry(models::SpecName::Shanghai)
                .or_insert_with(|| Vec::new());
            let mut new_post_value = std::mem::take(post_value);

            let state_root = state_merkle_trie_root(plain_accounts);
            new_post_value.push(models::Test {
                expect_exception: None,
                indexes: models::TxPartIndices {
                    data: idx,
                    gas: idx,
                    value: idx,
                },
                post_state: new_state,
                // TODO: fill logs
                logs: FixedBytes::default(),
                txbytes: Some(Bytes::from_iter(txbytes)),
                hash: state_root,
            });

            test_post.insert(
                // TODO: get specID
                models::SpecName::Shanghai,
                new_post_value,
            );
        }
    }

    let mut test_env = models::Env {
        current_coinbase: Address(block.author.map(|h160| FixedBytes(h160.0)).unwrap()),
        current_difficulty: U256::default(),
        current_gas_limit: U256::default(),
        current_number: U256::default(),
        current_timestamp: U256::default(),
        current_base_fee: Some(U256::default()),
        previous_hash: B256::default(),

        current_random: Some(B256::default()),
        current_beacon_root: Some(B256::default()),
        current_withdrawals_root: Some(B256::default()),

        parent_blob_gas_used: Some(U256::default()),
        parent_excess_blob_gas: Some(U256::default()),
    };
    test_env.current_coinbase = Address(block.author.map(|h160| FixedBytes(h160.0)).unwrap());
    local_fill!(
        test_env.current_difficulty,
        Some(block.difficulty),
        U256::from_limbs
    );
    local_fill!(
        test_env.current_gas_limit,
        Some(block.gas_limit),
        U256::from_limbs
    );
    if let Some(number) = block.number {
        let nn = number.0[0];
        test_env.current_number = U256::from(nn);
    }
    local_fill!(
        test_env.current_timestamp,
        Some(block.timestamp),
        U256::from_limbs
    );
    let mut base_fee = Uint::ZERO;
    local_fill!(base_fee, block.base_fee_per_gas, U256::from_limbs);
    test_env.current_base_fee = Some(base_fee);
    test_env.previous_hash = FixedBytes(block.parent_hash.0);
    // local_fill!(test_env.current_random, block.random);
    // local_fill!(test_env.current_beacon_root, block.beacon_root);
    test_env.current_withdrawals_root = Some(FixedBytes(block.withdrawals_root.unwrap().0));

    let mut gas_used = Uint::ZERO;
    local_fill!(gas_used, Some(block.gas_used), U256::from_limbs);
    test_env.parent_blob_gas_used = Some(gas_used);
    test_env.parent_excess_blob_gas = Some(gas_used);

    let test_unit = models::TestUnit {
        info: None,
        env: test_env,
        // pre: HashMap<Address, AccountInfo, BuildHasherDefault<AHasher>, Global>
        pre: test_pre,
        // post: BTreeMap<SpecName, Vec<Test, Global>, Global>
        post: test_post,
        chain_id: Some(chain_id),
        transaction: transaction_parts,
        out: None,
    };

    // println!("test_unit: {:#?}", test_unit);
    let json_string = serde_json::to_string(&test_unit).expect("Failed to serialize");
    let suite_json = json_string.clone();

    let output_path = format!("{}/{}/{}", base_dir, task_id, task);
    log::info!("output_path: {}", output_path);
    std::fs::create_dir_all(output_path.clone())
        .unwrap_or_else(|_| panic!("Failed to write to file, output_path: {}", output_path));
    std::fs::write(format!("{}/batch.json", output_path), json_string)
        .expect("Failed to write to file");

    let project_root_path = project_root::get_project_root()
        .unwrap_or_else(|_| panic!("Failed to get project root path"));
    let workspace = format!(
        "{}/executor/vm/{}",
        project_root_path.to_str().unwrap(),
        task
    );
    log::info!("workspace: {}", workspace);
    let bootloader_inputs =
        zkvm_evm_generate_chunks(workspace.as_str(), &suite_json, output_path.as_str()).unwrap();
    let cnt_chunks: usize = bootloader_inputs.len();
    log::info!("Generated {} chunks", cnt_chunks);
    // save the chunks
    let bi_files: Vec<_> = (0..cnt_chunks)
        .map(|i| Path::new(output_path.as_str()).join(format!("{task}_chunks_{i}.data")))
        .collect();
    log::info!("bi_files: {:#?}", bi_files);
    bootloader_inputs
        .iter()
        .zip(&bi_files)
        .for_each(|(data, filename)| {
            let mut f = fs::File::create(filename).unwrap();
            for d in data {
                f.write_all(&d.to_bytes_le()[0..8]).unwrap();
            }
        });
    (Ok(all_result), cnt_chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zkvm_evm_generate_chunks() {
        env_logger::try_init().unwrap_or_default();
        //let test_file = "test-vectors/blockInfo.json";
        let test_file = "test-vectors/solidityExample.json";
        let suite_json = fs::read_to_string(test_file).unwrap();
        let task = "evm";
        let task_id = "0";
        let output_path = format!("../prover/data/proof/{}/{}", task_id, task);
        let workspace = format!("vm/{}", task);
        let bootloader_inputs =
            zkvm_evm_generate_chunks(workspace.as_str(), &suite_json, output_path.as_str())
                .unwrap();
        let cnt_chunks: usize = bootloader_inputs.len();
        log::info!("Generated {} chunks", cnt_chunks);
        // save the chunks
        let bi_files: Vec<_> = (0..cnt_chunks)
            .map(|i| Path::new(output_path.as_str()).join(format!("{task}_chunks_{i}.data")))
            .collect();
        log::info!("bi_files: {:#?}", bi_files);
        bootloader_inputs
            .iter()
            .zip(&bi_files)
            .for_each(|(data, filename)| {
                let mut f = fs::File::create(filename).unwrap();
                for d in data {
                    f.write_all(&d.to_bytes_le()[0..8]).unwrap();
                }
            });
    }
}
