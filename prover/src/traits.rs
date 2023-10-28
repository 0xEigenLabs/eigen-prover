pub trait Executor {
    fn execute(&self, basedir: &str, task_id: &str) -> algebraic::errors::Result<()>;
}
