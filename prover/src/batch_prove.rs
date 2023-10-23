use crate::ProveStage;
use dsl_compile::circom_compiler;
use starky::prove::stark_prove;
use std::io::Read;

struct BatchProveParam {
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

/// Generate stark proof and generate its verifier circuit in circom
async fn batch_proof(
    task_id: usize,
    param: &BatchProveParam,
    link_directories: &String,
) -> Result<(), ()> {
    log::info!("start batch prove");

    let output_dir = ProveStage::BatchProve::to_path(task_id);
    let circom_file = format!("{}/C12_VERIFIER.circom", output_dir);

    // 1. stark prove: generate `.circom` file.
    stark_prove(
        &param.stark_struct,
        &param.pil_json,
        true,
        true,
        &param.const_pols,
        &param.cm_pols,
        &circom_file,
        &param.zkin,
        &param.prover_addr,
    )?;

    // 2. Compile circom circuit to r1cs, and generate witness
    circom_compiler(
        circom_file,
        "goldilocks".to_string(), // prime
        "full".to_string(),       // full_simplification
        param.link_directories,
        output_dir,
        true, // no_simplification
        true, // reduced_simplification
    )?;
    log::info!("end batch prove");
}
