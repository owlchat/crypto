use std::{io, path::PathBuf};

use sha2::{Digest, Sha256};

pub fn sha256_hash_file(path: PathBuf) -> io::Result<[u8; 32]> {
    let bytes = std::fs::read(path)?;
    Ok(sha256_hash_bytes(&bytes))
}

pub fn sha256_hash_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
