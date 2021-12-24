#include <stdint.h>

typedef enum OwlchatResult {
  Ok = 1,
  NullPointerDetected = 2,
  InvalidProtobuf = 3,
} OwlchatResult;

// A Opaque pointer to a [`cryoto::KeyPair`]
typedef const void *RawKeyPair;

#if !defined(DART)
typedef struct Buffer {
  uint8_t *data;
  uintptr_t len;
} Buffer;
#endif

// Creates a new [crypto::KeyPair] and return an opaque pointer to it.
// # Safety
//
// You must call [owlchat_crypto_keypair_drop] once you are done with it.
RawKeyPair owlchat_crypto_keypair_new(void);

// Drops a [crypto::KeyPair]
//
// # Safety
// Make sure that the pointer is valid.
void owlchat_crypto_keypair_drop(RawKeyPair pair);

#if defined(DART)
// This a Dart FFI interface to be called inside an Isolate.
//
// Passing a Isolate Port, along with some Protobuf payload, to this function will
// process the payload and return the result to the isolate over the port.
//
// # Errors
//
// This function will return an error if the provided payload is not valid.
//
// # Safety
//
// This function is unsafe because it deals with raw pointers.
enum OwlchatResult owlchat_crypto_dispatch(RawKeyPair keypair, int64_t port, const uint8_t *data, uintptr_t len);
#endif

#if !defined(DART)
// Main function to handle the request.
//
// # Errors
//
// This function will return null if the provided payload is not valid.
//
// # Safety
// you should free the returned buffer using [owlchat_crypto_free_buffer] after you are done with it.
struct Buffer owlchat_crypto_dispatch(RawKeyPair keypair, const uint8_t *data, uintptr_t len);
#endif

#if !defined(DART)
// Free the buffer returned by [owlchat_crypto_dispatch].
//
// # Examples
//
// ```
// use owlchat_crypto::owlchat_crypto_free_buffer;
//
// unsafe { owlchat_crypto_free_buffer(buffer) };
// ```
//
// # Safety
//
// This function is unsafe because it deals with raw pointers.
void owlchat_crypto_free_buffer(struct Buffer buffer);
#endif
