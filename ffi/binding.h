#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const void *RawKeyStore;

typedef struct {
  uint8_t *data;
} Fixed32Array;

typedef struct {
  uint8_t *buf;
  uintptr_t len;
  uintptr_t cap;
} SharedBuffer;

/**
 * Create a [`Mnemonic`] Backup from the provided seed (or the keystore seed if exist).
 *
 * the caller should call [`keystore_string_free`] after being done with it.
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 * - if `seed` is empty, it will try to use the `KeyStore` seed if available.
 *
 * otherwise it will return null.
 */
const char *keystore_backup(RawKeyStore ks, Fixed32Array seed);

/**
 * Decrypt the Given data using `KeyStore` owned `SecretKey`
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 * - if `shared_secret` is empty, it will use the `KeyStore` secret key.
 */
int32_t keystore_decrypt(RawKeyStore ks, SharedBuffer *data, Fixed32Array shared_secret);

/**
 * Perform a Diffie-Hellman key agreement to produce a `SharedSecret`.
 *
 * see [`KeyStore::dh`] for full docs.
 * the caller should call [`keystore_fixed32_array_free`] after being done with it.
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 *
 * otherwise it will return `null`.
 */
Fixed32Array keystore_dh(RawKeyStore ks, Fixed32Array their_public);

/**
 * Encrypt the Given data using `KeyStore` owned `SecretKey`
 *
 * the caller should call [`keystore_cvec_free`] after being done with the pointer.
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 * - if `shared_secret` is empty, it will use the `KeyStore` secret key.
 */
int32_t keystore_encrypt(RawKeyStore ks, SharedBuffer *data, Fixed32Array shared_secret);

/**
 * Free (Drop) a Fixed32Array value allocated by Rust.
 * ### Safety
 * this assumes that the given pointer is not null.
 */
void keystore_fixed32_array_free(Fixed32Array ptr);

/**
 * Free (Drop) the created KeyStore.
 * ### Safety
 * this assumes that the given pointer is not null.
 */
void keystore_free(RawKeyStore ks);

/**
 * Init the `KeyStore` with existing SecretKey Bytes.
 *
 * See [`KeyStore::init`] for full docs.
 */
RawKeyStore keystore_init(Fixed32Array secret_key);

/**
 * Create a new [`KeyStore`].
 *
 * See [`KeyStore::new`] for full docs.
 */
RawKeyStore keystore_new(void);

/**
 * Get the KeyStore Public Key as `Fixed32Array`.
 * the caller should call [`keystore_fixed32_array_free`] after being done with it.
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 *
 * otherwise it will return Empty Fixed32Array.
 */
Fixed32Array keystore_public_key(RawKeyStore ks);

/**
 * Restore a `KeyStore` from a [`Mnemonic`] Paper Backup.
 *
 * see [`KeyStore::restore`] for full docs.
 * ### Safety
 * this function assumes that:
 * - `paper_key` is not null and a valid c string.
 */
RawKeyStore keystore_restore(const char *paper_key);

/**
 * Get the KeyStore Secret Key as `Fixed32Array`.
 * the caller should call [`keystore_fixed32_array_free`] after being done with it.
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 *
 * otherwise it will return Empty Fixed32Array.
 */
Fixed32Array keystore_secret_key(RawKeyStore ks);

/**
 * Get the KeyStore Seed as `Fixed32Array`, returns Empty Fixed32Array if there is no seed.
 * the caller should call [`keystore_fixed32_array_free`] after being done with it.
 *
 * ### Safety
 * this function assumes that:
 * - `ks` is not null pointer to the `KeyStore`.
 *
 * otherwise it will return Empty Fixed32Array.
 */
Fixed32Array keystore_seed(RawKeyStore ks);

/**
 * Free (Drop) a string value allocated by Rust.
 * ### Safety
 * this assumes that the given pointer is not null.
 */
void keystore_string_free(const char *ptr);
