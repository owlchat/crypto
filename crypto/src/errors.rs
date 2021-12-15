use chacha20poly1305::aead;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyPairError {
    #[error(transparent)]
    AeadError(#[from] aead::Error),
    #[error(transparent)]
    Bip39Error(#[from] bip39::ErrorKind),
    #[error("Empty Seed Provided")]
    EmptySeed,
}
