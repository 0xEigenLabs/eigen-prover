use revm::{
    db::{CacheDB, EmptyDB, EthersDB},
    //interpreter::gas::ZERO,
    primitives::{
        Address, Bytes, Env, FixedBytes, HashMap, ResultAndState, TransactTo, B256, U256,
    },
    Database,
    DatabaseCommit,
    EVM,
};

use alloc::collections::BTreeMap;
use anyhow::Result;
use ethers_core::types::BlockId;
use ethers_providers::Middleware;
use ethers_providers::{Http, Provider};
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

pub async fn execute_one(block_number: u64, _addr: Address, chain_id: u64) -> ExecResult {
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
    let mut accs_info = vec![];
    for tx in &block.transactions {
        let from_acc = Address::from(tx.from.as_fixed_bytes());
        // query basic properties of an account incl bytecode
        let acc_info = ethersdb.basic(from_acc).unwrap().unwrap();
        println!("acc_info: {} => {:?}", from_acc, acc_info);
        accs_info.push(acc_info.clone());
        test_pre.insert(
            from_acc,
            serde_json::from_str(serde_json::to_string(&acc_info.clone())?.as_str())?,
        );
        cache_db.insert_account_info(from_acc, acc_info);

        if tx.to.is_some() {
            let to_acc = Address::from(tx.to.unwrap().as_fixed_bytes());
            let acc_info = ethersdb.basic(to_acc).unwrap().unwrap();
            println!("to_info: {} => {:?}", to_acc, acc_info);
            // setup storage
            /*
            uint!{
                for slot in [0x2_U256, 0x82440beeb8ea7bdc8fb6c47af3bdfbce49c97853988e80d3269a2ccae791587a_U256] {
                    let slot = U256::from(slot);
                    if acc_info.code.as_ref().unwrap().len() > 0 {
                        // query value of storage slot at account address
                        let value = ethersdb.storage(to_acc, slot).unwrap();
                        println!("slot:{}, value: {:?}", slot, value);

                        cache_db
                            .insert_account_storage(to_acc, slot, value)
                            .unwrap();
                    }
                }
            }
            */
            cache_db.insert_account_info(to_acc, acc_info);
        }
    }
    let mut evm = EVM::new();
    evm.database(cache_db);

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
        sender: Address::default(),
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

        evm.env = env.clone();

        let mut gas_limit_uint = Uint::ZERO;
        local_fill!(gas_limit_uint, Some(block.gas_limit), U256::from_limbs);
        let tx_data = tx.input.0.clone();
        transaction_parts.data.push(tx_data.into());
        transaction_parts.gas_limit.push(gas_limit_uint);
        transaction_parts.gas_price = Some(env.tx.gas_price);
        transaction_parts.nonce = U256::from(tx.nonce.as_u64());
        transaction_parts.secret_key = B256::default();
        transaction_parts.sender = Address::from(tx.from.as_fixed_bytes());
        transaction_parts.to = Some(Address::from(tx.to.unwrap().as_fixed_bytes()));
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
        evm.db().unwrap().commit(result.state.clone());
        let txbytes = serde_json::to_vec(&env.tx).unwrap();
        all_result.push((txbytes, env.tx.data, env.tx.value, result));
    }

    for (k, v) in &evm.db.as_ref().unwrap().accounts {
        println!("state: {}=>{:?}", k, v);
        if !v.storage.is_empty() {
            for (k, v) in v.storage.iter() {
                println!("slot => storage: {}=>{}", k, v);
            }
        }
    }

    let test_post = BTreeMap::new();
    for res in &all_result {
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

            // test_post.insert(
            //     models::SpecName::Shanghai,
            //     vec![models::Test {
            //         expect_exception: Some("err".to_string()),
            //         indexes: models::TxPartIndices {
            //             data: 0,
            //             gas: result.gas_used() as usize,
            //             value: value.as_u64,
            //         },
            //         post_state: state
            //             .into_iter()
            //             .map(|(address, account)| (address, account.info))
            //             .collect(),
            //         logs: result.logs(),
            //         txbytes: txbytes,
            //         hash: todo!(),
            //     }],
            // );
        }
    }

    let mut test_env = models::Env {
        current_coinbase: Address(block.author.map(|h160| FixedBytes(h160.0)).unwrap()),
        current_difficulty: U256::from(10000),
        current_gas_limit: U256::from(5000),
        current_number: U256::from(1),
        current_timestamp: U256::from(1642492800),
        current_base_fee: Some(U256::from(1000)),
        previous_hash: B256::default(),

        current_random: Some(B256::default()),
        current_beacon_root: Some(B256::default()),
        current_withdrawals_root: Some(B256::default()),

        parent_blob_gas_used: Some(U256::from(20000)),
        parent_excess_blob_gas: Some(U256::from(5000)),
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
        info: "sss".into(),
        env: test_env,
        // pre: HashMap<Address, AccountInfo, BuildHasherDefault<AHasher>, Global>
        pre: test_pre,
        // post: BTreeMap<SpecName, Vec<Test, Global>, Global>
        post: test_post,
        transaction: transaction_parts,
    };

    println!("test_unit: {:#?}", test_unit);
    // let json_string = serde_json::to_string(&test_unit).expect("Failed to serialize");
    // std::fs::write("output.json", json_string).expect("Failed to write to file");

    Ok(all_result)
}

#[cfg(test)]
mod tests {
    use super::execute_one;
    use revm::primitives::address;

    //use runtime::{print, get_prover_input, coprocessors::{get_data, get_data_len}};

    //use revm::inspectors::TracerEip3155;

    //use models::*;

    #[tokio::test]
    async fn test_execute_one() {
        // let test_file = "test-vectors/blockInfo.json";
        // let suite_json = std::fs::read_to_string(test_file).unwrap();
        // println!("suite json: {:?}", suite_json);

        let addr = address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b");
        // let t: TestUnit = serde_json::from_str(&suite_json).unwrap();
        // println!("TestUnit t: {:?}", t);

        let num: u64 = 3;
        let res = execute_one(num, addr, 1).await;

        match res {
            Ok(_) => {
                println!("exec sueccess");
            }
            Err(e) => {
                eprintln!("Error occurred: {}", e);
            }
        }
    }
}
