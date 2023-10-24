#![allow(dead_code)]
// mod aggregate_prove;
// mod batch_prove;
// mod c12_prove;
// mod final_stark_prove;
// mod snark;

#[derive(Clone, Debug)]
pub enum ProveStage {
    AggProve,
    BatchProve,
    C12Prove,
    FinalStarkProve,
    SnarkProve,
}

impl ProveStage {
    fn to_path(&self, task_id: usize) -> String {
        let prefix = "/tmp";
        // let stage_name = match self {
        //     Self::AggProve => "agg_proof",
        //     Self::BatchProve => "batch_proof",
        //     Self::C12Prove => "c12_proof",
        //     Self::FinalStarkProve => "final_stark_proof",
        //     Self::SnarkProve => "snark_proof",
        // };
        format!("{prefix}/{task_id}")
    }
}

