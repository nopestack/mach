use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::api::{ApiError, SharedServerState};
use crate::exec::task::{Task, TaskResult};
use crate::exec::TaskExecutor;
use crate::storage::FnStorage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallRequest {
    pub id: uuid::Uuid,
    pub args: Vec<u8>,
}

#[tracing::instrument]
pub async fn call_handler<F, T>(
    State(state): State<SharedServerState<F, T>>,
    req: Json<CallRequest>,
) -> Result<Json<TaskResult>, ApiError>
where
    F: FnStorage + 'static,
    T: TaskExecutor + 'static,
{
    let mut state = state.write().await;
    let module = state.storage_backend.load(&req.id).await?;

    let task = Task {
        id: uuid::Uuid::new_v4(),
        module,
    };

    let result = state.task_exec.exec(task)?;

    Ok(Json(result))
}
