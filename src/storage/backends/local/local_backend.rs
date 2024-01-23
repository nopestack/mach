use crate::storage::{FnEntry, FnId, FnStorage};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::storage::error::StorageError;
use crate::storage::Result;

// TODO: move to common
// pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct LocalBackend {
    path: PathBuf,
    db: sled::Db,
}

impl LocalBackend {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&path).map_err(|err| StorageError::Other(err.to_string()))?;
        tracing::info!("storage base dir: {}", path.display());

        let db = sled::open(path.join("db"))?;

        Ok(Self { path, db })
    }

    async fn find(&self, fn_id: &FnId) -> Result<FnEntry> {
        let found = self
            .db
            .get(fn_id)
            .map_err(|err| StorageError::NotFound(err.to_string()))?
            .ok_or(StorageError::NotFound(fn_id.to_string()))?;

        let fn_entry = bincode::deserialize::<FnEntry>(&found.to_vec())
            .map_err(|err| StorageError::Other(err.to_string()))?;

        Ok(fn_entry)
    }

    async fn delete_entry(&mut self, fn_id: &FnId) -> Result<()> {
        let removed = self
            .db
            .remove(fn_id)
            .map_err(|err| StorageError::NotFound(err.to_string()))?
            .ok_or(StorageError::NotFound(fn_id.to_string()))?;

        let fn_entry = bincode::deserialize::<FnEntry>(&removed.to_vec())
            .map_err(|err| StorageError::Other(err.to_string()))?;

        tokio::fs::remove_file(self.path.join(fn_entry.path))
            .await
            .map_err(|err| StorageError::Other(err.to_string()))?;

        Ok(())
    }
}

fn suffix_wasm_if_needed(name: &str) -> String {
    if name.ends_with(".wasm") {
        name.to_string()
    } else {
        format!("{}.wasm", name)
    }
}

impl FnStorage for LocalBackend {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    async fn load(&self, fn_id: &uuid::Uuid) -> Result<Vec<u8>> {
        let fn_entry = self.find(fn_id).await?;

        let file_data = tokio::fs::read(self.path.join(fn_entry.path))
            .await
            .map_err(|err| StorageError::Other(err.to_string()))?;

        Ok(file_data)
    }

    async fn save(&self, fn_entry: FnEntry) -> Result<()> {
        let fn_id = fn_entry.id;
        let fn_entry =
            bincode::serialize(&fn_entry).map_err(|err| StorageError::Other(err.to_string()))?;

        self.db
            .insert(fn_id, fn_entry)
            .map_err(|err| StorageError::Other(err.to_string()))?;

        Ok(())
    }

    async fn delete(&mut self, fn_id: &FnId) -> Result<()> {
        self.delete_entry(fn_id).await?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<FnEntry>> {
        let fn_entries = self
            .db
            .iter()
            .filter_map(|res| {
                let (_, entry) = res
                    .map_err(|err| StorageError::Other(err.to_string()))
                    .ok()?;

                bincode::deserialize::<FnEntry>(&entry)
                    .map_err(|err| StorageError::Other(err.to_string()))
                    .ok()
            })
            .collect::<Vec<FnEntry>>();

        Ok(fn_entries)
    }
}
