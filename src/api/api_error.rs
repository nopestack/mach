use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::storage::error::StorageError;

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    #[error("fn {0} called with invalid arguments")]
    InvalidArguments(String),

    #[error("storage errror {0}")]
    Storage(#[from] StorageError),

    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InvalidArguments(err) => {
                let error_string = json!({
                    "error": StatusCode::BAD_REQUEST.as_u16(),
                    "message": err.to_string(),
                })
                .to_string();

                (StatusCode::BAD_REQUEST, error_string).into_response()
            }

            ApiError::Storage(err) => handle_storage_error(err),

            _ => {
                let error_string = json!({
                    "error": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    "message": "internal server error",
                })
                .to_string();

                (StatusCode::INTERNAL_SERVER_ERROR, error_string).into_response()
            }
        }
    }
}

fn handle_storage_error(err: StorageError) -> Response {
    match err {
        StorageError::NotFound(err) => {
            let error_string = json!({
                "error": StatusCode::NOT_FOUND.as_u16(),
                "message": err.to_string(),
            })
            .to_string();

            (StatusCode::NOT_FOUND, error_string).into_response()
        }
        _ => {
            let error_string = json!({
                "error": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "internal server error",
            })
            .to_string();

            (StatusCode::INTERNAL_SERVER_ERROR, error_string).into_response()
        }
    }
}
