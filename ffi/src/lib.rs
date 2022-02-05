use crate::pb::response::{self, error};

pub mod pb;

/// A Opaque pointer to a [`cryoto::KeyPair`]
type RawKeyPair = *const std::ffi::c_void;

#[repr(C)]
pub enum OwlchatResult {
    Ok = 1,
    NullPointerDetected = 2,
    InvalidProtobuf = 3,
}

/// Creates a new [crypto::KeyPair] and return an opaque pointer to it.
/// # Safety
///
/// You must call [owlchat_crypto_keypair_drop] once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_keypair_new() -> RawKeyPair {
    let pair = crypto::KeyPair::new();
    keypair_ptr_of(pair)
}

/// Drops a [crypto::KeyPair]
///
/// # Safety
/// Make sure that the pointer is valid.
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_keypair_drop(pair: RawKeyPair) {
    // check if the pointer is null
    if pair.is_null() {
        return;
    }
    let _ = Box::from_raw(pair as *mut crypto::KeyPair);
}

/// A simple macro to get keyspair from a raw pointer
#[doc(hidden)]
macro_rules! keypair {
    ($ptr:expr) => {
        if $ptr.is_null() {
            return crate::OwlchatResult::NullPointerDetected;
        } else {
            ($ptr as *const crypto::KeyPair).as_ref().unwrap()
        }
    };
}

/// This a Dart FFI interface to be called inside an Isolate.
///
/// Passing a Isolate Port, along with some Protobuf payload, to this function will
/// process the payload and return the result to the isolate over the port.
///
/// # Errors
///
/// This function will return an error if the provided payload is not valid.
///
/// # Safety
///
/// This function is unsafe because it deals with raw pointers.
#[cfg(feature = "dart-ffi")]
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_dispatch(
    keypair: RawKeyPair,
    port: i64,
    data: *const u8,
    len: usize,
) -> OwlchatResult {
    use allo_isolate::Isolate;
    use allo_isolate::ZeroCopyBuffer;
    use prost::Message;

    let keypair = keypair!(keypair);
    // check if the pointer is null first
    if data.is_null() {
        return OwlchatResult::NullPointerDetected;
    }
    // then read it as a slice
    let buf = std::slice::from_raw_parts(data, len);
    let isolate = Isolate::new(port);
    let req = match Message::decode(buf) {
        Ok(v) => v,
        Err(e) => {
            isolate.post(e.to_string());
            return OwlchatResult::InvalidProtobuf;
        }
    };
    let res = handle_request(keypair, req);
    let mut res_buf = Vec::with_capacity(res.encoded_len());
    res.encode(&mut res_buf).unwrap();
    isolate.post(ZeroCopyBuffer(res_buf));
    OwlchatResult::Ok
}

#[cfg(not(feature = "dart-ffi"))]
#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

#[cfg(not(feature = "dart-ffi"))]
impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

/// Main function to handle the request.
///
/// # Errors
///
/// This function will return null if the provided payload is not valid.
///
/// # Safety
/// you should free the returned buffer using [owlchat_crypto_free_buffer] after you are done with it.
#[cfg(not(feature = "dart-ffi"))]
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_dispatch(
    keypair: RawKeyPair,
    data: *const u8,
    len: usize,
) -> *const Buffer {
    use prost::Message;
    let keypair = keypair!(keypair);
    // check if the pointer is null first
    if data.is_null() {
        return Default::default();
    }
    // then read it as a slice
    let buf = std::slice::from_raw_parts(data, len);
    let req = match Message::decode(buf) {
        Ok(v) => v,
        Err(_) => {
            return Default::default();
        }
    };
    let res = handle_request(keypair, req);
    let mut res_buf = Vec::with_capacity(res.encoded_len());
    res.encode(&mut res_buf).unwrap();
    let mut boxed_buf = res_buf.into_boxed_slice();
    let data = boxed_buf.as_mut_ptr();
    let len = boxed_buf.len();
    std::mem::forget(boxed_buf);
    Box::leak(Box::new(Buffer { data, len }))
}

