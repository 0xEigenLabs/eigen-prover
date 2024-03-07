use anyhow::{bail, Result};
use statedb::database::Database;

pub trait Storage {
    fn read_nodes(&mut self, key: &str) -> Result<String>;
    fn write_nodes(&mut self, key: &str, value: &str, update: bool) -> Result<usize>;
}

#[derive(Default)]
pub struct MemoryDB {
    db: std::collections::HashMap<String, String>,
}

pub struct StateDB {
    db: Database,
}

impl StateDB {
    pub fn new() -> Self {
        Self {
            db: Database::new(None)
        }
    }
}

impl Storage for MemoryDB {
    fn read_nodes(&mut self, key: &str) -> Result<String> {
        if let Some(v) = self.db.get(key) {
            Ok(v.clone())
        } else {
            bail!("Key not exist")
        }
    }
    fn write_nodes(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        self.db.insert(key.to_string(), value.to_string());
        Ok(value.len())
    }
}

impl Storage for StateDB {
    fn read_nodes(&mut self, key: &str) -> Result<String> {
        self.db.read_nodes(key)
    }
    fn write_nodes(&mut self, key: &str, value: &str, update: bool) -> Result<usize> {
        self.db.write_nodes(key, value, update)
    }
}

pub fn new() -> Box<dyn Storage> {
    let use_memdb = std::env::var("USE_MEM").unwrap_or("NO".to_string());
    if use_memdb.as_str() == "NO" {
        Box::new(StateDB::new())
    } else {
        Box::new(MemoryDB::default())
    }
}
