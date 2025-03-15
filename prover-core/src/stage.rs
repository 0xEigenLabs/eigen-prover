use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Stage of the proof
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stage {
    Batch(String, String, String),     // task_key, chunk_id, l2_batch_data
    Aggregate(String, String, String), // task_key, input, input2
    Final(String, String, String),     // task_key, curve, prover_addr
}

impl Stage {
    /// get the path of the stage
    pub fn path(&self) -> String {
        match self {
            Self::Batch(task_id, _, _) => {
                format!("proof/{task_id}/batch_proof")
            }
            Self::Aggregate(task_id, _, _) => format!("proof/{task_id}/agg_proof"),
            Self::Final(task_id, _, _) => format!("proof/{task_id}/snark_proof"),
        }
    }

    /// keep track of task status
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_batch_stage_path() {
        let stage = Stage::Batch(
            "task_id".to_string(),
            "chunk_id".to_string(),
            "".to_string(),
        );
        assert_eq!(stage.path(), "proof/task_id/batch_proof_chunk_id");
    }

    #[test]
    fn test_agg_stage_path() {
        let stage = Stage::Aggregate(
            "task_id".to_string(),
            "input".to_string(),
            "input2".to_string(),
        );
        assert_eq!(stage.path(), "proof/task_id/agg_proof");
    }

    #[test]
    fn test_final_stage_path() {
        let stage = Stage::Final(
            "task_id".to_string(),
            "curve".to_string(),
            "prover_addr".to_string(),
        );
        assert_eq!(stage.path(), "proof/task_id/snark_proof");
    }

    #[test]
    fn test_stage_to_string() {
        let stage = Stage::Batch(
            "task_id".to_string(),
            "chunk_id".to_string(),
            "".to_string(),
        );
        assert_eq!(stage.to_string().unwrap(), r#"["task_id","chunk_id",""]"#);
    }
}
