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
    let mut request = ProcessBatchRequest::default();
    request.batch_l2_data=batch_l2_data_json.as_bytes().to_vec();
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
