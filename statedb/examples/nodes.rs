use statedb::database::Database;

fn main() {
    let mut db = Database::new();
    let key = "name".to_string();
    let res = db.read_program(&key);
    println!("{:?}", res);
}
