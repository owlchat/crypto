use std::convert::TryFrom;

use aes_gcm_siv::aead::{AeadInPlace, Buffer, NewAead};
use aes_gcm_siv::Aes256GcmSiv;
use bip39::{Language, Mnemonic, MnemonicType};
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Digest, Sha512};
use subtle::ConstantTimeEq;
use x25519_dalek::{PublicKey, StaticSecret as SecretKey};
use zeroize::Zeroize;

pub const AGREEMENT_LENGTH: usize = 32;
pub const SEED_LENGTH: usize = 32;
pub const PRIVATE_KEY_LENGTH: usize = 32;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const SIGNATURE_LENGTH: usize = 64;

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
    seed: Option<[u8; SEED_LENGTH]>,
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
        let mut seed = [0u8; SEED_LENGTH];
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
    pub fn init(mut secret_key: [u8; PRIVATE_KEY_LENGTH]) -> Self {
        let sk = SecretKey::from(secret_key); // copy
        let pk = PublicKey::from(&sk);
        // so we zeroize the last copy here before dropping it.
        secret_key.zeroize();
        Self { pk, sk, seed: None }
    }

    /// Get your [`PublicKey`] as bytes.
    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        self.pk.to_bytes()
    }

    /// Get your [`SecretKey`] as bytes.
    pub fn secret_key(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.sk.to_bytes()
    }

    /// Get your `Seed` as bytes (if any).
    ///
    /// ### Note
    /// Only Avaiable for a newly created `KeyStore`.
    pub fn seed(&self) -> Option<[u8; SEED_LENGTH]> {
        self.seed
    }

    /// Create a [`Mnemonic`] Backup from the provided seed as `String`.
    ///
    /// if this a newly created `KeyStroe` you could pass `None` since it will use the current seed.
    /// it will return Error if both the current seed and the provided one is both `None`.
    pub fn backup(&self, seed: Option<[u8; SEED_LENGTH]>) -> Result<String, KeyStoreError> {
        let seed = self.seed.or(seed).ok_or(KeyStoreError::EmptySeed)?;
        let mnemonic = Mnemonic::from_entropy(&seed, Language::English).map_err(|_| {
            KeyStoreError::Bip39Error(bip39::ErrorKind::InvalidEntropyLength(
                32,
                MnemonicType::Words12,
            ))
        })?;
        Ok(mnemonic.to_string())
    }

    /// Restore a `KeyStore` from a [`Mnemonic`] Paper Backup.
    ///
    /// The new `KeyStore` will also contian the `Seed` used to create the [`SecretKey`].
    /// See [`KeyStore::new`] for the following steps after creating a new KeyStore.
    pub fn restore(paper_key: String) -> Result<Self, KeyStoreError> {
        let mnemonic = Mnemonic::from_phrase(&paper_key, Language::English)
            .map_err(|_| KeyStoreError::Bip39Error(bip39::ErrorKind::InvalidWord))?;
        let entropy = mnemonic.entropy();
        let mut seed = [0u8; SEED_LENGTH];
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
    pub fn dh(&self, their_public: [u8; PUBLIC_KEY_LENGTH]) -> [u8; AGREEMENT_LENGTH] {
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
        mut sk: [u8; PRIVATE_KEY_LENGTH],
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
        mut sk: [u8; PRIVATE_KEY_LENGTH],
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

    /// Calculates an XEdDSA signature using the X25519 private key directly.
    ///
    /// Refer to https://signal.org/docs/specifications/xeddsa/#curve25519 for more details.
    ///
    /// Note that this implementation varies slightly from that paper in that the sign bit is not
    /// fixed to 0, but rather passed back in the most significant bit of the signature which would
    /// otherwise always be 0. This is for compatibility with the implementation found in
    /// libsignal-protocol-java.
    pub fn calculate_signature(&self, message: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        let mut csprng = OsRng::default();
        let mut random_bytes = [0u8; 64];
        csprng.fill(&mut random_bytes);

        let a = Scalar::from_bits(self.secret_key());
        let ed_public_key_point = &a * &ED25519_BASEPOINT_TABLE;
        let ed_public_key = ed_public_key_point.compress();
        let sign_bit = ed_public_key.as_bytes()[31] & 0b1000_0000_u8;

        let mut hash1 = Sha512::new();
        let hash_prefix = [
            0xFEu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
            0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        ];
        hash1.update(&hash_prefix);
        hash1.update(&self.secret_key());
        hash1.update(&message);
        hash1.update(&random_bytes[..]);

        let r = Scalar::from_hash(hash1);
        let cap_r = (&r * &ED25519_BASEPOINT_TABLE).compress();

        let mut hash = Sha512::new();
        hash.update(cap_r.as_bytes());
        hash.update(ed_public_key.as_bytes());
        hash.update(&message);

        let h = Scalar::from_hash(hash);
        let s = (h * a) + r;

        let mut result = [0u8; SIGNATURE_LENGTH];
        result[..32].copy_from_slice(cap_r.as_bytes());
        result[32..].copy_from_slice(s.as_bytes());
        result[SIGNATURE_LENGTH - 1] &= 0b0111_1111_u8;
        result[SIGNATURE_LENGTH - 1] |= sign_bit;
        result
    }

    pub fn verify_signature(
        their_public_key: [u8; PUBLIC_KEY_LENGTH],
        message: &[u8],
        signature: [u8; SIGNATURE_LENGTH],
    ) -> bool {
        let mont_point = MontgomeryPoint(their_public_key);
        let ed_pub_key_point =
            match mont_point.to_edwards((signature[SIGNATURE_LENGTH - 1] & 0b1000_0000_u8) >> 7) {
                Some(x) => x,
                None => return false,
            };
        let cap_a = ed_pub_key_point.compress();
        let mut cap_r = [0u8; 32];
        cap_r.copy_from_slice(&signature[..32]);
        let mut s = [0u8; 32];
        s.copy_from_slice(&signature[32..]);
        s[31] &= 0b0111_1111_u8;
        if (s[31] & 0b1110_0000_u8) != 0 {
            return false;
        }
        let minus_cap_a = -ed_pub_key_point;

        let mut hash = Sha512::new();
        hash.update(&cap_r);
        hash.update(cap_a.as_bytes());
        hash.update(&message);
        let h = Scalar::from_hash(hash);

        let cap_r_check_point = EdwardsPoint::vartime_double_scalar_mul_basepoint(
            &h,
            &minus_cap_a,
            &Scalar::from_bits(s),
        );
        let cap_r_check = cap_r_check_point.compress();

        bool::from(cap_r_check.as_bytes().ct_eq(&cap_r))
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

    #[test]
    fn agreement() {
        let alice_public: [u8; 32] = [
            0x1b, 0xb7, 0x59, 0x66, 0xf2, 0xe9, 0x3a, 0x36, 0x91, 0xdf, 0xff, 0x94, 0x2b, 0xb2,
            0xa4, 0x66, 0xa1, 0xc0, 0x8b, 0x8d, 0x78, 0xca, 0x3f, 0x4d, 0x6d, 0xf8, 0xb8, 0xbf,
            0xa2, 0xe4, 0xee, 0x28,
        ];
        let alice_private: [u8; 32] = [
            0xc8, 0x06, 0x43, 0x9d, 0xc9, 0xd2, 0xc4, 0x76, 0xff, 0xed, 0x8f, 0x25, 0x80, 0xc0,
            0x88, 0x8d, 0x58, 0xab, 0x40, 0x6b, 0xf7, 0xae, 0x36, 0x98, 0x87, 0x90, 0x21, 0xb9,
            0x6b, 0xb4, 0xbf, 0x59,
        ];
        let bob_public: [u8; 32] = [
            0x65, 0x36, 0x14, 0x99, 0x3d, 0x2b, 0x15, 0xee, 0x9e, 0x5f, 0xd3, 0xd8, 0x6c, 0xe7,
            0x19, 0xef, 0x4e, 0xc1, 0xda, 0xae, 0x18, 0x86, 0xa8, 0x7b, 0x3f, 0x5f, 0xa9, 0x56,
            0x5a, 0x27, 0xa2, 0x2f,
        ];
        let bob_private: [u8; 32] = [
            0xb0, 0x3b, 0x34, 0xc3, 0x3a, 0x1c, 0x44, 0xf2, 0x25, 0xb6, 0x62, 0xd2, 0xbf, 0x48,
            0x59, 0xb8, 0x13, 0x54, 0x11, 0xfa, 0x7b, 0x03, 0x86, 0xd4, 0x5f, 0xb7, 0x5d, 0xc5,
            0xb9, 0x1b, 0x44, 0x66,
        ];
        let shared: [u8; 32] = [
            0x32, 0x5f, 0x23, 0x93, 0x28, 0x94, 0x1c, 0xed, 0x6e, 0x67, 0x3b, 0x86, 0xba, 0x41,
            0x01, 0x74, 0x48, 0xe9, 0x9b, 0x64, 0x9a, 0x9c, 0x38, 0x06, 0xc1, 0xdd, 0x7c, 0xa4,
            0xc4, 0x77, 0xe6, 0x29,
        ];

        let alice_key_pair = KeyStore::from(alice_private);
        let bob_key_pair = KeyStore::from(bob_private);

        assert_eq!(alice_public, alice_key_pair.public_key());
        assert_eq!(bob_public, bob_key_pair.public_key());

        let alice_computed_secret = alice_key_pair.dh(bob_public);
        let bob_computed_secret = bob_key_pair.dh(alice_public);

        assert_eq!(shared, alice_computed_secret);
        assert_eq!(shared, bob_computed_secret);
    }
    #[test]
    fn random_agreements() {
        for _ in 0..50 {
            let alice_key_pair = KeyStore::new();
            let bob_key_pair = KeyStore::new();

            let alice_computed_secret = alice_key_pair.dh(bob_key_pair.public_key());
            let bob_computed_secret = bob_key_pair.dh(alice_key_pair.public_key());

            assert_eq!(alice_computed_secret, bob_computed_secret);
        }
    }

    #[test]
    fn signature() {
        let alice_identity_private: [u8; PRIVATE_KEY_LENGTH] = [
            0xc0, 0x97, 0x24, 0x84, 0x12, 0xe5, 0x8b, 0xf0, 0x5d, 0xf4, 0x87, 0x96, 0x82, 0x05,
            0x13, 0x27, 0x94, 0x17, 0x8e, 0x36, 0x76, 0x37, 0xf5, 0x81, 0x8f, 0x81, 0xe0, 0xe6,
            0xce, 0x73, 0xe8, 0x65,
        ];
        let alice_identity_public: [u8; PUBLIC_KEY_LENGTH] = [
            0xab, 0x7e, 0x71, 0x7d, 0x4a, 0x16, 0x3b, 0x7d, 0x9a, 0x1d, 0x80, 0x71, 0xdf, 0xe9,
            0xdc, 0xf8, 0xcd, 0xcd, 0x1c, 0xea, 0x33, 0x39, 0xb6, 0x35, 0x6b, 0xe8, 0x4d, 0x88,
            0x7e, 0x32, 0x2c, 0x64,
        ];
        let alice_ephemeral_public: [u8; PUBLIC_KEY_LENGTH + 1] = [
            0x05, 0xed, 0xce, 0x9d, 0x9c, 0x41, 0x5c, 0xa7, 0x8c, 0xb7, 0x25, 0x2e, 0x72, 0xc2,
            0xc4, 0xa5, 0x54, 0xd3, 0xeb, 0x29, 0x48, 0x5a, 0x0e, 0x1d, 0x50, 0x31, 0x18, 0xd1,
            0xa8, 0x2d, 0x99, 0xfb, 0x4a,
        ];
        let alice_signature: [u8; SIGNATURE_LENGTH] = [
            0x5d, 0xe8, 0x8c, 0xa9, 0xa8, 0x9b, 0x4a, 0x11, 0x5d, 0xa7, 0x91, 0x09, 0xc6, 0x7c,
            0x9c, 0x74, 0x64, 0xa3, 0xe4, 0x18, 0x02, 0x74, 0xf1, 0xcb, 0x8c, 0x63, 0xc2, 0x98,
            0x4e, 0x28, 0x6d, 0xfb, 0xed, 0xe8, 0x2d, 0xeb, 0x9d, 0xcd, 0x9f, 0xae, 0x0b, 0xfb,
            0xb8, 0x21, 0x56, 0x9b, 0x3d, 0x90, 0x01, 0xbd, 0x81, 0x30, 0xcd, 0x11, 0xd4, 0x86,
            0xce, 0xf0, 0x47, 0xbd, 0x60, 0xb8, 0x6e, 0x88,
        ];

        let alice_identity_key_pair = KeyStore::from(alice_identity_private);

        assert_eq!(alice_identity_public, alice_identity_key_pair.public_key());

        assert!(
            KeyStore::verify_signature(
                alice_identity_public,
                &alice_ephemeral_public,
                alice_signature
            ),
            "signature check failed"
        );

        for i in 0..alice_signature.len() {
            let mut alice_signature_copy: [u8; SIGNATURE_LENGTH] = [0; SIGNATURE_LENGTH];
            alice_signature_copy.copy_from_slice(&alice_signature);
            alice_signature_copy[i] ^= 0x01u8;

            assert!(
                !KeyStore::verify_signature(
                    alice_identity_public,
                    &alice_ephemeral_public,
                    alice_signature_copy
                ),
                "signature check passed when it should not have"
            );
        }
    }

    #[test]
    fn random_signatures() {
        let mut rng = OsRng::default();
        for _ in 0..50 {
            let mut message = [0u8; 64];
            rng.fill(&mut message);
            let key_pair = KeyStore::new();
            let signature = key_pair.calculate_signature(&message);
            assert!(
                KeyStore::verify_signature(key_pair.public_key(), &message, signature),
                "signature check failed"
            );
        }
    }
}
