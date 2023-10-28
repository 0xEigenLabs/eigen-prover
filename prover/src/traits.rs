use crate::{AggContext, BatchContext, FinalContext};
use algebraic::errors::Result;

pub trait StageProver {
    fn batch_prove(&self, _ctx: &BatchContext) -> Result<()> {
        Ok(())
    }
    fn agg_prove(&self, _ctx: &AggContext) -> Result<()> {
        Ok(())
    }
    fn final_prove(&self, _ctx: &FinalContext) -> Result<()> {
        Ok(())
    }
}
