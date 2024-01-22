use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum StorageError {
    #[error("{0} not found")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for StorageError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}
