use statedb_service::state_db_service_client::StateDbServiceClient;
use statedb_service::{SetRequest, SetResponse};
use statedb_service::Fea;

pub mod statedb_service {
    tonic::include_proto!("statedb.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StateDbServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SetRequest {
        old_root: Some(Fea { fe0: 1, fe1: 1, fe2: 1, fe3: 1 }),
        key: Some(Fea { fe0: 1, fe1: 1, fe2: 1, fe3: 1 }),
        value: "1".to_string(),
        persistent: true,
        details: true,
        get_db_read_log: true
    });

    let response = client.set(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
