// registry.rs

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::prover::Prover;

pub type ProverFactory<T> = fn() -> Box<dyn Prover<T> + Send + Sync>;

static REGISTRY: Lazy<Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_prover<T: 'static>(name: &str, factory: ProverFactory<T>) {
    use std::collections::hash_map::Entry;
    let type_id = TypeId::of::<T>();
    let mut registry = REGISTRY.lock().unwrap();
    match registry.entry(type_id) {
        Entry::Occupied(mut entry) => {
            if let Some(map) = entry.get_mut().downcast_mut::<HashMap<String, ProverFactory<T>>>() {
                map.insert(name.to_string(), factory);
            } else {
                panic!("Registry entry exists for type but downcast failed");
            }
        }
        Entry::Vacant(entry) => {
            let mut map = HashMap::<String, ProverFactory<T>>::new();
            map.insert(name.to_string(), factory);
            entry.insert(Box::new(map));
        }
    }
}

pub fn get_prover_by_type<T: 'static>(prover_type: &str) -> Option<Box<dyn Prover<T> + Send + Sync>> {
    let type_id = TypeId::of::<T>();
    let registry = REGISTRY.lock().unwrap();
    let any_map = registry.get(&type_id)?;
    let map = any_map.downcast_ref::<HashMap<String, ProverFactory<T>>>()?;
    let factory = map.get(prover_type)?;
    Some(factory())
}
