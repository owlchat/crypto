use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyStoreError {
    #[error(transparent)]
    InvalidKeyIvLength(#[from] block_modes::InvalidKeyIvLength),
    #[error(transparent)]
    BlockModeError(#[from] block_modes::BlockModeError),
    #[error(transparent)]
    Bip39Error(#[from] bip39::Error),
    #[error("Empty Seed Provided")]
    EmptySeed,
}
