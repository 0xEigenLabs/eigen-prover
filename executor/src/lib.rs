#![allow(clippy::redundant_closure)]
use anyhow::Result;
use ethers_core::types::{
    BlockId, GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions, GethTrace,
    GethTraceFrame, PreStateFrame,
};
use ethers_providers::{Http, Middleware, Provider};
use revm::{
    db::{CacheDB, EthersDB, PlainAccount, StateBuilder},
    inspector_handle_register,
    inspectors::TracerEip3155,
    primitives::{
        Address, Bytes, FixedBytes, HashMap, ResultAndState, Storage, TransactTo, B256, U256,
    },
    DatabaseCommit, Evm,
};
use ruint::Uint;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;
//use std::{fs, io::Write};
//use zkvm::zkvm_generate_chunks;

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

fn core256_to_revm256(core256: ethers_core::types::U256) -> revm::primitives::U256 {
    revm::primitives::U256::from_str_radix(core256.to_string().as_str(), 10).unwrap()
}

fn fill_test_tx(
    transaction_parts: &mut models::TransactionParts,
    tx: &ethers_core::types::Transaction,
    block: &ethers_core::types::Block<ethers_core::types::Transaction>,
) {
    let gas_limit_uint =
        core256_to_revm256(if tx.gas.as_u64() > 0 { tx.gas } else { block.gas_limit });
    log::info!("gas_limit: {:?}", gas_limit_uint);
    transaction_parts.gas_limit.push(gas_limit_uint);

    let tx_data = tx.input.0.clone();
    transaction_parts.data.push(tx_data.into());

    let mut tx_gas_price = Uint::ZERO;
    local_fill!(tx_gas_price, tx.gas_price, U256::from_limbs);
    transaction_parts.gas_price = Some(tx_gas_price);
    transaction_parts.nonce = U256::from(tx.nonce.as_u64());
    transaction_parts.secret_key = B256::default();
    transaction_parts.sender = Some(Address::from(tx.from.as_fixed_bytes()));

    transaction_parts.to = tx.to.map_or_else(
        || Some(Address::default()),
        |to_address| Some(Address::from(to_address.as_fixed_bytes())),
    );

    let mut tx_value = Uint::ZERO;
    local_fill!(tx_value, Some(tx.value), U256::from_limbs);
    transaction_parts.value.push(tx_value);
    transaction_parts.max_fee_per_gas = if tx.max_fee_per_gas.is_some() {
        Some(U256::from(tx.max_fee_per_gas.unwrap().as_u64()))
    } else {
        None
    };
    transaction_parts.max_priority_fee_per_gas = if tx.max_priority_fee_per_gas.is_some() {
        Some(U256::from(tx.max_priority_fee_per_gas.unwrap().as_u64()))
    } else {
        None
    };

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
}

fn fill_test_env(
    block: &ethers_core::types::Block<ethers_core::types::Transaction>,
) -> models::Env {
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
    local_fill!(test_env.current_difficulty, Some(block.difficulty), U256::from_limbs);
    local_fill!(test_env.current_gas_limit, Some(block.gas_limit), U256::from_limbs);
    if let Some(number) = block.number {
        let nn = number.0[0];
        test_env.current_number = U256::from(nn);
    }
    local_fill!(test_env.current_timestamp, Some(block.timestamp), U256::from_limbs);
    let mut base_fee = Uint::ZERO;
    local_fill!(base_fee, block.base_fee_per_gas, U256::from_limbs);
    test_env.current_base_fee = Some(base_fee);
    test_env.previous_hash = FixedBytes(block.parent_hash.0);
    // local_fill!(test_env.current_random, block.random);
    // local_fill!(test_env.current_beacon_root, block.beacon_root);
    test_env.current_withdrawals_root = if block.withdrawals_root.is_some() {
        Some(FixedBytes(block.withdrawals_root.unwrap().0))
    } else {
        None
    };

    let mut gas_used = Uint::ZERO;
    local_fill!(gas_used, Some(block.gas_used), U256::from_limbs);
    test_env.parent_blob_gas_used = Some(gas_used);
    test_env.parent_excess_blob_gas = Some(gas_used);

    test_env
}

