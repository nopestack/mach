use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use super::error::StorageError;

pub type Result<T> = std::result::Result<T, StorageError>;

pub type FnId = uuid::Uuid;

#[allow(async_fn_in_trait)]
#[trait_variant::make(FnStorage: Send)]
pub trait LocalFnStorage: std::fmt::Debug {
    async fn list(&self) -> Result<Vec<FnEntry>>;
    async fn load(&self, fn_id: &FnId) -> Result<Vec<u8>>;
    async fn save(&self, fn_entry: FnEntry) -> Result<()>;
    async fn delete(&mut self, fn_id: &FnId) -> Result<()>;
    fn path(&self) -> &PathBuf;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnEntry {
    /// Unique identifier for the function
    pub id: FnId,
    /// Display name for the function
    pub name: String,
    /// Path to the function file
    pub path: PathBuf,
    /// Cecksum of the function file
    pub hash: Vec<u8>,
}
