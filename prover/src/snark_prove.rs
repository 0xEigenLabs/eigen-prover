use crate::traits::Executor;
use algebraic::errors::Result;
use starky::{compressor12_exec::exec, compressor12_setup::setup, zkin_join::join_zkin};

pub struct SnarkProver {}
impl SnarkProver {
    pub fn new() -> Self { SnarkProver{} }
}

impl Executor for SnarkProver {
    fn execute(&self, basedir: &str, task_id: &str) -> Result<()> {
        log::info!("start snark prove");
        // snark_verifier.sh

        log::info!("end snark prove");
        Ok(())
    }
}
