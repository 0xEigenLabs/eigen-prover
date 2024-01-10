use executor_service::executor_service_client::ExecutorServiceClient;
use executor_service::{ExecutorError, ProcessBatchRequest, TraceConfig};

use std::collections::HashMap;

pub mod executor_service {
    tonic::include_proto!("executor.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut executor_client = ExecutorServiceClient::connect("http://0.0.0.0:50061").await?;

    let test_file = "../executor/test-vectors/blockInfo.json";
    println!("test_file {}", test_file);
    let batch_l2_data_json = std::fs::read_to_string(test_file).unwrap();
    let request = ProcessBatchRequest {
        old_state_root: "0x".as_bytes().to_vec(),
        old_acc_input_hash: "0x".as_bytes().to_vec(),
        old_batch_num: 1,
        chain_id: 1,
        fork_id: 1,
        batch_l2_data: batch_l2_data_json.as_bytes().to_vec(),
        global_exit_root: "0x".as_bytes().to_vec(),
        eth_timestamp: 1635870424,
        coinbase: "0x".to_string(),
        update_merkle_tree: 1,
        no_counters: 0,
        from: "0x".to_string(),
        db: HashMap::new(),
        contracts_bytecode: HashMap::new(),
        trace_config: Some(TraceConfig {
            disable_storage: 1,
            disable_stack: 1,
            enable_memory: 1,
            enable_return_data: 1,
            tx_hash_to_generate_execute_trace: Vec::new(),
            tx_hash_to_generate_call_trace: Vec::new(),
        }),
    };
    println!("request: {:?}", request);
    let response = executor_client.process_batch(request).await?.into_inner();
    if response.error == ExecutorError::NoError.into() {
        println!("process batch success");
    } else {
        // Handle other error cases
        eprintln!("process batch failed, Error: {}", response.error);
    }

    println!("response: {:?}", response);
    Ok(())
}
