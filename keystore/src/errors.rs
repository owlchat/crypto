use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyStoreError {
    #[error(transparent)]
    AeadError(#[from] aes_gcm_siv::aead::Error),
    #[error(transparent)]
    Bip39Error(#[from] bip39::ErrorKind),
    #[error("Empty Seed Provided")]
    EmptySeed,
}
