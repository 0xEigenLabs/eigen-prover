#![allow(dead_code)]
use ethers_core::types::BlockId;
use ethers_providers::Middleware;
use ethers_providers::{Http, Provider};
use indicatif::ProgressBar;
use revm::db::{CacheDB, EmptyDB, EthersDB};
use revm::inspectors::TracerEip3155;
use revm::primitives::HashSet;
use revm::primitives::{Address, ResultAndState, TransactTo, U256};
use revm::{inspector_handle_register, Database, DatabaseCommit, Evm};
use ruint::uint;
use ruint::Uint;
use std::env as stdenv;
use std::io::BufWriter;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use statedb::database::Database as StateDB;

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

struct FlushWriter {
    writer: Arc<Mutex<BufWriter<std::fs::File>>>,
}

impl FlushWriter {
    fn new(writer: Arc<Mutex<BufWriter<std::fs::File>>>) -> Self {
        Self { writer }
    }
}

impl Write for FlushWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.lock().unwrap().flush()
    }
}

// Usage: NO=457 cargo run --release --example exec_block
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create ethers client and wrap it in Arc<M>
    //let client = Provider::<Http>::try_from(
    //    "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    //)?;
    let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let client = Arc::new(client);
    // Params
    let chain_id: u64 = 1;
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("0"));
    let block_number: u64 = env_block_number.parse().unwrap();

    // Fetch the transaction-rich block
    let block = match client.get_block_with_txs(block_number).await {
        Ok(Some(block)) => block,
        Ok(None) => anyhow::bail!("Block not found"),
        Err(error) => anyhow::bail!("Error: {:?}", error),
    };
    println!("Fetched block number: {}", block.number.unwrap().0[0]);
    let previous_block_number = block_number - 1;

    // Use the previous block state as the db with caching
    let prev_id: BlockId = previous_block_number.into();

    let mut ethersdb = EthersDB::new(Arc::clone(&client), Some(prev_id)).unwrap();
    //let mut cache_db = CacheDB::new(ethersdb);
    let mut cache_db = CacheDB::new(EmptyDB::default());
    // get pre

    let mut db = StateDB::new(None);

    for tx in &block.transactions {
        let from_acc = Address::from(tx.from.as_fixed_bytes());
        // query basic properties of an account incl bytecode
        let acc_info = ethersdb.basic(from_acc).unwrap().unwrap();
        log::info!("acc_info: {} => {:?}", from_acc, acc_info);
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
                    println!("not found slot in db, account_slot_json: {:?}", account_slot_json_str);
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
    println!("Found {txs} transactions.");

    let console_bar = Arc::new(ProgressBar::new(txs as u64));
    let elapsed = std::time::Duration::ZERO;

    // Create the traces directory if it doesn't exist
    // std::fs::create_dir_all("traces").expect("Failed to create traces directory");

    let mut all_result = vec![];
    // Fill in CfgEnv
    for tx in block.transactions {
        evm = evm
            .modify()
            .modify_tx_env(|etx| {
                etx.caller = Address::from(tx.from.as_fixed_bytes());
                etx.gas_limit = tx.gas.as_u64();
                local_fill!(etx.gas_price, tx.gas_price, U256::from_limbs);
                local_fill!(etx.value, Some(tx.value), U256::from_limbs);
                etx.data = tx.input.0.into();
                let mut gas_priority_fee = U256::ZERO;
                local_fill!(
                    gas_priority_fee,
                    tx.max_priority_fee_per_gas,
                    U256::from_limbs
                );
                etx.gas_priority_fee = Some(gas_priority_fee);
                etx.chain_id = Some(chain_id);
                etx.nonce = Some(tx.nonce.as_u64());
                if let Some(access_list) = tx.access_list {
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

        let result = evm.transact().unwrap();
        println!("evm transact result: {:?}", result.result);
        evm.context.evm.db.commit(result.state.clone());
        let env = evm.context.evm.env.clone();
        let txbytes = serde_json::to_vec(&env.tx).unwrap();
        all_result.push((txbytes, env.tx.data, env.tx.value, result));
        // Construct the file writer to write the trace to
        // let tx_number = tx.transaction_index.unwrap().0[0];
        // let file_name = format!("traces/{}.json", tx_number);
        // let write = OpenOptions::new().write(true).create(true).open(file_name);
        // let inner = Arc::new(Mutex::new(BufWriter::new(
        //     write.expect("Failed to open file"),
        // )));
        // let writer = FlushWriter::new(Arc::clone(&inner));

        // // Inspect and commit the transaction to the EVM
        // evm.context.external.set_writer(Box::new(writer));
        // if let Err(error) = evm.transact_commit() {
        //     println!("Got error: {:?}", error);
        // }

        // // Flush the file writer
        // inner.lock().unwrap().flush().expect("Failed to flush file");

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
                panic!("Failed to write nodes: {:?}", write_res);
            }
        }
        console_bar.inc(1);
    }
    for (k, v) in &evm.context.evm.db.accounts {
        println!("state: {}=>{:?}", k, v);
        if !v.storage.is_empty() {
            for (k, v) in v.storage.iter() {
                println!("slot => storage: {}=>{}", k, v);
            }
        }
    }
    // get `post: BTreeMap<SpecName, Vec<Test>>`
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
        }
    }
    console_bar.finish_with_message("Finished all transactions.");
    println!(
        "Finished execution. Total CPU time: {:.6}s",
        elapsed.as_secs_f64()
    );

    Ok(())
}
