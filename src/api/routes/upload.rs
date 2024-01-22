use axum::extract::{Multipart, State};

use crate::{
    api::{ApiError, SharedServerState},
    exec::TaskExecutor,
    storage::{hash_fn_content, FnEntry, FnStorage},
};

use axum::{body::Bytes, BoxError};
use futures::{Stream, TryStreamExt};
use std::{io, path::Path};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

/// Handler that accepts a multipart form upload and streams each field to a file.
#[tracing::instrument]
pub async fn upload_handler<F, T>(
    State(state): State<SharedServerState<F, T>>,
    mut multipart: Multipart,
) -> Result<(), ApiError>
where
    F: FnStorage + 'static,
    T: TaskExecutor + 'static,
{
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        let storage_path = state.read().await.storage_backend.path().to_owned();

        let fn_id = uuid::Uuid::new_v4();
        let fn_name = "test";

        let fn_hash = stream_to_file(&storage_path, &file_name, field).await?;

        let fn_path = storage_path.join(file_name);

        let fn_entry = FnEntry {
            id: fn_id,
            name: fn_name.to_string(),
            path: fn_path,
            hash: fn_hash,
        };

        state.write().await.storage_backend.save(fn_entry).await?;
    }

    Ok(())
}

async fn stream_to_file<S, E>(path: &Path, file_name: &str, stream: S) -> anyhow::Result<Vec<u8>>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    // TODO: revisit
    // if !path_is_valid(path) {
    //     anyhow::bail!("Invalid path");
    // }

    let path = path.join(file_name);

    // Convert the stream into an `AsyncRead`.
    let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // Create the file. `File` implements `AsyncWrite`.
    let file = File::create(path).await?;
    let mut file_buffer = BufWriter::new(file);

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file_buffer).await?;

    let data_hash = hash_fn_content(file_buffer.buffer());

    Ok(data_hash)
}

fn path_is_valid(path: &Path) -> bool {
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
