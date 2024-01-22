pub mod backends;
pub mod error;
pub mod fn_storage;
pub mod test_utils;
pub use fn_storage::*;

use sha3::Digest;

pub const DATA_DIR: &str = ".data";
pub const UPLOADS_DIR: &str = ".data/uploads";

pub fn hash_fn_content(data: &[u8]) -> Vec<u8> {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
