use core::slice;
use std::{ffi::c_void, ffi::CString, mem::ManuallyDrop, os::raw::c_char, ptr};

use keystore::KeyStore;
pub use keystore::SharedBuffer;

mod macros;

type RawKeyStore = *const c_void;

#[repr(C)]
pub struct Fixed32Array {
    data: *mut u8,
}

impl Fixed32Array {
    fn empty() -> Self {
        Self {
            data: ptr::null_mut(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_null()
    }
}

impl Into<[u8; 32]> for Fixed32Array {
    fn into(self) -> [u8; 32] {
        let slice = unsafe { slice::from_raw_parts(self.data, 32) };
        let mut fixed_slice = [0u8; 32];
        fixed_slice.copy_from_slice(slice);
        fixed_slice
    }
}

impl From<[u8; 32]> for Fixed32Array {
    fn from(data: [u8; 32]) -> Self {
        let mut vec = ManuallyDrop::new(Vec::from(data));
        Self {
            data: vec.as_mut_ptr(),
        }
    }
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
#[no_mangle]
pub extern "C" fn keystore_init(secret_key: Fixed32Array) -> RawKeyStore {
    let ks = KeyStore::init(secret_key.into());
    let boxed = Box::new(ks);
    Box::into_raw(boxed) as _
}

/// Get the KeyStore Public Key as `Fixed32Array`.
/// the caller should call [`keystore_fixed32_array_free`] after being done with it.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
///
/// otherwise it will return Empty Fixed32Array.
#[no_mangle]
pub unsafe extern "C" fn keystore_public_key(ks: RawKeyStore) -> Fixed32Array {
    let ks = keystore!(ks, Fixed32Array::empty());
    let pk = ks.public_key();
    Fixed32Array::from(pk)
}

/// Get the KeyStore Secret Key as `Fixed32Array`.
/// the caller should call [`keystore_fixed32_array_free`] after being done with it.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
///
/// otherwise it will return Empty Fixed32Array.
#[no_mangle]
pub unsafe extern "C" fn keystore_secret_key(ks: RawKeyStore) -> Fixed32Array {
    let ks = keystore!(ks, Fixed32Array::empty());
    let sk = ks.secret_key();
    Fixed32Array::from(sk)
}

/// Get the KeyStore Seed as `Fixed32Array`, returns Empty Fixed32Array if there is no seed.
/// the caller should call [`keystore_fixed32_array_free`] after being done with it.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
///
/// otherwise it will return Empty Fixed32Array.
#[no_mangle]
pub unsafe extern "C" fn keystore_seed(ks: RawKeyStore) -> Fixed32Array {
    let ks = keystore!(ks, Fixed32Array::empty());
    let seed = ks.seed();
    if let Some(seed) = seed {
        Fixed32Array::from(seed)
    } else {
        Fixed32Array::empty()
    }
}

/// Perform a Diffie-Hellman key agreement to produce a `SharedSecret`.
///
/// see [`KeyStore::dh`] for full docs.
/// the caller should call [`keystore_fixed32_array_free`] after being done with it.
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
///
/// otherwise it will return `null`.
#[no_mangle]
pub unsafe extern "C" fn keystore_dh(ks: RawKeyStore, their_public: Fixed32Array) -> Fixed32Array {
    let ks = keystore!(ks, Fixed32Array::empty());
    let shared_secret = ks.dh(their_public.into());
    Fixed32Array::from(shared_secret)
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
pub unsafe extern "C" fn keystore_backup(ks: RawKeyStore, seed: Fixed32Array) -> *const c_char {
    let ks = keystore!(ks);
    let paper_key = if !seed.is_empty() {
        result!(ks.backup(Some(seed.into())))
    } else {
        result!(ks.backup(None))
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
    let paper_key = cstr!(paper_key);
    let ks = result!(KeyStore::restore(paper_key.to_string()));
    let boxed = Box::new(ks);
    Box::into_raw(boxed) as _
}

/// Encrypt the Given data using `KeyStore` owned `SecretKey`
///
/// the caller should call [`keystore_cvec_free`] after being done with the pointer.
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - if `shared_secret` is empty, it will use the `KeyStore` secret key.
#[no_mangle]
pub unsafe extern "C" fn keystore_encrypt(
    ks: RawKeyStore,
    data: *mut SharedBuffer,
    shared_secret: Fixed32Array,
) -> i32 {
    let ks = keystore!(ks, 1);
    if let Some(data) = data.as_mut() {
        if !shared_secret.is_empty() {
            result!(ks.encrypt_with(shared_secret.into(), data), 2);
        } else {
            result!(ks.encrypt(data), 3);
        };
        0
    } else {
        -1
    }
}

/// Decrypt the Given data using `KeyStore` owned `SecretKey`
///
/// ### Safety
/// this function assumes that:
/// - `ks` is not null pointer to the `KeyStore`.
/// - if `shared_secret` is empty, it will use the `KeyStore` secret key.
#[no_mangle]
pub unsafe extern "C" fn keystore_decrypt(
    ks: RawKeyStore,
    data: *mut SharedBuffer,
    shared_secret: Fixed32Array,
) -> i32 {
    let ks = keystore!(ks, 1);
    if let Some(data) = data.as_mut() {
        if !shared_secret.is_empty() {
            result!(ks.decrypt_with(shared_secret.into(), data), 2);
        } else {
            result!(ks.decrypt(data), 3);
        };
        0
    } else {
        -1
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

/// Free (Drop) a Fixed32Array value allocated by Rust.
/// ### Safety
/// this assumes that the given pointer is not null.
#[no_mangle]
pub unsafe extern "C" fn keystore_fixed32_array_free(ptr: Fixed32Array) {
    if !ptr.is_empty() {
        let vec = Vec::from_raw_parts(ptr.data, 32, 32);
        drop(vec)
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