/*
fn generate_chunks(task: &str, task_id: &str, base_dir: &str, json_string: &String) -> usize {
    let output_path = format!("{}/{}/{}", base_dir, task_id, task);
    log::debug!("output_path: {}", output_path);

    let project_root_path = project_root::get_project_root()
        .unwrap_or_else(|_| panic!("Failed to get project root path"));
    let workspace = format!("{}/executor/program/{}", project_root_path.to_str().unwrap(), task);
    log::debug!("workspace: {}", workspace);
    let bootloader_inputs =
        zkvm_generate_chunks(workspace.as_str(), json_string, output_path.as_str()).unwrap();
    let cnt_chunks: usize = bootloader_inputs.len();
    log::debug!("Generated {} chunks", cnt_chunks);
    // save the chunks
    let bi_files: Vec<_> = (0..cnt_chunks)
        .map(|i| Path::new(output_path.as_str()).join(format!("{task}_chunks_{i}.data")))
        .collect();
    log::debug!("bi_files: {:#?}", bi_files);
    bootloader_inputs.iter().zip(&bi_files).for_each(|(data, filename)| {
        let mut f = fs::File::create(filename).unwrap();
        // write the start_of_shutdown_routine
        f.write_all(&data.1.to_le_bytes()).unwrap();
        for d in &data.0 {
            f.write_all(&d.to_bytes_le()[0..8]).unwrap();
        }
    });

    cnt_chunks
}
*/

