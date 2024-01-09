#![allow(clippy::all)]
#![allow(unknown_lints)]

use executor_service::executor_service_server::ExecutorService;
use executor_service::{ProcessBatchRequest, ProcessBatchResponse, ExecutorError};
use log::debug;
use models::*;

use tonic::{Request, Response, Status};
pub mod executor_service {
    tonic::include_proto!("executor.v1");
}
use executor::execute_one;
use revm::primitives::address;

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
        let _res = execute_one(&t, addr, 1);
        let mut response = executor_service::ProcessBatchResponse::default();
        let last_element = match _res {
            Ok(res) => {
                response.error = ExecutorError::NoError.into();
                println!("exec success");
                res.last().cloned()
            }
            Err(e) => {
                response.error = ExecutorError::Unspecified.into();
                eprintln!("exec error: {:?}", e);
                None
            }
        };

        debug!("execute_one last_element: {:?}", last_element);

        Ok(Response::new(response))
    }
}
