use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    pub basedir: String,
    pub l2_batch_data: String,
    pub task_id: String,
    pub task_name: String,

    pub elf_path: String,
}

impl BatchContext {
    pub fn new(
        basedir: &str,
        task_id: &str,
        task_name: &str,
        l2_batch_data: String,
        elf_path: &str,
    ) -> Self {
        BatchContext {
            basedir: basedir.to_string(),
            l2_batch_data,
            task_id: task_id.to_string(),

            task_name: task_name.to_string(),

            elf_path: elf_path.to_string(),
        }
    }
}
