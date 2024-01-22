use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub module: Vec<u8>, // TODO: change to bytevec of wasm module
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub stdout: String,
    pub stderr: String,
    /// Represents the execution time of the task in milliseconds
    pub execution_time: u64,
}
