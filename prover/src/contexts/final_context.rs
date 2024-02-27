use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::FinalProveArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalContext {
    pub basedir: String,

    pub final_circom: CircomCompileArgs,
    pub final_snark: FinalProveArgs,
    pub final_stark: StarkProveArgs,
    pub final_stark_struct: String,

    pub prover_addr: String,
    pub recursive2_circom: CircomCompileArgs,
    pub recursive2_stark: StarkProveArgs,

    pub task_id: String,
    pub task_name: String,
}

impl FinalContext {
    pub fn new(
        basedir: String,
        task_id: String,
        task_name: String,
        curve: String,
        prover_addr: String,
    ) -> Self {
        let prev_task_path = Stage::Aggregate(task_id.clone(), "".into(), "".into()).path();
        let task_path = Stage::Final(task_id.clone(), curve.clone(), prover_addr.clone()).path();
        let r2_task_name = format!("{}.recursive2", task_name);
        let final_task_name = format!("{}.final", task_name);
        FinalContext {
            basedir: basedir.clone(),
            task_id,
            prover_addr,
            task_name,
            final_stark_struct: format!("{}/final.stark_struct.json", basedir),
            final_stark: StarkProveArgs::new(&basedir, &prev_task_path, &final_task_name, &curve),
            recursive2_stark: StarkProveArgs::new(&basedir, &prev_task_path, &r2_task_name, &curve),
            final_circom: CircomCompileArgs::new(
                &basedir,
                &prev_task_path,
                &final_task_name,
                &curve,
            ),
            recursive2_circom: CircomCompileArgs::new(
                &basedir,
                &prev_task_path,
                &r2_task_name,
                "GL",
            ),
            final_snark: FinalProveArgs {
                curve_type: curve,
                pk_file: format!("{basedir}/{task_path}/g16.key"),
                vk_file: format!("{basedir}/{task_path}/verification_key.json"),
                public_input_file: format!("{basedir}/{task_path}/public_input.json"),
                proof_file: format!("{basedir}/{task_path}/proof.json"),
            },
        }
    }
}
