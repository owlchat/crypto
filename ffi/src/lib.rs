use core::slice;
use std::{ffi::c_void, ffi::CString, mem::ManuallyDrop, os::raw::c_char, path::PathBuf, ptr};

use keystore::KeyStore;
pub use keystore::SharedBuffer;

mod macros;

type RawKeyStore = *const c_void;
type RawMutFixed32Array = *mut Fixed32Array;
type RawFixed32Array = *const Fixed32Array;
type RawSharedBuffer = *mut SharedBuffer;

#[repr(C)]
pub struct Fixed32Array {
    buf: *mut u8,
}

impl Fixed32Array {
    unsafe fn write(&mut self, data: [u8; 32]) {
        let buf = slice::from_raw_parts_mut(self.buf, 32);
        buf.copy_from_slice(&data);
    }
}

impl<'a> Into<[u8; 32]> for &'a Fixed32Array {
    fn into(self) -> [u8; 32] {
        let slice = unsafe { slice::from_raw_parts(self.buf, 32) };
        let mut fixed_slice = [0u8; 32];
        fixed_slice.copy_from_slice(slice);
        fixed_slice
    }
}

#[repr(C)]
pub enum OperationStatus {
    OK,
    Unknwon,
    KeyStoreNotInialized,
    BadFixed32ArrayProvided,
    BadSharedBufferProvided,
    KeyStoreHasNoSeed,
    AeadError,
    Bip39Error,
    Utf8Error,
    IOError,
}
/// Create a new [`KeyStore`].
///
/// See [`KeyStore::new`] for full docs.
#[no_mangle]
pub extern "C" fn keystore_new() -> RawKeyStore {
    let ks = KeyStore::new();
    let boxed = Box::new(ks);
    Box::into_raw(boxed) as _
}

/// Init the `KeyStore` with existing SecretKey Bytes.
///
/// See [`KeyStore::init`] for full docs.
///
/// ### Safety
/// this function assumes that:
/// - `secret_key` is not null
/// otherwise it will return null.
#[no_mangle]
pub unsafe extern "C" fn keystore_init(secret_key: RawFixed32Array) -> RawKeyStore {
    if let Some(secret_key) = secret_key.as_ref() {
        let ks = KeyStore::init(secret_key.into());
        let boxed = Box::new(ks);
        Box::into_raw(boxed) as _
    } else {
        ptr::null()
    }
}

/// Get the KeyStore Public Key as `Fixed32Array`.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
#[no_mangle]
pub unsafe extern "C" fn keystore_public_key(
    ks: RawKeyStore,
    out: RawMutFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    let pk = ks.public_key();
    if let Some(out) = out.as_mut() {
        out.write(pk);
        OperationStatus::OK
    } else {
        OperationStatus::BadFixed32ArrayProvided
    }
}

/// Get the KeyStore Secret Key as `Fixed32Array`.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
#[no_mangle]
pub unsafe extern "C" fn keystore_secret_key(
    ks: RawKeyStore,
    out: RawMutFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    let sk = ks.secret_key();
    if let Some(out) = out.as_mut() {
        out.write(sk);
        OperationStatus::OK
    } else {
        OperationStatus::BadFixed32ArrayProvided
    }
}

/// Get the KeyStore Seed as `Fixed32Array`.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
#[no_mangle]
pub unsafe extern "C" fn keystore_seed(
    ks: RawKeyStore,
    out: RawMutFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    let seed = ks.seed();
    match (seed, out.as_mut()) {
        (Some(seed), Some(out)) => {
            out.write(seed);
            OperationStatus::OK
        }
        (None, _) => OperationStatus::KeyStoreHasNoSeed,
        (Some(_), None) => OperationStatus::BadFixed32ArrayProvided,
    }
}

/// Perform a Diffie-Hellman key agreement to produce a `SharedSecret`.
///
/// see [`KeyStore::dh`] for full docs.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
#[no_mangle]
pub unsafe extern "C" fn keystore_dh(
    ks: RawKeyStore,
    their_public: RawFixed32Array,
    out: RawMutFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    match (their_public.as_ref(), out.as_mut()) {
        (Some(pk), Some(out)) => {
            let shared_secret = ks.dh(pk.into());
            out.write(shared_secret);
            OperationStatus::OK
        }
        _ => OperationStatus::BadFixed32ArrayProvided,
    }
}