fn fill_test_post(
    all_result: &[(Vec<u8>, Bytes, Uint<256, 4>, ResultAndState)],
) -> BTreeMap<models::SpecName, Vec<models::Test>> {
    let mut test_post: BTreeMap<models::SpecName, Vec<models::Test>> = BTreeMap::new();
    for (idx, res) in all_result.iter().enumerate() {
        let (txbytes, data, value, ResultAndState { result, state }) = res;
        {
            // 1. expect_exception: Option<String>,
            log::debug!("expect_exception: {:?}", result.is_success());
            // indexes: TxPartIndices,
            log::debug!("indexes: data{:?}, value: {}, gas: {}", data, value, result.gas_used());
            log::debug!("output: {:?}", result.output());

            // post_state: HashMap<Address, AccountInfo>,
            log::debug!("post_state: {:?}", state);
            // logs: B256,
            log::debug!("logs: {:?}", result.logs());
            // txbytes: Option<Bytes>,
            log::debug!("txbytes: {:?}", txbytes);

            let mut new_state: HashMap<Address, models::AccountInfo> = HashMap::new();

            let mut plain_accounts = vec![];
            for (address, account) in state.iter() {
                let account_info = models::AccountInfo {
                    balance: account.info.balance,
                    code: account.info.code.clone().map(|code| code.bytecode).unwrap_or_default(),
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

            let post_value =
                test_post.entry(models::SpecName::Shanghai).or_insert_with(|| Vec::new());
            let mut new_post_value = std::mem::take(post_value);

            let state_root = state_merkle_trie_root(plain_accounts);
            new_post_value.push(models::Test {
                expect_exception: None,
                indexes: models::TxPartIndices { data: idx, gas: idx, value: idx },
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
    test_post
}

async fn fill_test_pre(
    // block: &ethers_core::types::Block<ethers_core::types::Transaction>,
    tx: &ethers_core::types::Transaction,
    // state: &mut revm::db::State<CacheDB<EthersDB<Provider<Http>>>>,
    client: &Arc<Provider<Http>>,
) -> HashMap<Address, models::AccountInfo> {
    let mut test_pre: HashMap<Address, models::AccountInfo> = HashMap::new();
    // let from_acc = Address::from(tx.from.as_fixed_bytes());
    // // query basic properties of an account incl bytecode
    // let acc_info: revm::primitives::AccountInfo = state.basic(from_acc).unwrap().unwrap();
    // log::info!("acc_info: {} => {:?}", from_acc, acc_info);

    let trace_options = GethDebugTracingOptions {
        tracer: Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::PreStateTracer,
        )),
        ..Default::default()
    };

    let geth_trace_res = client.debug_trace_transaction(tx.hash, trace_options).await;

    match geth_trace_res {
        Ok(geth_trace) => {
            log::debug!("geth_trace: {:#?}", geth_trace);

            match geth_trace.clone() {
                GethTrace::Known(frame) => {
                    if let GethTraceFrame::PreStateTracer(PreStateFrame::Default(pre_state_mode)) =
                        frame
                    {
                        for (address, account_state) in pre_state_mode.0.iter() {
                            let mut account_info = models::AccountInfo {
                                balance: U256::from(0),
                                code: Bytes::from(account_state.code.clone().unwrap_or_default()),
                                nonce: account_state.nonce.unwrap_or_default().as_u64(),
                                storage: HashMap::new(),
                            };

                            let balance: ethers_core::types::U256 =
                                account_state.balance.unwrap_or_default();
                            // The radix of account_state.balance is 10, while that of account_info.balance is 16.
                            account_info.balance = revm::primitives::U256::from_str_radix(
                                balance.to_string().as_str(),
                                10,
                            )
                            .unwrap();

                            if let Some(storage) = account_state.storage.clone() {
                                for (key, value) in storage.iter() {
                                    let new_key: U256 = U256::from_be_bytes(key.0);
                                    let new_value: U256 = U256::from_be_bytes(value.0);
                                    account_info.storage.insert(new_key, new_value);
                                }
                            }
                            log::info!("test_pre acc_info: {} => balance:{:?} code_len:{} nonce:{} storage:{:?}",
                                Address::from(address.as_fixed_bytes()),
                                account_info.balance,
                                account_info.code.len(),
                                account_info.nonce,
                                account_info.storage,
                            );
                            if !test_pre.contains_key(&Address::from(address.as_fixed_bytes())) {
                                test_pre
                                    .insert(Address::from(address.as_fixed_bytes()), account_info);
                            }
                        }
                    }
                }
                GethTrace::Unknown(_) => {}
            }
        }
        Err(e) => {
            log::info!("debug_trace_transaction faild {}", e)
        }
    }
    test_pre
}

pub async fn batch_process(
    client: Arc<Provider<Http>>,
    block_number: u64,
    chain_id: u64,
) -> (ExecResult, String) {
    let (all_result, json_string) = gen_block_json(client, block_number, chain_id).await;
    //let cnt_chunks = generate_chunks(task, task_id, base_dir, &json_string);
    log::info!("all_result: {:?}", all_result);
    (all_result, json_string)
}

pub async fn gen_block_json(
    client: Arc<Provider<Http>>,
    block_number: u64,
    chain_id: u64,
) -> (ExecResult, String) {
    let client = Arc::clone(&client);
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            gen_block_json_inner(client, block_number, chain_id).await
        })
    })
    .await
    .expect("spawn_blocking panicked")
}

