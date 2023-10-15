async fn final_stark() {
    log::info!("start final_stark prove");
    // 1. Compile circom circuit to r1cs, and generate witness
    // circom_compiler(
    //     args.input,
    //     args.prime,
    //     args.full_simplification,
    //     args.link_directories,
    //     args.output,
    //     args.no_simplification,
    //     args.reduced_simplification,
    // )?;

    // 2. compress setup
    // setup(
    // &args.r1cs_file,
    // &args.pil_file,
    // &args.const_file,
    // &args.exec_file,
    // args.force_n_bits,
    // )?;

    // 3. compress exec
    // exec(
    //     &args.input_file,
    //     &args.wasm_file,
    //     &args.pil_file,
    //     &args.exec_file,
    //     &args.commit_file,
    // )?;

    // 4. generate final proof

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

    log::info!("end final stark prove");
}
