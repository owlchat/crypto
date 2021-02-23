use core::slice;
use std::{ffi::c_void, ffi::CString, mem::ManuallyDrop, os::raw::c_char, path::PathBuf, ptr};

use keystore::KeyStore;
pub use keystore::SharedBuffer;

mod macros;

type RawKeyStore = *const c_void;
type RawMutFixed32Array = *mut Fixed32Array;
type RawFixed32Array = *const Fixed32Array;
type RawSharedBuffer = *mut SharedBuffer;
type RawMutFixed64Array = *mut Fixed64Array;
type RawFixed64Array = *const Fixed64Array;

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

impl<'a> From<&'a Fixed32Array> for [u8; 32] {
    fn from(arr: &'a Fixed32Array) -> [u8; 32] {
        let slice = unsafe { slice::from_raw_parts(arr.buf, 32) };
        let mut fixed_slice = [0u8; 32];
        fixed_slice.copy_from_slice(slice);
        fixed_slice
    }
}

#[repr(C)]
pub struct Fixed64Array {
    buf: *mut u8,
}

impl Fixed64Array {
    unsafe fn write(&mut self, data: [u8; 64]) {
        let buf = slice::from_raw_parts_mut(self.buf, 64);
        buf.copy_from_slice(&data);
    }
}

impl<'a> From<&'a Fixed64Array> for [u8; 64] {
    fn from(arr: &'a Fixed64Array) -> [u8; 64] {
        let slice = unsafe { slice::from_raw_parts(arr.buf, 64) };
        let mut fixed_slice = [0u8; 64];
        fixed_slice.copy_from_slice(slice);
        fixed_slice
    }
}

#[repr(C)]
pub enum OperationStatus {
    Ok,
    Unknwon,
    KeyStoreNotInialized,
    BadFixed32ArrayProvided,
    BadFixed64ArrayProvided,
    BadSharedBufferProvided,
    KeyStoreHasNoSeed,
    AeadError,
    Bip39Error,
    Utf8Error,
    IoError,
    InvalidSignature,
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
        OperationStatus::Ok
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
        OperationStatus::Ok
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
            OperationStatus::Ok
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
            OperationStatus::Ok
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
        OperationStatus::Ok
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
        OperationStatus::Ok
    } else {
        OperationStatus::BadSharedBufferProvided
    }
}

/// Calculate the signature of the message using the given `KeyStore`.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - `message` is not null pointer and valid bytes buffer.
#[no_mangle]
pub unsafe extern "C" fn keystore_calculate_signature(
    ks: RawKeyStore,
    message: RawSharedBuffer,
    out: RawMutFixed64Array,
) -> OperationStatus {
    let ks = keystore!(ks);
    match (message.as_ref(), out.as_mut()) {
        (Some(msg), Some(out)) => {
            let sig = ks.calculate_signature(msg.as_ref());
            out.write(sig);
            OperationStatus::Ok
        }
        (Some(_), None) => OperationStatus::BadFixed64ArrayProvided,
        (None, Some(_)) => OperationStatus::BadSharedBufferProvided,
        _ => OperationStatus::Unknwon,
    }
}

/// Verifies the signature of the message using the given `PublicKey`.
///
/// ### Safety
/// this function assumes that:
/// - `thier_public` is not null pointer to the fixed size 32 bytes array.
/// - `message` is not null pointer and valid bytes buffer.
/// - `signature` is not null pointer to the fixed size 64 bytes array.
#[no_mangle]
pub unsafe extern "C" fn keystore_verify_signature(
    thier_public: RawFixed32Array,
    message: RawSharedBuffer,
    signature: RawFixed64Array,
) -> OperationStatus {
    match (thier_public.as_ref(), message.as_ref(), signature.as_ref()) {
        (Some(public), Some(msg), Some(sig)) => {
            let ok = KeyStore::verify_signature(public.into(), msg.as_ref(), sig.into());
            if ok {
                OperationStatus::Ok
            } else {
                OperationStatus::InvalidSignature
            }
        }
        (None, _, _) => OperationStatus::BadFixed32ArrayProvided,
        (_, _, None) => OperationStatus::BadFixed64ArrayProvided,
        (_, None, _) => OperationStatus::BadSharedBufferProvided,
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
            OperationStatus::Ok
        }
        _ => OperationStatus::IoError,
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
