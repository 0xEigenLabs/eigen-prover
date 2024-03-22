#![allow(clippy::all)]
#![allow(unknown_lints)]
use ethers_core::types::{Block, Transaction, H160, H256, U256, U64};
use ethers_core::utils::hex;
use ethers_core::utils::rlp::{Decodable, Rlp};
use executor_service::executor_service_server::ExecutorService;
use executor_service::{ExecutorError, ProcessBatchRequest, ProcessBatchResponse};
use log::debug;
use std::env as stdenv;
//use models::*;
use ethers_providers::{Http, Provider};
use std::sync::Arc;
use tonic::{Request, Response, Status};
pub mod executor_service {
    tonic::include_proto!("executor.v1");
}
use executor::batch_process;
use revm::primitives::ResultAndState;

#[derive(Debug)]
pub struct ExecutorServiceSVC {
    client: Arc<Provider<Http>>,
}

impl ExecutorServiceSVC {
    pub fn new() -> Self {
        let url = std::env::var("URL").unwrap_or(String::from("http://localhost:8545"));
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);
        ExecutorServiceSVC { client }
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
        // FIXME: need more block data
        let raw_tx = Rlp::new(&msg.batch_l2_data);
        let tx = Transaction::decode(&raw_tx).unwrap();
        let author: Option<H160> = match hex::decode(msg.coinbase.clone()) {
            Ok(x) => Some(H160::from_slice(&x)),
            _ => None,
        };
        let block = Block {
            author: author,
            number: Some(U64::from(0)),
            transactions: vec![tx],
            timestamp: msg.eth_timestamp.into(),
            gas_limit: U256::from(80_000_000u128),
            parent_hash: H256::default(),
            gas_used: U256::from(0),
            difficulty: U256::from(0),
            ..Default::default()
        };

        let task = stdenv::var("TASK").unwrap_or(String::from("lr"));
        let base_dir = stdenv::var("BASEDIR").unwrap_or(String::from("/tmp"));
        let execute_task_id = uuid::Uuid::new_v4();
        let (res, cnt_chunks) = batch_process(
            self.client.clone(),
            msg.chain_id,
            block,
            &task,
            execute_task_id.to_string().as_str(),
            base_dir.as_str(),
        )
        .await;
        let mut response = executor_service::ProcessBatchResponse::default();
        let last_element = match res {
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

        debug!("batch_process last_element: {:?}", last_element);

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
