use statedb::database::Database;

fn main() {
    let mut db = Database::new(None);
    let key = "name".to_string();
    let res = db.read_program(&key);
    println!("read program: {} {:?}", key, res);

    let value = "value".to_string();
    let res = db.write_program(&key, &value, false);
    println!("{:?}", res);

    let res = db.read_program(&key);
    println!("{:?}", res);

    let value = "value2".to_string();
    let res = db.write_program(&key, &value, true);
    println!("{:?}", res);

    let res = db.read_program(&key);
    println!("{:?}", res);

    let res = db.read_nodes(&key);
    println!("read nodes: {} {:?}", key, res);

    let value = "value".to_string();
    let res = db.write_nodes(&key, &value, false);
    println!("{:?}", res);

    let res = db.read_nodes(&key);
    println!("{:?}", res);

    let value = "value2".to_string();
    let res = db.write_nodes(&key, &value, true);
    println!("{:?}", res);

    let res = db.read_nodes(&key);
    println!("{:?}", res);
}