/// Free the buffer returned by [owlchat_crypto_dispatch].
///
/// # Examples
///
/// ```
/// use owlchat_crypto::owlchat_crypto_free_buffer;
///
/// unsafe { owlchat_crypto_free_buffer(buffer) };
/// ```
///
/// # Safety
///
/// This function is unsafe because it deals with raw pointers.
#[cfg(not(feature = "dart-ffi"))]
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_free_buffer(buffer: Box<Buffer>) {
    if buffer.data.is_null() {
        return;
    }
    let s = std::slice::from_raw_parts_mut(buffer.data, buffer.len);
    Box::from_raw(s.as_mut_ptr());
}

unsafe fn keypair_ptr_of(pair: crypto::KeyPair) -> RawKeyPair {
    let boxed = Box::new(pair);
    Box::into_raw(boxed) as _
}

unsafe fn handle_request(keypair: &crypto::KeyPair, req: pb::Request) -> pb::Response {
    use pb::request::Body as RequestBody;
    use pb::response::Body as ResponseBody;
    let body = match req.body {
        Some(body) => body,
        None => {
            return pb::Response {
                body: Some(ResponseBody::Error(response::Error {
                    kind: error::Kind::MissingRequestBody.into(),
                    message: "No request body found".to_string(),
                })),
            }
        }
    };

    let res_body = match body {
        RequestBody::CurrentKeyPair(_) => ResponseBody::KeyPair(pb::KeyPair {
            public_key: keypair.public_key().to_vec(),
            secret_key: keypair.secret_key().to_vec(),
            seed: keypair.seed().map(|v| v.to_vec()).unwrap_or_default(),
            raw_pointer: keypair as *const _ as _,
        }),
        RequestBody::ValidateMnemonic(v) => {
            let is_valid = crypto::KeyPair::is_valid_mnemonic(&v.phrase);
            ResponseBody::ValidMnemonic(is_valid)
        }
        RequestBody::Verify(v) => {
            let their_public_key = match public_key_from(&v.public_key) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let signature = match signature_from(&v.sig) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let is_valid = crypto::KeyPair::verify_signature(their_public_key, &v.msg, signature);
            ResponseBody::ValidSignature(is_valid)
        }
        RequestBody::GenerateKeyPair(_) => {
            let keypair = crypto::KeyPair::new();
            let public_key = keypair.public_key().to_vec();
            let secret_key = keypair.secret_key().to_vec();
            let seed = keypair.seed().map(|v| v.to_vec()).unwrap_or_default();
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed,
                raw_pointer: keypair_ptr_of(keypair) as _,
            })
        }
        RequestBody::InitKeyPair(v) => {
            let secret_key = match secret_key_from(&v.secret_key) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let keypair = crypto::KeyPair::init(secret_key);
            let public_key = keypair.public_key().to_vec();
            let secret_key = keypair.secret_key().to_vec();
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed: Default::default(),
                raw_pointer: keypair_ptr_of(keypair) as _,
            })
        }
        RequestBody::RestoreKeyPair(v) => {
            let keypair = match crypto::KeyPair::restore(v.paper_key) {
                Ok(v) => v,
                Err(e) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(response::Error {
                            kind: error::Kind::InvalidPaperKey.into(),
                            message: e.to_string(),
                        })),
                    }
                }
            };
            let public_key = keypair.public_key().to_vec();
            let secret_key = keypair.secret_key().to_vec();
            let seed = keypair.seed().map(|v| v.to_vec()).unwrap_or_default();
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed,
                raw_pointer: keypair_ptr_of(keypair) as _,
            })
        }
        RequestBody::BackupKeyPair(v) => {
            let seed = seed_from(&v.maybe_seed).ok();
            let paper_key = match keypair.backup(seed) {
                Ok(v) => v,
                Err(e) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(response::Error {
                            kind: error::Kind::InvalidSeed.into(),
                            message: e.to_string(),
                        })),
                    }
                }
            };
            ResponseBody::Mnemonic(paper_key)
        }
        RequestBody::Encrypt(mut v) => {
            // try to read the secret key from the request
            // if it's not there, try to read it from the keypair.
            let secret_key = match secret_key_from(&v.secret_key) {
                Ok(v) => v,
                Err(_) => keypair.secret_key(),
            };
            let msg = &mut v.plaintext;
            match keypair.encrypt_with(secret_key, msg) {
                Ok(_) => {
                    // the plaintext now is the ciphertext
                    let ciphertext = v.plaintext;
                    ResponseBody::EncryptedMessage(ciphertext)
                }
                Err(e) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(response::Error {
                            kind: error::Kind::Unknown.into(),
                            message: e.to_string(),
                        })),
                    }
                }
            }
        }
        RequestBody::Decrypt(mut v) => {
            // try to read the secret key from the request
            // if it's not there, try to read it from the keypair.
            let secret_key = match secret_key_from(&v.secret_key) {
                Ok(v) => v,
                Err(_) => keypair.secret_key(),
            };
            let msg = &mut v.ciphertext;
            match keypair.decrypt_with(secret_key, msg) {
                Ok(_) => {
                    // the ciphertext now is the plaintext
                    let plaintext = v.ciphertext;
                    ResponseBody::DecryptedMessage(plaintext)
                }
                Err(e) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(response::Error {
                            kind: error::Kind::Unknown.into(),
                            message: e.to_string(),
                        })),
                    }
                }
            }
        }
        RequestBody::Sign(v) => {
            let signature = keypair.calculate_signature(&v.msg);
            ResponseBody::Signature(signature.to_vec())
        }
        RequestBody::DiffieHellmanKeyExchange(v) => {
            let their_public_key = match public_key_from(&v.their_public_key) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let shared_secret = keypair.dh(their_public_key);
            ResponseBody::SharedSecret(shared_secret.to_vec())
        }
        RequestBody::HashSha256(v) => {
            let hash = crypto::util::sha256_hash_bytes(&v.input);
            ResponseBody::Sha256Hash(hash.to_vec())
        }
        RequestBody::HashFileSha256(v) => {
            match crypto::util::sha256_hash_file(std::path::PathBuf::from(&v.path)) {
                Ok(hash) => ResponseBody::Sha256Hash(hash.to_vec()),
                Err(e) => ResponseBody::Error(response::Error {
                    kind: error::Kind::Unknown.into(),
                    message: e.to_string(),
                }),
            }
        }
    };
    pb::Response {
        body: Some(res_body),
    }
}

