use axum::response::IntoResponse;

#[tracing::instrument]
pub async fn root() -> impl IntoResponse {
    "Hello, World!"
}
