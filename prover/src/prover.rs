use anyhow::Result;

pub trait Prover<T> {
    fn prove(&self, ctx: &T) -> Result<()>;
}