
#include "full.h"
#include <stdio.h>

const char OWL_CHAT[] = "Owlchat";

void test_create_keypair()
{
  println("test create_keystore ...");
  auto result = owlchat_crypto_init();
  assert(result == OwlchatResult::Ok);
  owlchat_crypto_destory();
  println("test create_keypair ...ok");
}

void test_encrypt_decrypt()
{
  println("test encrypt_decrypt ...");
  auto result = owlchat_crypto_init();
  assert(result == OwlchatResult::Ok);
  owlchat::Request req;
  owlchat::Encrypt encypt;
  auto text = std::string(OWL_CHAT);
  encypt.set_allocated_plaintext(&text);
  req.set_allocated_encrypt(&encypt);
  auto size = req.ByteSizeLong();
  auto *buffer = (uint8_t *)malloc(size);
  req.SerializeToArray(buffer, size);
  auto buf = owlchat_crypto_dispatch(buffer, size);
  assert(buf.data != nullptr);
  owlchat_crypto_destory();
  owlchat_crypto_free_buffer(buf);
  println("test encrypt_decrypt ...ok");
}

int main()
{
  GOOGLE_PROTOBUF_VERIFY_VERSION;
  test_create_keypair();
  test_encrypt_decrypt();
  google::protobuf::ShutdownProtobufLibrary();
  return 0;
}
