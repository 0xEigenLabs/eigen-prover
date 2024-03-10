use statedb::database::Database;

#[tokio::main]
async fn main() {
    let mut db = Database::new(None).await;

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
