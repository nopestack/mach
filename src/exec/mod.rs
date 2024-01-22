use crate::exec::task::{Task, TaskResult};

pub mod task;
pub mod task_processor;
pub mod wasm_task_exec;

pub trait TaskExecutor: std::fmt::Debug + Send + Sync {
    fn exec(&mut self, task: Task) -> anyhow::Result<TaskResult>;
}
