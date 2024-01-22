use axum::extract::{Path, State};

use crate::{
    api::{ApiError, SharedServerState},
    exec::TaskExecutor,
    storage::FnStorage,
};

#[tracing::instrument]
pub async fn delete_handler<F, T>(
    Path(fn_id): Path<String>,
    State(state): State<SharedServerState<F, T>>,
) -> Result<(), ApiError>
where
    F: FnStorage + 'static,
    T: TaskExecutor + 'static,
{
    state.write().await.storage_backend.delete(&fn_id).await?;

    Ok(())
}
