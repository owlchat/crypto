use std::{io, path::PathBuf};

use sha2::{Digest, Sha256};

pub fn sh256_hash(path: PathBuf) -> io::Result<[u8; 32]> {
    let bytes = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    Ok(hash)
}
