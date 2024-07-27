use algebraic::circom_circuit::{CircomCircuit, R1CS};
use algebraic::reader::load_r1cs;
use groth16::api::{read_pk_from_file, read_vk_from_file};
use groth16::bellman_ce::bls12_381::Bls12;
use groth16::bellman_ce::bn256::Bn256;
use groth16::bellman_ce::groth16::{Parameters, VerifyingKey};
use groth16::bellman_ce::pairing::Engine;
use groth16::bellman_ce::ScalarEngine;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub enum SnarkCache {
    Bn256(GenericData<Bn256>),
    Bls12(GenericData<Bls12>),
}

#[derive(Clone)]
pub enum CurveType {
    BN256,
    BLS12381,
}

#[derive(Clone)]
pub struct GenericData<E: ScalarEngine + Engine> {
    pub circuit: CircomCircuit<E>,
    pub pk: Parameters<E>,
    pub vk: VerifyingKey<E>,
}

lazy_static! {
    static ref CACHE_INITIALIZED: Mutex<bool> = Mutex::new(false);
    pub static ref GLOBAL_SNARK_CACHE: Arc<RwLock<Option<SnarkCache>>> =
        Arc::new(RwLock::new(None));
}

fn initialize_cache(curve_type: &str, r1cs_file: &str, pk_file: &str, vk_file: &str) -> SnarkCache {
    match curve_type {
        "BN128" => {
            let initial_data = GenericData {
                circuit: CircomCircuit {
                    r1cs: load_r1cs(r1cs_file),
                    witness: None,
                    wire_mapping: None,
                    aux_offset: 0,
                },
                pk: read_pk_from_file(pk_file, false).unwrap(),
                vk: read_vk_from_file(vk_file).unwrap(),
            };
            SnarkCache::Bn256(initial_data)
        }
        "BLS12381" => {
            let initial_data = GenericData {
                circuit: CircomCircuit {
                    r1cs: load_r1cs(r1cs_file),
                    witness: None,
                    wire_mapping: None,
                    aux_offset: 0,
                },
                pk: read_pk_from_file(pk_file, false).unwrap(),
                vk: read_vk_from_file(vk_file).unwrap(),
            };
            SnarkCache::Bls12(initial_data)
        }
        _ => panic!("Unsupported curve type"),
    }
}

pub fn initialize_global_cache(curve_type: &str, r1cs_file: &str, pk_file: &str, vk_file: &str) {
    let mut cache_initialized = CACHE_INITIALIZED.lock().unwrap();
    if !*cache_initialized {
        log::debug!("Initializing global cache");
        let cache = initialize_cache(curve_type, r1cs_file, pk_file, vk_file);
        let mut global_cache = GLOBAL_SNARK_CACHE.write();
        *global_cache = Some(cache);
        *cache_initialized = true;
    } else {
        log::debug!("Global cache has already been initialized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_global_cache() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::try_init().unwrap_or_default();
        let current_dir = std::env::current_dir().unwrap();
        let r1cs_path = current_dir.join("test-vectors/mycircuit_bls12381.r1cs");
        let pk_path = current_dir.join("test-vectors/g16.key");
        let vk_path = current_dir.join("test-vectors/verification_key.json");

        let r1cs_file = r1cs_path.to_str().expect("Invalid r1cs_file path");
        let pk_file = pk_path.to_str().expect("Invalid pk_file path");
        let vk_file = vk_path.to_str().expect("Invalid vk_file path");

        // Call initialize_global_cache
        initialize_global_cache("bls12381", &r1cs_file, &pk_file, &vk_file);

        // Check the cache has been initialized
        let snark_cache = GLOBAL_SNARK_CACHE.read();

        if let Some(snark_cache) = &*snark_cache {
            match snark_cache {
                SnarkCache::Bls12(snark_data) => {}

                SnarkCache::Bn256(snark_data) => {}
                _ => panic!("Unsupported curve type"),
            }
        }

        initialize_global_cache("bls12381", &r1cs_file, &pk_file, &vk_file);
    }
}
