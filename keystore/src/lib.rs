use std::convert::TryFrom;

use aes::Aes256;
use bip39::Mnemonic;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{rngs::OsRng, Rng};
use x25519_dalek::{PublicKey, StaticSecret as SecretKey};
use zeroize::Zeroize;

mod errors;
pub use errors::KeyStoreError;

// Create an alias for convenience
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

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
        let seed = self.seed.or(seed).ok_or_else(|| KeyStoreError::EmptySeed)?;
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

    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, KeyStoreError> {
        let mut sk = self.sk.to_bytes();
        let encrypted = self.encrypt_with(sk, data)?;
        sk.zeroize();
        Ok(encrypted)
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, KeyStoreError> {
        let mut sk = self.sk.to_bytes();
        let decrypted = self.decrypt_with(sk, data)?;
        sk.zeroize();
        Ok(decrypted)
    }

    pub fn encrypt_with(&self, mut sk: [u8; 32], data: Vec<u8>) -> Result<Vec<u8>, KeyStoreError> {
        let mut rnd = OsRng::default();
        let mut iv = [0u8; 16];
        rnd.fill(&mut iv);
        let mut encrypted = aes_encrypt(&sk, &iv, &data)?;
        encrypted.extend_from_slice(&iv);
        sk.zeroize();
        Ok(encrypted)
    }

    pub fn decrypt_with(&self, mut sk: [u8; 32], data: Vec<u8>) -> Result<Vec<u8>, KeyStoreError> {
        let len = data.len();
        let (data, iv) = data.split_at(len - 16);
        let decrypted = aes_decrypt(&sk, iv, data)?;
        sk.zeroize();
        Ok(decrypted)
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

fn aes_decrypt(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, KeyStoreError> {
    let cipher = Aes256Cbc::new_var(&key, &iv)?;
    cipher.decrypt_vec(input).map_err(Into::into)
}

fn aes_encrypt(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, KeyStoreError> {
    let cipher = Aes256Cbc::new_var(&key, &iv)?;
    Ok(cipher.encrypt_vec(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ks = KeyStore::new();
        let data = b"Owlchat".to_vec();
        let encrypted = ks.encrypt(data.clone()).expect("ecnrypt");
        let decrypted = ks.decrypt(encrypted).expect("decrypt");
        assert_eq!(decrypted, data);
    }

    #[test]
    fn keystore_init() {
        let ks = KeyStore::new();
        let data = b"Owlchat".to_vec();
        let encrypted = ks.encrypt(data.clone()).unwrap();
        let sk = ks.secret_key();
        drop(ks);
        let ks = KeyStore::init(sk);
        let decrypted = ks.decrypt(encrypted).unwrap();
        assert_eq!(decrypted, data);
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

        let m0 = b"Knock, knock".to_vec();
        let alice_to_bob = alice_ks.encrypt_with(alice_sk, m0.clone()).unwrap();
        let message = bob_ks.decrypt_with(bob_sk, alice_to_bob).unwrap();
        assert_eq!(message, m0);

        let m1 = b"Who's there?".to_vec();
        let bob_to_alice = bob_ks.encrypt_with(bob_sk, m1.clone()).unwrap();
        let message = alice_ks.decrypt_with(alice_sk, bob_to_alice).unwrap();
        assert_eq!(message, m1);
    }

    #[test]
    fn backup_and_restore() {
        let ks = KeyStore::new();
        let paper_key = ks.backup(None).unwrap();
        println!("Backup Paper Key: {}", paper_key);
        let data = b"Owlchat".to_vec();
        let encrypted = ks.encrypt(data.clone()).unwrap();
        drop(ks);

        let ks = KeyStore::restore(paper_key).unwrap();
        let decrypted = ks.decrypt(encrypted).unwrap();

        assert_eq!(decrypted, data);
    }
}