/// Create a [`Mnemonic`] Backup from the provided seed (or the keystore seed if exist).
///
/// the caller should call [`keystore_string_free`] after being done with it.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - if `seed` is empty, it will try to use the `KeyStore` seed if available.
///
/// otherwise it will return null.
#[no_mangle]
pub unsafe extern "C" fn keystore_backup(ks: RawKeyStore, seed: RawFixed32Array) -> *const c_char {
    let ks = keystore!(ks, ptr::null());
    let paper_key = if let Some(seed) = seed.as_ref() {
        unwrap_or_null!(ks.backup(Some(seed.into())))
    } else {
        unwrap_or_null!(ks.backup(None))
    };
    let cstring = CString::new(paper_key).expect("should never fails");
    let cstring = ManuallyDrop::new(cstring);
    cstring.as_ptr()
}

/// Restore a `KeyStore` from a [`Mnemonic`] Paper Backup.
///
/// see [`KeyStore::restore`] for full docs.
/// ### Safety
/// this function assumes that:
/// - `paper_key` is not null and a valid c string.
#[no_mangle]
pub unsafe extern "C" fn keystore_restore(paper_key: *const c_char) -> RawKeyStore {
    let paper_key = cstr!(paper_key, ptr::null());
    let ks = unwrap_or_null!(KeyStore::restore(paper_key.to_string()));
    let boxed = Box::new(ks);
    Box::into_raw(boxed) as _
}

/// Encrypt the Given data using `KeyStore` owned `SecretKey`
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - if `shared_secret` is null, it will use the `KeyStore` secret key.
#[no_mangle]
pub unsafe extern "C" fn keystore_encrypt(
    ks: RawKeyStore,
    data: RawSharedBuffer,
    shared_secret: RawFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    if let Some(data) = data.as_mut() {
        if let Some(shared_secret) = shared_secret.as_ref() {
            result!(ks.encrypt_with(shared_secret.into(), data));
        } else {
            result!(ks.encrypt(data));
        };
        OperationStatus::OK
    } else {
        OperationStatus::BadSharedBufferProvided
    }
}

/// Decrypt the Given data using `KeyStore` owned `SecretKey`
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - if `shared_secret` is null, it will use the `KeyStore` secret key.
#[no_mangle]
pub unsafe extern "C" fn keystore_decrypt(
    ks: RawKeyStore,
    data: RawSharedBuffer,
    shared_secret: RawFixed32Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    if let Some(data) = data.as_mut() {
        if let Some(shared_secret) = shared_secret.as_ref() {
            result!(ks.decrypt_with(shared_secret.into(), data));
        } else {
            result!(ks.decrypt(data));
        };
        OperationStatus::OK
    } else {
        OperationStatus::BadSharedBufferProvided
    }
}

/// Calculate SHA256 Hash of the provided file path.
///
/// ### Safety
/// this function assumes that:
/// - `file_path` is not null pointer.
/// - `out` is not null pointer.
#[no_mangle]
pub unsafe extern "C" fn keystore_sha256_hash(
    file_path: *const c_char,
    out: RawMutFixed32Array,
) -> OperationStatus {
    let path = cstr!(file_path, OperationStatus::Utf8Error);
    if out.is_null() {
        return OperationStatus::BadFixed32ArrayProvided;
    }
    let path = PathBuf::from(path);
    match keystore::util::sh256_hash(path) {
        Ok(hash) => {
            (*out).write(hash);
            OperationStatus::OK
        }
        _ => OperationStatus::IOError,
    }
}

/// Free (Drop) a string value allocated by Rust.
/// ### Safety
/// this assumes that the given pointer is not null.
#[no_mangle]
pub unsafe extern "C" fn keystore_string_free(ptr: *const c_char) {
    if !ptr.is_null() {
        let cstring = CString::from_raw(ptr as _);
        drop(cstring)
    }
}

/// Free (Drop) the created KeyStore.
/// ### Safety
/// this assumes that the given pointer is not null.
#[no_mangle]
pub unsafe extern "C" fn keystore_free(ks: RawKeyStore) {
    if !ks.is_null() {
        let ks = Box::from_raw(ks as *mut KeyStore);
        drop(ks);
    }
}
