#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
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
} OperationStatus;

typedef const void *RawKeyStore;

typedef struct {
  uint8_t *buf;
} Fixed32Array;

typedef const Fixed32Array *RawFixed32Array;

typedef struct {
  uint8_t *buf;
  uintptr_t len;
  uintptr_t cap;
} SharedBuffer;

typedef SharedBuffer *RawSharedBuffer;

typedef Fixed32Array *RawMutFixed32Array;

// Create a [`Mnemonic`] Backup from the provided seed (or the keystore seed if exist).
//
// the caller should call [`keystore_string_free`] after being done with it.
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
// - if `seed` is empty, it will try to use the `KeyStore` seed if available.
//
// otherwise it will return null.
const char *keystore_backup(RawKeyStore ks, RawFixed32Array seed);

// Decrypt the Given data using `KeyStore` owned `SecretKey`
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
// - if `shared_secret` is null, it will use the `KeyStore` secret key.
OperationStatus keystore_decrypt(RawKeyStore ks,
                                 RawSharedBuffer data,
                                 RawFixed32Array shared_secret);

// Perform a Diffie-Hellman key agreement to produce a `SharedSecret`.
//
// see [`KeyStore::dh`] for full docs.
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
OperationStatus keystore_dh(RawKeyStore ks, RawFixed32Array their_public, RawMutFixed32Array out);

// Encrypt the Given data using `KeyStore` owned `SecretKey`
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
// - if `shared_secret` is null, it will use the `KeyStore` secret key.
OperationStatus keystore_encrypt(RawKeyStore ks,
                                 RawSharedBuffer data,
                                 RawFixed32Array shared_secret);

// Free (Drop) the created KeyStore.
// ### Safety
// this assumes that the given pointer is not null.
void keystore_free(RawKeyStore ks);

// Init the `KeyStore` with existing SecretKey Bytes.
//
// See [`KeyStore::init`] for full docs.
//
// ### Safety
// this function assumes that:
// - `secret_key` is not null
// otherwise it will return null.
RawKeyStore keystore_init(RawFixed32Array secret_key);

// Create a new [`KeyStore`].
//
// See [`KeyStore::new`] for full docs.
RawKeyStore keystore_new(void);

// Get the KeyStore Public Key as `Fixed32Array`.
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
OperationStatus keystore_public_key(RawKeyStore ks, RawMutFixed32Array out);

// Restore a `KeyStore` from a [`Mnemonic`] Paper Backup.
//
// see [`KeyStore::restore`] for full docs.
// ### Safety
// this function assumes that:
// - `paper_key` is not null and a valid c string.
RawKeyStore keystore_restore(const char *paper_key);

// Get the KeyStore Secret Key as `Fixed32Array`.
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
OperationStatus keystore_secret_key(RawKeyStore ks, RawMutFixed32Array out);

// Get the KeyStore Seed as `Fixed32Array`.
//
// ### Safety
// this function assumes that:
// - `ks` is not null pointer to the `KeyStore`.
OperationStatus keystore_seed(RawKeyStore ks, RawMutFixed32Array out);

// Calculate SHA256 Hash of the provided file path.
//
// ### Safety
// this function assumes that:
// - `file_path` is not null pointer.
// - `out` is not null pointer.
OperationStatus keystore_sha256_hash(const char *file_path, RawMutFixed32Array out);

// Free (Drop) a string value allocated by Rust.
// ### Safety
// this assumes that the given pointer is not null.
void keystore_string_free(const char *ptr);
