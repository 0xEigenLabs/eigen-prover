#![allow(clippy::all)]
#![allow(unknown_lints)]
use executor_service::executor_service_client::ExecutorServiceClient;
use executor_service::executor_service_server::ExecutorService;
use executor_service::{
    CallTrace, Contract, ExecutionTraceStep, ExecutorError, InfoReadWrite, Log,
    ProcessBatchRequest, ProcessBatchResponse, RomError, TraceConfig, TransactionContext,
    TransactionStep,
};
use log::{debug, error};
use models::*;
use revm::primitives::bitvec::ptr::null;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
pub mod executor_service {
    tonic::include_proto!("executor.v1");
}
use executor::execute_one;
use revm::{
    db::CacheState,
    interpreter::CreateScheme,
    primitives::{
        address, calc_excess_blob_gas, keccak256, Address, Bytecode, Env, SpecId, TransactTo, U256,
    },
};

#[derive(Debug, Default)]
pub struct ExecutorServiceSVC {}

impl ExecutorServiceSVC {
    pub fn new() -> Self {
        ExecutorServiceSVC {}
    }
}

#[tonic::async_trait]
impl ExecutorService for ExecutorServiceSVC {
    async fn process_batch(
        &self,
        request: Request<ProcessBatchRequest>,
    ) -> Result<Response<ProcessBatchResponse>, Status> {
        debug!("Got a request: {:?}", request);
        let msg = request.get_ref();
        let batch_l2_data_result: Result<String, _> = String::from_utf8(msg.batch_l2_data.clone());

        let batch_l2_data = match batch_l2_data_result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting to String: {}", e);
                // Handle the error or return a default string if needed
                String::default()
            }
        };
        let addr = address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b");
        let t: TestUnit = serde_json::from_str(&batch_l2_data).unwrap();
        let res: () = execute_one(&t, addr, 1).unwrap();
        let response = executor_service::ProcessBatchResponse {
            new_state_root: "0x".as_bytes().to_vec(),
            new_acc_input_hash: "0x".as_bytes().to_vec(),
            new_local_exit_root: "0x".as_bytes().to_vec(),
            new_batch_num: 1,
            cnt_keccak_hashes: 1,
            cnt_poseidon_hashes: 1,
            cnt_poseidon_paddings: 1,
            cnt_mem_aligns: 1,
            cnt_arithmetics: 1,
            cnt_binaries: 1,
            cnt_steps: 1,
            cumulative_gas_used: 1,
            responses: [].to_vec(),
            error: 0,
            read_write_addresses: HashMap::new(),
        };
        Ok(Response::new(response))
    }
}
