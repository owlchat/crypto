use std::convert::TryFrom;

use aes_gcm_siv::aead::{AeadInPlace, Buffer, NewAead};
use aes_gcm_siv::Aes256GcmSiv;

use bip39::Mnemonic;
use rand::{rngs::OsRng, Rng};
use x25519_dalek::{PublicKey, StaticSecret as SecretKey};
use zeroize::Zeroize;

mod errors;
pub mod util;
pub use errors::KeyStoreError;
mod buffer;
pub use buffer::SharedBuffer;

/// A Simple KeyStore that holds Keypair ([`PublicKey`], [`SecretKey`]).
/// Also it holds the `Seed` used to generate the [`SecretKey`]
pub struct KeyStore {
    pk: PublicKey,
    sk: SecretKey,
    seed: Option<[u8; 32]>,
}

impl KeyStore {
    /// Create a new `KeyStore`.
    /// ### Note
    /// After creating a new `KeyStore` you should call [`KeyStore::secret_key`] to get your [`SecretKey`]
    /// and [`KeyStore::seed`] to get the `Seed` used in creating that private keys.
    /// Those two `[u8; u32]` arrays should be stored securly in the device [`iOS KeyChain`][1] or [`Android KeyStore`][2].
    ///
    /// [1]: https://developer.apple.com/documentation/security/keychain_services
    /// [2]: https://developer.android.com/training/articles/keystore.html
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        let mut rnd = OsRng::default();
        rnd.fill(&mut seed);
        let sk = SecretKey::from(seed);
        let pk = PublicKey::from(&sk);
        let ks = Self {
            pk,
            sk,
            seed: Some(seed),
        };
        seed.zeroize();
        ks
    }

    /// Init the `KeyStore` with existing SecretKey Bytes.
    /// ### Note
    /// The created `KeyStore` dose not contains any seed.
    pub fn init(mut secret_key: [u8; 32]) -> Self {
        let sk = SecretKey::from(secret_key); // copy
        let pk = PublicKey::from(&sk);
        // so we zeroize the last copy here before dropping it.
        secret_key.zeroize();
        Self { pk, sk, seed: None }
    }

    /// Get your [`PublicKey`] as bytes.
    pub fn public_key(&self) -> [u8; 32] {
        self.pk.to_bytes()
    }

    /// Get your [`SecretKey`] as bytes.
    pub fn secret_key(&self) -> [u8; 32] {
        self.sk.to_bytes()
    }

    /// Get your `Seed` as bytes (if any).
    ///
    /// ### Note
    /// Only Avaiable for a newly created `KeyStore`.
    pub fn seed(&self) -> Option<[u8; 32]> {
        self.seed
    }

    /// Create a [`Mnemonic`] Backup from the provided seed as `String`.
    ///
    /// if this a newly created `KeyStroe` you could pass `None` since it will use the current seed.
    /// it will return Error if both the current seed and the provided one is both `None`.
    pub fn backup(&self, seed: Option<[u8; 32]>) -> Result<String, KeyStoreError> {
        let seed = self.seed.or(seed).ok_or(KeyStoreError::EmptySeed)?;
        let mnemonic = Mnemonic::from_entropy(&seed)?;
        Ok(mnemonic.to_string())
    }

    /// Restore a `KeyStore` from a [`Mnemonic`] Paper Backup.
    ///
    /// The new `KeyStore` will also contian the `Seed` used to create the [`SecretKey`].
    /// See [`KeyStore::new`] for the following steps after creating a new KeyStore.
    pub fn restore(paper_key: String) -> Result<Self, KeyStoreError> {
        let mnemonic = Mnemonic::parse(paper_key)?;
        let entropy = mnemonic.to_entropy();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&entropy);
        let sk = SecretKey::from(seed);
        let pk = PublicKey::from(&sk);
        let ks = Self {
            pk,
            sk,
            seed: Some(seed),
        };
        seed.zeroize();
        Ok(ks)
    }

    /// Perform a Diffie-Hellman key agreement to produce a `SharedSecret`.
    pub fn dh(&self, their_public: [u8; 32]) -> [u8; 32] {
        let their_public = PublicKey::from(their_public);
        let shared_secret = self.sk.diffie_hellman(&their_public);
        shared_secret.to_bytes()
    }

    pub fn encrypt<B: Buffer>(&self, data: &mut B) -> Result<(), KeyStoreError> {
        let mut sk = self.sk.to_bytes();
        self.encrypt_with(sk, data)?;
        sk.zeroize();
        Ok(())
    }

    pub fn decrypt<B: Buffer>(&self, data: &mut B) -> Result<(), KeyStoreError> {
        let mut sk = self.sk.to_bytes();
        self.decrypt_with(sk, data)?;
        sk.zeroize();
        Ok(())
    }

    pub fn encrypt_with<B: Buffer>(
        &self,
        mut sk: [u8; 32],
        data: &mut B,
    ) -> Result<(), KeyStoreError> {
        let mut rnd = OsRng::default();
        let mut nonce = [0u8; 12];
        rnd.fill(&mut nonce);
        aes_encrypt(&sk, &nonce, data)?;
        data.extend_from_slice(&nonce)?;
        sk.zeroize();
        Ok(())
    }

    pub fn decrypt_with<B: Buffer>(
        &self,
        mut sk: [u8; 32],
        data: &mut B,
    ) -> Result<(), KeyStoreError> {
        const NONCE_LEN: usize = 12;
        if data.len() < NONCE_LEN {
            return Err(KeyStoreError::AeadError(aes_gcm_siv::aead::Error));
        }
        let mut nonce = [0u8; NONCE_LEN];
        let other = data.as_ref().iter().rev().take(NONCE_LEN);
        nonce.iter_mut().rev().zip(other).for_each(|(v, b)| *v = *b);
        // remove the nonce, we got it now.
        data.truncate(data.as_ref().len() - NONCE_LEN);
        aes_decrypt(&sk, &nonce, data)?;
        sk.zeroize();
        Ok(())
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for KeyStore {
    fn drop(&mut self) {
        self.seed.zeroize();
    }
}

impl From<[u8; 32]> for KeyStore {
    fn from(mut sk: [u8; 32]) -> Self {
        let ks = Self::init(sk);
        sk.zeroize();
        ks
    }
}

impl TryFrom<String> for KeyStore {
    type Error = KeyStoreError;
    fn try_from(paper_key: String) -> Result<Self, Self::Error> {
        Self::restore(paper_key)
    }
}

fn aes_decrypt<B: Buffer>(key: &[u8], nonce: &[u8], data: &mut B) -> Result<(), KeyStoreError> {
    let cipher = Aes256GcmSiv::new(key.into());
    cipher
        .decrypt_in_place(nonce.into(), b"", data)
        .map_err(Into::into)
}

fn aes_encrypt<B: Buffer>(key: &[u8], nonce: &[u8], data: &mut B) -> Result<(), KeyStoreError> {
    let cipher = Aes256GcmSiv::new(key.into());
    cipher
        .encrypt_in_place(nonce.into(), b"", data)
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ks = KeyStore::new();
        let mut data = Vec::with_capacity((8 + 12) * 4);
        data.extend_from_slice(b"Owlchat");
        ks.encrypt(&mut data).expect("ecnrypt");
        let original = b"Owlchat".to_vec();
        assert_ne!(data, original);
        ks.decrypt(&mut data).expect("decrypt");
        assert_eq!(data, original);
    }

    #[test]
    fn keystore_init() {
        let ks = KeyStore::new();
        let mut data = Vec::with_capacity((8 + 12) * 4);
        data.extend_from_slice(b"Owlchat");
        let original = b"Owlchat".to_vec();
        ks.encrypt(&mut data).unwrap();
        let sk = ks.secret_key();
        drop(ks);
        let ks = KeyStore::init(sk);
        ks.decrypt(&mut data).unwrap();
        assert_eq!(data, original);
    }
    #[test]
    fn same_shared_secret() {
        let alice_ks = KeyStore::new();
        let bob_ks = KeyStore::new();

        let alice_sk = alice_ks.dh(bob_ks.public_key());
        let bob_sk = bob_ks.dh(alice_ks.public_key());
        assert_eq!(alice_sk, bob_sk);
    }

    #[test]
    fn funny_conversation() {
        let alice_ks = KeyStore::new();
        let bob_ks = KeyStore::new();

        let alice_sk = alice_ks.dh(bob_ks.public_key());
        let bob_sk = bob_ks.dh(alice_ks.public_key());

        let mut m0 = Vec::with_capacity((12 + 12) * 4);
        m0.extend_from_slice(b"Knock, knock");
        let original = b"Knock, knock".to_vec();
        alice_ks.encrypt_with(alice_sk, &mut m0).unwrap();
        bob_ks.decrypt_with(bob_sk, &mut m0).unwrap();
        assert_eq!(original, m0);

        let mut m1 = Vec::with_capacity((12 + 12) * 4);
        m1.extend_from_slice(b"Who's there?");
        let original = b"Who's there?".to_vec();
        bob_ks.encrypt_with(bob_sk, &mut m1).unwrap();
        alice_ks.decrypt_with(alice_sk, &mut m1).unwrap();
        assert_eq!(original, m1);
    }

    #[test]
    fn backup_and_restore() {
        let ks = KeyStore::new();
        let paper_key = ks.backup(None).unwrap();
        println!("Backup Paper Key: {}", paper_key);
        let mut data = Vec::with_capacity((8 + 12) * 4);
        data.extend_from_slice(b"Owlchat");
        let original = b"Owlchat".to_vec();
        ks.encrypt(&mut data).unwrap();
        drop(ks);

        let ks = KeyStore::restore(paper_key).unwrap();
        ks.decrypt(&mut data).unwrap();
        assert_eq!(original, data);
    }
}
