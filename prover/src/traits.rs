use crate::Context;
use algebraic::errors::Result;

pub trait Executor {
    fn execute(&self, ctx: &Context) -> Result<()>;
}
