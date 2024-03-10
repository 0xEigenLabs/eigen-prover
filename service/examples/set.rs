use proto::state::{state_service_client::StateServiceClient, Fea, SetRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StateServiceClient::connect("http://0.0.0.0:50061").await?;

    let request = tonic::Request::new(SetRequest {
        old_root: Some(Fea {
            fe0: 0,
            fe1: 0,
            fe2: 0,
            fe3: 0,
        }),
        key: Some(Fea {
            fe0: 1,
            fe1: 1,
            fe2: 1,
            fe3: 1,
        }),
        value: "1".to_string(),
        persistent: true,
        details: true,
        get_db_read_log: true,
    });

    let response = client.set(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
