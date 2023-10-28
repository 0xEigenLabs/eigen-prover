use crate::traits::Executor;
use crate::ProveStage;
use algebraic::errors::{EigenError, Result};
use dsl_compile::circom_compiler;
use starky::prove::stark_prove;
use std::io::Read;

#[derive(Default)]
pub struct BatchProver {
    // stark prove params
    stark_struct: String, // stark_struct.json
    pil_json: String,     // .pil.json file
    // norm_stage: bool,// default: true
    // agg_stage: bool,// default: true
    const_pols: String,
    cm_pols: String,
    zkin: String,
    prover_addr: String,

    // compile params
    link_directories: Vec<String>, // setup the library path
}

impl BatchProver {
    pub fn new() -> Self {
        BatchProver {
            ..Default::default()
        }
    }
}

impl Executor for BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn execute(&self, basedir: &str, task_id: &str) -> Result<()> {
        log::info!("start batch prove");

        let output_dir = "".to_string();
        let circom_file = format!("{}/C12_VERIFIER.circom", output_dir);

        // 1. stark prove: generate `.circom` file.
        stark_prove(
            &self.stark_struct,
            &self.pil_json,
            true,
            true,
            &self.const_pols,
            &self.cm_pols,
            &circom_file,
            &self.zkin,
            &self.prover_addr,
        ).unwrap();

        // 2. Compile circom circuit to r1cs, and generate witness
        circom_compiler(
            circom_file,
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            self.link_directories.clone(),
            output_dir,
            true, // no_simplification
            true, // reduced_simplification
        ).unwrap();
        log::info!("end batch prove");
        Ok(())
    }
}
