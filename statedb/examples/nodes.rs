use statedb::database::Database;

fn main() {
    let mut db = Database::new();
    let key = "name".to_string();
    let res = db.read_program(&key);
    println!("{:?}", res);

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
}
