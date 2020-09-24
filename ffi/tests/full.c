#include "full.h"
#include <stdio.h>

#pragma clang diagnostic ignored "-Wformat"

void test_create_keystore() {
  println("Creating KeyStore");
  RawKeyStore ks = keystore_new();
  println("Free KeyStore");
  keystore_free(ks);
}

void test_encrypt_decrypt() {
  println("Creating KeyStore");
  RawKeyStore ks = keystore_new();

  const uint8_t *msg = (uint8_t *)"Owlchat";
  const uintptr_t msg_len = sizeof(&msg) / sizeof(msg[0]);
  const uintptr_t total_len = msg_len + 12; // 12 for nonce
  printf("Total Length: %llu\n", total_len);

  SharedBuffer *shared_buffer = shared_buffer_new(total_len * 4);
  shared_buffer_append(shared_buffer, msg, msg_len);
  print_shared_buffer(shared_buffer);

  Fixed32Array *secret_key = malloc(sizeof(Fixed32Array));
  secret_key->data = NULL;
  keystore_encrypt(ks, shared_buffer, *secret_key);
  hex_dump_shared_buffer(shared_buffer);

  keystore_decrypt(ks, shared_buffer, *secret_key);
  print_shared_buffer(shared_buffer);

  SharedBuffer *shared_buffer2 = shared_buffer_new(total_len * 4);
  shared_buffer_append(shared_buffer2, msg, msg_len);

  assert_shared_buffer_eq(shared_buffer, shared_buffer2);

  shared_buffer_free(shared_buffer);
  shared_buffer_free(shared_buffer2);
  free(secret_key);
  println("Free KeyStore");
  keystore_free(ks);
}

int main() {
  test_create_keystore();
  test_encrypt_decrypt();
  return 0;
}
