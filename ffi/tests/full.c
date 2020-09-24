#include "full.h"
#include <stdio.h>

const uint8_t *OWL_CHAT = (uint8_t *)"Owlchat";
const uintptr_t OWL_CHAT_LEN = sizeof(&OWL_CHAT) / sizeof(OWL_CHAT[0]);

void test_create_keystore() {
  println("test create_keystore ...");
  RawKeyStore ks = keystore_new();
  assert_not_null(ks);

  keystore_free(ks);
  println("test create_keystore ...ok");
}

void test_encrypt_decrypt() {
  println("test encrypt_decrypt ...");
  OperationStatus status = Unknwon;
  RawKeyStore ks = keystore_new();
  const uintptr_t total_len = OWL_CHAT_LEN + 12; // 12 for nonce
  SharedBuffer *shared_buffer = shared_buffer_new(total_len * 4);
  shared_buffer_append(shared_buffer, OWL_CHAT, OWL_CHAT_LEN);

  status = keystore_encrypt(ks, shared_buffer, NULL);
  assert_ok(status, "encrypt");
  status = keystore_decrypt(ks, shared_buffer, NULL);
  assert_ok(status, "decrypt");

  SharedBuffer *shared_buffer2 = shared_buffer_new(total_len * 4);
  shared_buffer_append(shared_buffer2, OWL_CHAT, OWL_CHAT_LEN);

  assert_shared_buffer_eq(shared_buffer, shared_buffer2);

  shared_buffer_free(shared_buffer);
  shared_buffer_free(shared_buffer2);
  keystore_free(ks);

  println("test encrypt_decrypt ...ok");
}

void test_keystore_init() {
  println("test keystore_init ...");
  RawKeyStore ks = keystore_new();
  OperationStatus status = Unknwon;

  const uintptr_t total_len = OWL_CHAT_LEN + 12; // 12 for nonce
  Fixed32Array *sk = fixed32_array_empty();

  SharedBuffer *shared_buffer = shared_buffer_new(total_len * 4);

  {
    shared_buffer_append(shared_buffer, OWL_CHAT, OWL_CHAT_LEN);
    status = keystore_encrypt(ks, shared_buffer, NULL);
    assert_ok(status, "encrypt");
    status = keystore_secret_key(ks, sk);
    assert_ok(status, "secret_key");
  }

  keystore_free(ks);

  {
    SharedBuffer *original = shared_buffer_new(total_len);
    shared_buffer_append(original, OWL_CHAT, OWL_CHAT_LEN);

    RawKeyStore ks = keystore_init(sk);
    fixed32_array_free(sk);
    assert_not_null(ks);
    status = keystore_decrypt(ks, shared_buffer, NULL);
    assert_ok(status, "decrypt");

    assert_shared_buffer_eq(original, shared_buffer);
    shared_buffer_free(original);
    keystore_free(ks);
  }
  shared_buffer_free(shared_buffer);
  println("test keystore_init ...ok");
}

void test_same_shared_secret() {
  println("test same_shared_secret ...");
  OperationStatus status = Unknwon;
  RawKeyStore alice_ks = keystore_new();
  RawKeyStore bob_ks = keystore_new();

  Fixed32Array *alice_pk = fixed32_array_empty();
  status = keystore_public_key(alice_ks, alice_pk);
  assert_ok(status, "alice_public_key");

  Fixed32Array *bob_pk = fixed32_array_empty();
  status = keystore_public_key(bob_ks, bob_pk);
  assert_ok(status, "bob_public_key");

  Fixed32Array *alice_ssk = fixed32_array_empty();
  status = keystore_dh(alice_ks, bob_pk, alice_ssk);
  assert_ok(status, "alice_dh");

  Fixed32Array *bob_ssk = fixed32_array_empty();
  status = keystore_dh(bob_ks, alice_pk, bob_ssk);
  assert_ok(status, "bob_dh");

  fixed32_array_free(alice_pk);
  fixed32_array_free(bob_pk);

  assert_fixed32_array_eq(alice_ssk, bob_ssk);

  fixed32_array_free(alice_ssk);
  fixed32_array_free(bob_ssk);

  keystore_free(alice_ks);
  keystore_free(bob_ks);

  println("test same_shared_secret ...ok");
}

void test_backup_and_restore() {
  println("test backup_and_restore ...");
  OperationStatus status = Unknwon;
  const char *paper_key = NULL;

  const uintptr_t total_len = OWL_CHAT_LEN + 12; // 12 for nonce
  SharedBuffer *shared_buffer = shared_buffer_new(total_len * 4);
  {
    RawKeyStore ks = keystore_new();
    paper_key = keystore_backup(ks, NULL);
    assert_not_null(paper_key);
    shared_buffer_append(shared_buffer, OWL_CHAT, OWL_CHAT_LEN);
    status = keystore_encrypt(ks, shared_buffer, NULL);
    assert_ok(status, "encrypt");
    keystore_free(ks);
  }

  {
    RawKeyStore ks = keystore_restore(paper_key);
    assert_not_null(ks);
    status = keystore_decrypt(ks, shared_buffer, NULL);
    assert_ok(status, "decrypt");
    keystore_free(ks);
  }

  SharedBuffer *original = shared_buffer_new(OWL_CHAT_LEN);
  shared_buffer_append(original, OWL_CHAT, OWL_CHAT_LEN);

  assert_shared_buffer_eq(original, shared_buffer);

  shared_buffer_free(shared_buffer);
  shared_buffer_free(original);

  keystore_string_free(paper_key);

  println("test backup_and_restore ...ok");
}

int main() {
  test_create_keystore();
  test_encrypt_decrypt();
  test_keystore_init();
  test_same_shared_secret();
  test_backup_and_restore();
  return 0;
}
