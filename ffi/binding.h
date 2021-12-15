#include <stdint.h>

enum class OwlchatResult {
  Ok = 1,
  NotInitialized = 2,
  AlreadyInitialized = 3,
  NullPointerDetected = 4,
  InvalidProtobuf = 5,
};

#if !defined(DEFINE_DART)
struct Buffer {
  uint8_t *data;
  uintptr_t len;
};
#endif

extern "C" {

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
OwlchatResult owlchat_crypto_init();

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
OwlchatResult owlchat_crypto_destory();

#if defined(DEFINE_DART)
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
OwlchatResult owlchat_crypto_dispatch(int64_t port, const uint8_t *data, uintptr_t len);
#endif

#if !defined(DEFINE_DART)
/// Main function to handle the request.
///
/// # Errors
///
/// This function will return null if the provided payload is not valid.
///
/// # Safety
/// you should free the returned buffer using [owlchat_crypto_free_buffer] after you are done with it.
Buffer owlchat_crypto_dispatch(const uint8_t *data, uintptr_t len);
#endif

#if !defined(DEFINE_DART)
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
void owlchat_crypto_free_buffer(Buffer buffer);
#endif

} // extern "C"
