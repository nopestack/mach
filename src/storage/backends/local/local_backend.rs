use crate::storage::{FnEntry, FnStorage};

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

    async fn load(&self, name: &str) -> Result<Vec<u8>> {
        // TODO: read fn path from db
        // return fn file read

        let name = suffix_wasm_if_needed(name);

        let found = tokio::fs::try_exists(&name)
            .await
            .map_err(|_| StorageError::NotFound(name.clone()))?;

        if !found {
            return Err(StorageError::NotFound(name));
        }

        let file_data = tokio::fs::read(self.path.join(name))
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

    async fn delete(&self, name: &str) -> Result<()> {
        let found = tokio::fs::try_exists(name)
            .await
            .map_err(|_| StorageError::NotFound(name.to_string()))?;

        if !found {
            return Err(StorageError::NotFound(name.to_string()));
        }

        todo!()
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
