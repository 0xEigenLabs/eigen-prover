use std::sync::Arc;

use state::database::{Database, DEFAULT_ROOT_KEY};

#[tokio::main]
async fn main() {
    // Create a new state database connection pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let root_key = std::env::var("ROOT_KEY").unwrap_or(DEFAULT_ROOT_KEY.to_string());
    let db = Arc::new(Database::new(&url, &root_key).await);

    let key = "name".to_string();
    let res = db.read_program(&key).await;
    println!("read program: {} {:?}", key, res);

    let value = "value".chars().map(|c| c as u8).collect::<Vec<_>>();
    let res = db.write_program(&key, &value, false).await;
    println!("{:?}", res);

    let res = db.read_program(&key).await;
    println!("{:?}", res);

    let value = "value2".chars().map(|c| c as u8).collect::<Vec<_>>();
    let res = db.write_program(&key, &value, true).await;
    println!("{:?}", res);

    let res = db.read_program(&key).await;
    println!("{:?}", res);

    let res = db.read_nodes(&key).await;
    println!("read nodes: {} {:?}", key, res);

    let value = "value".to_string();
    let res = db.write_nodes(&key, &value, false).await;
    println!("{:?}", res);

    let res = db.read_nodes(&key).await;
    println!("{:?}", res);

    let value = "value2".to_string();
    let res = db.write_nodes(&key, &value, true).await;
    println!("{:?}", res);

    let res = db.read_nodes(&key).await;
    println!("{:?}", res);
}
