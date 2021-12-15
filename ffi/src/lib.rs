use once_cell::sync::OnceCell;

pub mod pb;

static mut KEYPAIR: OnceCell<crypto::KeyPair> = OnceCell::new();

#[repr(C)]
pub enum OwlchatResult {
    Ok = 1,
    NotInitialized = 2,
    AlreadyInitialized = 3,
    NullPointerDetected = 4,
    InvalidProtobuf = 5,
}

/// Initialize the crypto library.
///
/// This function must be called before any other crypto function.
/// It is **NOT** safe to call this function multiple times.
/// # Examples
///
/// ```
/// use owlchat_crypto::*;
///
/// assert_eq!(unsafe { owlchat_crypto_init() }, OwlchatResult::Ok);
/// ```
///
/// # Errors
///
/// This function will return an error if the [crypto::KeyPair] is already initialized.
///
/// # Safety
///
/// Should be only called once during the lifecycle of the application.
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_init() -> OwlchatResult {
    if KEYPAIR.get().is_some() {
        OwlchatResult::AlreadyInitialized
    } else {
        let _ = KEYPAIR.set(crypto::KeyPair::new());
        OwlchatResult::Ok
    }
}

/// Destroy the crypto library, freeing all memory.
///
/// This function must be called before the application exits.
/// It is **NOT** safe to call this function multiple times.
///
/// # Examples
///
/// ```
/// use owlchat_crypto::*;
///
/// assert_eq!(unsafe { owlchat_crypto_destory() }, OwlchatResult::NotInitialized);
/// assert_eq!(unsafe { owlchat_crypto_init() }, OwlchatResult::Ok);
/// assert_eq!(unsafe { owlchat_crypto_destory() }, OwlchatResult::Ok);
/// ```
///
/// # Errors
///
/// This function will return an error if Keypair is not initialized yet.
///
/// # Safety
///
/// Calling this function will deallocate the [crypto::KeyPair] and remove it from the memory
/// so calling it, while the [crypto::KeyPair] is still in use, will cause undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn owlchat_crypto_destory() -> OwlchatResult {
    match KEYPAIR.take() {
        Some(_) => OwlchatResult::Ok,
        None => OwlchatResult::NotInitialized,
    }
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
    port: i64,
    data: *const u8,
    len: usize,
) -> OwlchatResult {
    use allo_isolate::Isolate;
    use allo_isolate::ZeroCopyBuffer;
    use prost::Message;

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
    let res = handle_request(req);
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
pub unsafe extern "C" fn owlchat_crypto_dispatch(data: *const u8, len: usize) -> Buffer {
    use prost::Message;

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
    let res = handle_request(req);
    let mut res_buf = Vec::with_capacity(res.encoded_len());
    res.encode(&mut res_buf).unwrap();
    let mut boxed_buf = res_buf.into_boxed_slice();
    let data = boxed_buf.as_mut_ptr();
    let len = boxed_buf.len();
    std::mem::forget(boxed_buf);
    Buffer { data, len }
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
pub unsafe extern "C" fn owlchat_crypto_free_buffer(buffer: Buffer) {
    if buffer.data.is_null() {
        return;
    }
    let s = std::slice::from_raw_parts_mut(buffer.data, buffer.len);
    Box::from_raw(s.as_mut_ptr());
}

unsafe fn handle_request(req: pb::Request) -> pb::Response {
    use pb::request::Body as RequestBody;
    use pb::response::Body as ResponseBody;
    let body = match req.body {
        Some(body) => body,
        None => {
            return pb::Response {
                body: Some(ResponseBody::Error(
                    pb::response::Error::MissingRequestBody.into(),
                )),
            }
        }
    };

    let res_body = match body {
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
            let _ = KEYPAIR.set(keypair);
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed,
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
            let _ = KEYPAIR.set(keypair);
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed: Default::default(),
            })
        }
        RequestBody::RestoreKeyPair(v) => {
            let keypair = match crypto::KeyPair::restore(v.paper_key) {
                Ok(v) => v,
                Err(_) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::InvalidPaperKey.into(),
                        )),
                    }
                }
            };
            let public_key = keypair.public_key().to_vec();
            let secret_key = keypair.secret_key().to_vec();
            let _ = KEYPAIR.set(keypair);
            ResponseBody::KeyPair(pb::KeyPair {
                public_key,
                secret_key,
                seed: vec![],
            })
        }
        RequestBody::BackupKeyPair(v) => {
            let seed = seed_from(&v.maybe_seed).ok();
            let keypair = match KEYPAIR.get() {
                Some(v) => v,
                None => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::NotInitialized.into(),
                        )),
                    }
                }
            };
            let paper_key = match keypair.backup(seed) {
                Ok(v) => v,
                Err(_) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(pb::response::Error::InvalidSeed.into())),
                    }
                }
            };
            ResponseBody::Mnemonic(paper_key)
        }
        RequestBody::Encrypt(mut v) => {
            let msg = &mut v.plaintext;
            let keypair = match KEYPAIR.get() {
                Some(v) => v,
                None => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::NotInitialized.into(),
                        )),
                    }
                }
            };
            match keypair.encrypt(msg) {
                Ok(_) => {
                    // the plaintext now is the ciphertext
                    let ciphertext = v.plaintext;
                    ResponseBody::EncryptedMessage(ciphertext)
                }
                Err(_) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(pb::response::Error::Unknown.into())),
                    }
                }
            }
        }
        RequestBody::Decrypt(mut v) => {
            let msg = &mut v.ciphertext;
            let keypair = match KEYPAIR.get() {
                Some(v) => v,
                None => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::NotInitialized.into(),
                        )),
                    }
                }
            };
            match keypair.decrypt(msg) {
                Ok(_) => {
                    // the ciphertext now is the plaintext
                    let plaintext = v.ciphertext;
                    ResponseBody::DecryptedMessage(plaintext)
                }
                Err(_) => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(pb::response::Error::Unknown.into())),
                    }
                }
            }
        }
        RequestBody::Sign(v) => {
            let keypair = match KEYPAIR.get() {
                Some(v) => v,
                None => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::NotInitialized.into(),
                        )),
                    }
                }
            };
            let signature = keypair.calculate_signature(&v.msg);
            ResponseBody::Signature(signature.to_vec())
        }
        RequestBody::DiffieHellmanKeyExchange(v) => {
            let their_public_key = match public_key_from(&v.their_public_key) {
                Ok(v) => v,
                Err(e) => return e,
            };
            let keypair = match KEYPAIR.get() {
                Some(v) => v,
                None => {
                    return pb::Response {
                        body: Some(ResponseBody::Error(
                            pb::response::Error::NotInitialized.into(),
                        )),
                    }
                }
            };
            let shared_secret = keypair.dh(their_public_key);
            ResponseBody::SharedSecret(shared_secret.to_vec())
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
            body: Some(pb::response::Body::Error(
                pb::response::Error::InvalidPublicKey.into(),
            )),
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
            body: Some(pb::response::Body::Error(
                pb::response::Error::InvalidSecretKey.into(),
            )),
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
            body: Some(pb::response::Body::Error(
                pb::response::Error::InvalidSeed.into(),
            )),
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
            body: Some(pb::response::Body::Error(
                pb::response::Error::InvalidSignature.into(),
            )),
        });
    }
    let mut res = [0u8; crypto::SIGNATURE_LENGTH];
    res.copy_from_slice(&v[0..crypto::SIGNATURE_LENGTH]);
    Ok(res)
}
