use statedb::statedb_client::statedb_service::{Fea};
use statedb::statedb_client::StateDBClientCli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://0.0.0.0:50061".to_string();
    let client = StateDBClientCli::new(addr).await.unwrap();

    let old_root = Fea {
        fe0: 0,
        fe1: 0,
        fe2: 0,
        fe3: 0,
    };
    let key = Fea {
        fe0: 1,
        fe1: 1,
        fe2: 1,
        fe3: 1,
    };
    let value = "1".to_string();

    let resp = client.set(old_root, key.clone(), value).await.unwrap();
    println!("RESPONSE={:?}", resp);

    // let new_root = resp.get_ref().new_root.clone().unwrap();
    // let resp1 = client.get(new_root, key.clone()).await.unwrap();
    // let val = resp1.get_ref().value.clone();
    // println!("val={:?}", val);

    Ok(())
}