use dsl_compile::circom_compiler;
use starky::prove::stark_prove;
use std::io::Read;

// # task_id_1/execute/{cm, const, pil.json}
// # batch_proof: stark_prove cm const pil.  task_id_1/batch_proof/{}
// # c12(norm): compression_setup/exec/stark_prove
// # agg:...
// # final stark:...
// # snark(g16): bellman g16(ark-works/g16)

async fn batch_proof(task_id: usize, pil_file: &String) -> Result<(), ()> {
    log::info!("start batch prove");

    // 1. stark prove
    // generate .circom file.
    // input files :  .pil json & starkStruct.json.gl
    // output files : .circom
    let pil_json_file = "1";
    // stark_prove(
    //     &args.stark_struct,
    //     &args.piljson,
    //     args.norm_stage,
    //     args.agg_stage,
    //     &args.const_pols,
    //     &args.cm_pols,
    //     &args.circom_file,
    //     &args.zkin,
    //     &args.prover_addr,
    // )?;

    // 2. Compile circom circuit to r1cs, and generate witness
    // circom_compiler(
    //     args.input,
    //     args.prime,
    //     args.full_simplification,
    //     args.link_directories,
    //     args.output,
    //     args.no_simplification,
    //     args.reduced_simplification,
    // )?;
    log::info!("end batch prove");
}
