use axum::response::IntoResponse;
use hyper::StatusCode;
use serde_json::json;

pub async fn not_found_handler() -> impl IntoResponse {
    let error_string = json!({
        "error": StatusCode::NOT_FOUND.as_u16(),
        "message": "Not Found",
    })
    .to_string();

    (StatusCode::NOT_FOUND, error_string)
}