pub async fn gen_block_json_inner(
    client: Arc<Provider<Http>>,
    block_number: u64,
    chain_id: u64,
) -> (ExecResult, String) {
    //let client = Provider::<Http>::try_from(url).unwrap();
    //let client = Arc::new(client);
    let block: ethers_core::types::Block<ethers_core::types::Transaction> =
        match client.get_block_with_txs(block_number).await {
            Ok(Some(block)) => block,
            Ok(None) => panic!("Block not found"),
            Err(error) => panic!("Error: {:?}", error),
        };

    log::debug!("Fetched block number: {:?}", block.number.unwrap());
    let previous_block_number = block_number - 1;

    let prev_id: BlockId = previous_block_number.into();
    // SAFETY: This cannot fail since this is in the top-level tokio runtime
    let state_db = EthersDB::new(Arc::clone(&client), Some(prev_id)).expect("panic");
    let cache_db: CacheDB<EthersDB<Provider<Http>>> = CacheDB::new(state_db);
    let mut state: revm::db::State<CacheDB<EthersDB<Provider<Http>>>> =
        StateBuilder::new_with_database(cache_db).build();

    let mut test_units: BTreeMap<String, models::TestUnit> = BTreeMap::new();

    let mut evm = Evm::builder()
        .with_db(&mut state)
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
    log::debug!("Found {txs} transactions.");

    let start = Instant::now();

    // Fill in CfgEnv
    let mut all_result: Vec<(Vec<u8>, Bytes, Uint<256, 4>, ResultAndState)> = vec![];
    for tx in block.transactions.clone() {
        evm = evm
            .modify()
            .modify_tx_env(|etx| {
                etx.caller = Address::from(tx.from.as_fixed_bytes());
                etx.gas_limit = tx.gas.as_u64();
                local_fill!(etx.gas_price, tx.gas_price, U256::from_limbs);
                local_fill!(etx.value, Some(tx.value), U256::from_limbs);
                etx.data = tx.input.0.clone().into();
                let mut gas_priority_fee = U256::ZERO;
                local_fill!(gas_priority_fee, tx.max_priority_fee_per_gas, U256::from_limbs);
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
        
        let mut cur_result: Vec<(Vec<u8>, Bytes, Uint<256, 4>, ResultAndState)> = vec![];

        let test_pre = fill_test_pre(&tx, &client).await;
        let mut transaction_parts = models::TransactionParts {
            sender: Some(Address::default()),
            to: Some(Address::default()),
            ..Default::default()
        };
        fill_test_tx(&mut transaction_parts, &tx, &block);

        let result = evm.transact();
        if result.is_err() {
            log::error!("evm transact error: {:?}", result);
            continue;
        }
        let result = result.unwrap();
        log::info!("evm transact result: {:?}", result.result);
        evm.context.evm.db.commit(result.state.clone());
        let env = evm.context.evm.env.clone();
        let txbytes = serde_cbor::to_vec(&env.tx).unwrap();

        all_result.push((txbytes.clone(), env.tx.data.clone(), env.tx.value, result.clone()));

        cur_result.push((txbytes, env.tx.data, env.tx.value, result));

        let test_env = fill_test_env(&block);
        let test_post = fill_test_post(&cur_result);

        let test_unit = models::TestUnit {
            info: None,
            chain_id: Some(chain_id),
            env: test_env,
            pre: test_pre,
            post: test_post,
            transaction: transaction_parts,
            out: None,
        };

        test_units.insert(format!("{:?}", tx.hash), test_unit);
    }

    let json_string = serde_json::to_string(&test_units).expect("Failed to serialize");
    log::debug!("test_units: {}", json_string);

    let elapsed = start.elapsed();
    log::info!("Finished execution. Total CPU time: {:.6}s", elapsed.as_secs_f64());

    (Ok(all_result), json_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;
    use revm::primitives::Bytecode;
    #[test]
    fn test_state_merkle_trie_root() {
        let addr =
            Address::from_hex("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap_or_default();
        let plain_account = PlainAccount {
            info: revm::primitives::AccountInfo {
                balance: U256::default(),
                nonce: 0,
                code_hash: FixedBytes::default(),
                code: Some(Bytecode {
                    bytecode: "0x".as_bytes().into(),
                    state: revm::primitives::BytecodeState::Raw,
                }),
            },
            storage: HashMap::new(),
        };

        let plain_accounts: Vec<(Address, PlainAccount)> = vec![(addr, plain_account)];
        let state_root = state_merkle_trie_root(plain_accounts);
        println!("state_root: {:?}", state_root);
    }
}
