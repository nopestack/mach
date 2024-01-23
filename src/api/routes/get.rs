use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{ApiError, SharedServerState},
    exec::TaskExecutor,
    storage::FnStorage,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResponse {
    pub name: String,
    pub fn_data: Vec<u8>,
}

#[tracing::instrument]
pub async fn get_handler<F, T>(
    Path(fn_id): Path<uuid::Uuid>,
    State(state): State<SharedServerState<F, T>>,
) -> Result<Json<GetResponse>, ApiError>
where
    F: FnStorage + 'static,
    T: TaskExecutor + 'static,
{
    let fn_data = state.read().await.storage_backend.load(&fn_id).await?;

    Ok(Json(GetResponse {
        name: fn_id.to_string(),
        fn_data,
    }))
}
