use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    api::{ApiError, SharedServerState},
    exec::TaskExecutor,
    storage::{FnEntry, FnStorage},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub functions: Vec<FnEntry>,
}

#[tracing::instrument]
pub async fn list_handler<F, T>(
    State(state): State<SharedServerState<F, T>>,
) -> Result<Json<ListResponse>, ApiError>
where
    F: FnStorage + 'static,
    T: TaskExecutor + 'static,
{
    let functions = state.read().await.storage_backend.list().await?;

    Ok(Json(ListResponse { functions }))
}