fn public_key_from(v: &[u8]) -> Result<[u8; crypto::PUBLIC_KEY_LENGTH], pb::Response> {
    // check the length of the public key
    if v.len() != crypto::PUBLIC_KEY_LENGTH {
        return Err(pb::Response {
            body: Some(response::Body::Error(response::Error {
                kind: error::Kind::InvalidPublicKey.into(),
                message: "Invalid public key length".to_string(),
            })),
        });
    }
    let mut res = [0u8; crypto::PUBLIC_KEY_LENGTH];
    res.copy_from_slice(&v[0..crypto::PUBLIC_KEY_LENGTH]);
    Ok(res)
}

fn secret_key_from(v: &[u8]) -> Result<[u8; crypto::SECRET_KEY_LENGTH], pb::Response> {
    // check the length of the secret key
    if v.len() != crypto::SECRET_KEY_LENGTH {
        return Err(pb::Response {
            body: Some(pb::response::Body::Error(response::Error {
                kind: error::Kind::InvalidSecretKey.into(),
                message: "Invalid secret key length".to_string(),
            })),
        });
    }
    let mut res = [0u8; crypto::SECRET_KEY_LENGTH];
    res.copy_from_slice(&v[0..crypto::SECRET_KEY_LENGTH]);
    Ok(res)
}

fn seed_from(v: &[u8]) -> Result<[u8; crypto::SEED_LENGTH], pb::Response> {
    // check the length of the seed
    if v.len() != crypto::SEED_LENGTH {
        return Err(pb::Response {
            body: Some(pb::response::Body::Error(response::Error {
                kind: error::Kind::InvalidSeed.into(),
                message: "Invalid seed length".to_string(),
            })),
        });
    }
    let mut res = [0u8; crypto::SEED_LENGTH];
    res.copy_from_slice(&v[0..crypto::SEED_LENGTH]);
    Ok(res)
}

fn signature_from(v: &[u8]) -> Result<[u8; crypto::SIGNATURE_LENGTH], pb::Response> {
    // check the length of the signature
    if v.len() != crypto::SIGNATURE_LENGTH {
        return Err(pb::Response {
            body: Some(pb::response::Body::Error(response::Error {
                kind: error::Kind::InvalidSignature.into(),
                message: "Invalid signature length".to_string(),
            })),
        });
    }
    let mut res = [0u8; crypto::SIGNATURE_LENGTH];
    res.copy_from_slice(&v[0..crypto::SIGNATURE_LENGTH]);
    Ok(res)
}
