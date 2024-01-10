use revm::{
    db::CacheState,
    interpreter::CreateScheme,
    primitives::{
        calc_excess_blob_gas, keccak256, Address, Bytecode, Env, ExecutionResult, SpecId,
        TransactTo, U256,
    },
};

use anyhow::Result;
use models::*;

extern crate alloc;

use alloc::vec::Vec;

// pub struct Executor {
// }

// impl Default for Executor {
//     fn default() -> Self {
//         Executor {}
//     }
// }

pub fn execute_one(unit: &TestUnit, addr: Address, chain_id: u64) -> Result<Vec<ExecutionResult>> {
    // Create database and insert cache
    let mut cache_state = CacheState::new(false);
    for (address, info) in &unit.pre {
        let acc_info = revm::primitives::AccountInfo {
            balance: info.balance,
            code_hash: keccak256(&info.code),
            code: Some(Bytecode::new_raw(info.code.clone())),
            nonce: info.nonce,
        };
        cache_state.insert_account_with_storage(*address, acc_info, info.storage.clone());
    }

    let mut env = Env::default();
    // for mainnet
    env.cfg.chain_id = chain_id;
    // env.cfg.spec_id is set down the road

    // block env
    env.block.number = unit.env.current_number;
    env.block.coinbase = unit.env.current_coinbase;
    env.block.timestamp = unit.env.current_timestamp;
    env.block.gas_limit = unit.env.current_gas_limit;
    env.block.basefee = unit.env.current_base_fee.unwrap_or_default();
    env.block.difficulty = unit.env.current_difficulty;
    // after the Merge prevrandao replaces mix_hash field in block and replaced difficulty opcode in EVM.
    env.block.prevrandao = Some(unit.env.current_difficulty.to_be_bytes().into());
    // EIP-4844
    if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)) = (
        unit.env.parent_blob_gas_used,
        unit.env.parent_excess_blob_gas,
    ) {
        env.block
            .set_blob_excess_gas_and_price(calc_excess_blob_gas(
                parent_blob_gas_used.to(),
                parent_excess_blob_gas.to(),
            ));
    }

    /*
     (
            b256!("45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8"),
            address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b"),
        ),
    */
    // tx env
    //let pk = unit.transaction.secret_key;
    env.tx.caller = addr; //Address::parse_checksummed(addr, Some(chain_id)).expect("Expect valid checksum"); //map_caller_keys.get(&pk).copied().ok_or_else(|| String::new())?;
    env.tx.gas_price = unit
        .transaction
        .gas_price
        .or(unit.transaction.max_fee_per_gas)
        .unwrap_or_default();
    env.tx.gas_priority_fee = unit.transaction.max_priority_fee_per_gas;
    // EIP-4844
    env.tx.blob_hashes = unit.transaction.blob_versioned_hashes.clone();
    env.tx.max_fee_per_blob_gas = unit.transaction.max_fee_per_blob_gas;

    let mut all_result = vec![];
    // post and execution
    for (spec_name, tests) in &unit.post {
        if matches!(
            spec_name,
            SpecName::ByzantiumToConstantinopleAt5 | SpecName::Constantinople | SpecName::Unknown
        ) {
            continue;
        }

        env.cfg.spec_id = spec_name.to_spec_id();

        for test in tests {
            env.tx.gas_limit = unit.transaction.gas_limit[test.indexes.gas].saturating_to();

            env.tx.data = unit
                .transaction
                .data
                .get(test.indexes.data)
                .unwrap()
                .clone();
            env.tx.value = unit.transaction.value[test.indexes.value];

            env.tx.access_list = unit
                .transaction
                .access_lists
                .get(test.indexes.data)
                .and_then(Option::as_deref)
                .unwrap_or_default()
                .iter()
                .map(|item| {
                    (
                        item.address,
                        item.storage_keys
                            .iter()
                            .map(|key| U256::from_be_bytes(key.0))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect();

            let to = match unit.transaction.to {
                Some(add) => TransactTo::Call(add),
                None => TransactTo::Create(CreateScheme::Create),
            };
            env.tx.transact_to = to;

            let mut cache = cache_state.clone();
            cache.set_state_clear_flag(SpecId::enabled(
                env.cfg.spec_id,
                revm::primitives::SpecId::SPURIOUS_DRAGON,
            ));
            let mut state = revm::db::State::builder()
                .with_cached_prestate(cache)
                .with_bundle_update()
                .build();
            let mut evm = revm::new();
            evm.database(&mut state);
            evm.env = env.clone();

            // do the deed
            let exec_result: ExecutionResult = evm.transact_commit().map_err(anyhow::Error::msg)?;

            all_result.push(exec_result);
        }
    }
    Ok(all_result)
}

#[cfg(test)]
mod tests {
    use super::execute_one;
    use revm::primitives::{address, b256};
    //use runtime::{print, get_prover_input, coprocessors::{get_data, get_data_len}};

    use models::*;

    #[test]
    fn test_execute_one() {
        let _map_caller_keys = [
            (
                b256!("45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8"),
                address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b"),
            ),
            (
                b256!("c85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4"),
                address!("cd2a3d9f938e13cd947ec05abc7fe734df8dd826"),
            ),
            (
                b256!("044852b2a670ade5407e78fb2863c51de9fcb96542a07186fe3aeda6bb8a116d"),
                address!("82a978b3f5962a5b0957d9ee9eef472ee55b42f1"),
            ),
            (
                b256!("6a7eeac5f12b409d42028f66b0b2132535ee158cfda439e3bfdd4558e8f4bf6c"),
                address!("c9c5a15a403e41498b6f69f6f89dd9f5892d21f7"),
            ),
            (
                b256!("a95defe70ebea7804f9c3be42d20d24375e2a92b9d9666b832069c5f3cd423dd"),
                address!("3fb1cd2cd96c6d5c0b5eb3322d807b34482481d4"),
            ),
            (
                b256!("fe13266ff57000135fb9aa854bbfe455d8da85b21f626307bf3263a0c2a8e7fe"),
                address!("dcc5ba93a1ed7e045690d722f2bf460a51c61415"),
            ),
        ];

        let test_file = "test-vectors/blockInfo.json";
        let suite_json = std::fs::read_to_string(test_file).unwrap();
        println!("suite json: {:?}", suite_json);

        let addr = address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b");
        let t: TestUnit = serde_json::from_str(&suite_json).unwrap();
        println!("TestUnit t: {:?}", t);
        let res: Result<Vec<revm::primitives::ExecutionResult>, anyhow::Error> =
            execute_one(&t, addr, 1);

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
