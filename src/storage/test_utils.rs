use std::{fs::File, io::Read, path::PathBuf};

use super::backends::local::LocalBackend;

pub fn read_file_to_string(path: &str) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_to_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

pub const TEST_WASM_NAME: &str = "hello.wasm";

pub async fn setup_mock_local_storage_backend() -> LocalBackend {
    let tmp_dir = tempdir::TempDir::new("evaly_fn_store").unwrap().into_path();
    LocalBackend::new(tmp_dir).unwrap()
}

pub async fn load_test_wasm_module() -> Vec<u8> {
    let path = PathBuf::from("tests/fixtures/").join(TEST_WASM_NAME);
    read_file_to_bytes(path.to_str().unwrap()).unwrap()
}
