#![allow(clippy::all)]
#![allow(unknown_lints)]

use executor_service::executor_service_server::ExecutorService;
use executor_service::{ExecutorError, ProcessBatchRequest, ProcessBatchResponse};
use log::debug;
use std::env as stdenv;
//use models::*;
use tonic::{Request, Response, Status};
pub mod executor_service {
    tonic::include_proto!("executor.v1");
}
use executor::execute_one;
use revm::primitives::ResultAndState;
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
        // let t: TestUnit = serde_json::from_str(&batch_l2_data).unwrap();
        let block_number = batch_l2_data.parse::<u64>().unwrap();

        let task = stdenv::var("TASK").unwrap_or(String::from("evm"));
        let slot_path = stdenv::var("SLOT").unwrap_or(format!("/mnt/data/{}/storage", task));
        let base_dir = stdenv::var("BASEDIR").unwrap_or(String::from("/mnt/data"));
        let execute_task_id = uuid::Uuid::new_v4();

        let (_res, cnt_chunks) = execute_one(
            block_number,
            1,
            &slot_path,
            &task,
            execute_task_id.to_string().as_str(),
            base_dir.as_str(),
        )
        .await;
        let mut response = executor_service::ProcessBatchResponse::default();
        let last_element = match _res {
            Ok(res) => {
                response.error = ExecutorError::NoError.into();
                debug!("exec success");
                res.last().cloned()
            }
            Err(e) => {
                response.error = ExecutorError::Unspecified.into();
                debug!("exec error: {:?}", e);
                None
            }
        };

        debug!("execute_one last_element: {:?}", last_element);

        response.execute_task_id = execute_task_id.to_string();
        response.cnt_chunks = cnt_chunks as u32;
        let (txbytes, data, value, ResultAndState { result, state }) = last_element.unwrap();
        {
            // 1. expect_exception: Option<String>,
            debug!("expect_exception: {:?}", result.is_success());
            // indexes: TxPartIndices,
            debug!(
                "indexes: data{:?}, value: {}, gas: {}",
                data,
                value,
                result.gas_used()
            );
            debug!("output: {:?}", result.output());

            // TODO: hash: B256, // post state root
            //let hash = serde_json::to_vec(&state).unwrap();
            //println!("hash: {:?}", state);
            // post_state: HashMap<Address, AccountInfo>,
            debug!("post_state: {:?}", state);
            // logs: B256,
            debug!("logs: {:?}", result.logs());
            // txbytes: Option<Bytes>,
            debug!("txbytes: {:?}", txbytes);
        }

        Ok(Response::new(response))
    }
}
