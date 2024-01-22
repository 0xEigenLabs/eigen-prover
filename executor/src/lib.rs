#![allow(clippy::redundant_closure)]
use alloc::collections::BTreeMap;
use anyhow::Result;
use ethers_core::types::BlockId;
use ethers_providers::Middleware;
use ethers_providers::{Http, Provider};
use revm::primitives::HashSet;
use revm::{
    db::{CacheDB, EmptyDB, EthersDB},
    //interpreter::gas::ZERO,
    primitives::{
        Address, Bytes, Env, FixedBytes, HashMap, ResultAndState, SpecId, TransactTo, B256, U256,
    },
    Database,
    DatabaseCommit,
};
use ruint::uint;
//use models::*;
use ruint::Uint;
use std::sync::Arc;
extern crate alloc;

use alloc::vec::Vec;

type ExecResult = Result<Vec<(Vec<u8>, Bytes, Uint<256, 4>, ResultAndState)>>;

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

pub async fn execute_one(block_number: u64, chain_id: u64, slot_path: &str) -> ExecResult {
    let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let client = Arc::new(client);
    let block = match client.get_block_with_txs(block_number).await {
        Ok(Some(block)) => block,
        Ok(None) => panic!("Block not found"),
        Err(error) => panic!("Error: {:?}", error),
    };

    println!("Fetched block number: {:?}", block.number.unwrap());
    let previous_block_number = block_number - 1;

    let prev_id: BlockId = previous_block_number.into();
    // SAFETY: This cannot fail since this is in the top-level tokio runtime
    let mut ethersdb = EthersDB::new(Arc::clone(&client), Some(prev_id)).unwrap();

    let mut cache_db = CacheDB::new(EmptyDB::default());

    let mut test_pre = HashMap::new();
    for tx in &block.transactions {
        let from_acc = Address::from(tx.from.as_fixed_bytes());
        // query basic properties of an account incl bytecode
        let acc_info = ethersdb.basic(from_acc).unwrap().unwrap();
        println!("acc_info: {} => {:?}", from_acc, acc_info);
        let account_info = models::AccountInfo {
            balance: acc_info.balance,
            code: acc_info.code.clone().unwrap().bytecode,
            nonce: acc_info.nonce,
            // TODO: fill storage
            storage: HashMap::new(),
        };
        test_pre.insert(from_acc, account_info);
        cache_db.insert_account_info(from_acc, acc_info);

        if tx.to.is_some() {
            let to_acc = Address::from(tx.to.unwrap().as_fixed_bytes());
            let acc_info = ethersdb.basic(to_acc).unwrap().unwrap();
            println!("to_info: {} => {:?}", to_acc, acc_info);
            // setup storage

            uint! {
                let account_slot_path = format!("{}/{}.json", slot_path, to_acc);
                let account_slot_json = std::fs::read_to_string(account_slot_path).unwrap_or_default();
                let account_slot: HashSet<Uint<256,4>>= serde_json::from_str(&account_slot_json).unwrap_or_default();
                for slot in account_slot {
                    let slot = U256::from(slot);
                    if !acc_info.code.as_ref().unwrap().is_empty() {
                        // query value of storage slot at account address
                        let value = ethersdb.storage(to_acc, slot).unwrap();
                        println!("slot:{}, value: {:?}", slot, value);

                        cache_db
                            .insert_account_storage(to_acc, slot, value)
                            .unwrap();
                    }
                }
            }

            cache_db.insert_account_info(to_acc, acc_info);
        }
    }

    let mut env = Env::default();
    if let Some(number) = block.number {
        let nn = number.0[0];
        env.block.number = U256::from(nn);
    }
    local_fill!(env.block.coinbase, block.author);
    local_fill!(env.block.timestamp, Some(block.timestamp), U256::from_limbs);
    local_fill!(
        env.block.difficulty,
        Some(block.difficulty),
        U256::from_limbs
    );
    local_fill!(env.block.gas_limit, Some(block.gas_limit), U256::from_limbs);
    if let Some(base_fee) = block.base_fee_per_gas {
        local_fill!(env.block.basefee, Some(base_fee), U256::from_limbs);
    }

    let txs = block.transactions.len();
    println!("Found {txs} transactions.");

    let _elapsed = std::time::Duration::ZERO;

    // Create the traces directory if it doesn't exist
    std::fs::create_dir_all("traces").expect("Failed to create traces directory");

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
    env.cfg.chain_id = chain_id;
    let mut all_result = vec![];
    for tx in block.transactions {
        env.tx.caller = Address::from(tx.from.as_fixed_bytes());
        env.tx.gas_limit = tx.gas.as_u64();
        local_fill!(env.tx.gas_price, tx.gas_price, U256::from_limbs);
        local_fill!(env.tx.value, Some(tx.value), U256::from_limbs);
        env.tx.data = tx.input.0.clone().into();

        let mut gas_priority_fee = U256::ZERO;
        local_fill!(
            gas_priority_fee,
            tx.max_priority_fee_per_gas,
            U256::from_limbs
        );
        env.tx.gas_priority_fee = Some(gas_priority_fee);
        env.tx.chain_id = Some(chain_id);
        env.tx.nonce = Some(tx.nonce.as_u64());
        if let Some(access_list) = tx.access_list.clone() {
            env.tx.access_list = access_list
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
            env.tx.access_list = Default::default();
        }

        env.tx.transact_to = match tx.to {
            Some(to_address) => TransactTo::Call(Address::from(to_address.as_fixed_bytes())),
            None => TransactTo::create(),
        };

        let mut evm = revm::Evm::builder()
            .with_db(&mut cache_db)
            .modify_env(|e| *e = env.clone())
            .spec_id(SpecId::FRONTIER)
            .build();

        let mut gas_limit_uint = Uint::ZERO;
        local_fill!(gas_limit_uint, Some(block.gas_limit), U256::from_limbs);
        let tx_data = tx.input.0.clone();
        transaction_parts.data.push(tx_data.into());
        transaction_parts.gas_limit.push(gas_limit_uint);
        transaction_parts.gas_price = Some(env.tx.gas_price);
        transaction_parts.nonce = U256::from(tx.nonce.as_u64());
        transaction_parts.secret_key = B256::default();
        transaction_parts.sender = Some(Address::from(tx.from.as_fixed_bytes()));
        transaction_parts.to = tx
            .to
            .map(|to_address| Address::from(to_address.as_fixed_bytes()));
        transaction_parts.value.push(env.tx.value);
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
        evm.context.evm.db.commit(result.state.clone());
        let txbytes = serde_json::to_vec(&env.tx).unwrap();
        all_result.push((txbytes, env.tx.data, env.tx.value, result));

        for (k, v) in &evm.context.evm.db.accounts {
            println!("state: {}=>{:?}", k, v);
            let account_slot_path = format!("{}/{}.json", slot_path, k);
            let account_slot_json = std::fs::read_to_string(&account_slot_path).unwrap_or_default();
            let mut account_slot: HashSet<Uint<256, 4>> =
                serde_json::from_str(&account_slot_json).unwrap_or_default();
            if !v.storage.is_empty() {
                for (k, v) in v.storage.iter() {
                    println!("slot => storage: {}=>{}", k, v);
                    account_slot.insert(*k);
                }
            }
            let new_account_slot_json =
                serde_json::to_string(&account_slot).expect("Failed to serialize");
            std::fs::write(format!("{}/{}.json", slot_path, k), new_account_slot_json)
                .unwrap_or_else(|_| panic!("Failed to write to file, slot_path: {}", slot_path))
        }
    }

    let mut test_post = BTreeMap::new();
    for (idx, res) in all_result.iter().enumerate() {
        let (txbytes, data, value, ResultAndState { result, state }) = res;
        {
            // 1. expect_exception: Option<String>,
            println!("expect_exception: {:?}", result.is_success());
            // indexes: TxPartIndices,
            println!(
                "indexes: data{:?}, value: {}, gas: {}",
                data,
                value,
                result.gas_used()
            );
            println!("output: {:?}", result.output());

            // TODO: hash: B256, // post state root
            //let hash = serde_json::to_vec(&state).unwrap();
            //println!("hash: {:?}", state);
            // post_state: HashMap<Address, AccountInfo>,
            println!("post_state: {:?}", state);
            // logs: B256,
            println!("logs: {:?}", result.logs());
            // txbytes: Option<Bytes>,
            println!("txbytes: {:?}", txbytes);

            let mut new_state: HashMap<Address, models::AccountInfo> = HashMap::new();

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
                    // TODO: fill storage
                    storage: HashMap::new(),
                };

                new_state.insert(*address, account_info);
            }

            let post_value = test_post
                .entry(models::SpecName::Shanghai)
                .or_insert_with(|| Vec::new());
            let mut new_post_value = std::mem::take(post_value);
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
                // TODO: fill hash
                hash: FixedBytes::default(),
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
    std::fs::write("output.json", json_string).expect("Failed to write to file");

    Ok(all_result)
}
