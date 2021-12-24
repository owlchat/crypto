import 'dart:typed_data';
import 'package:convert/convert.dart';

import 'package:flutter_test/flutter_test.dart';
import 'package:owlchat_crypto/owlchat_crypto.dart';

void main() {
  test('should generate new keypair', () async {
    final crypto = OwlchatCrypto();
    final keypair = await crypto.regenerateKeyPair();
    expect(keypair.publicKey, isA<PublicKey>());
    expect(keypair.publicKey.expose().length, 32);
    expect(keypair.secretKey.expose().length, 32);
    expect(keypair.seed, isA<Seed>());
    crypto.dispose();
  });

  test('should backup and restore the same key', () async {
    final crypto = OwlchatCrypto();
    final keypair = await crypto.regenerateKeyPair();
    final paperKey = await crypto.backupKeyPair(seed: keypair.seed);
    final isValidPaperkey = await crypto.validatePaperKey(paperKey);
    expect(isValidPaperkey, isTrue);
    final restoredKeypair = await crypto.restoreKeyPair(paperKey);
    expect(restoredKeypair.publicKey.asBase64(), keypair.publicKey.asBase64());
    expect(restoredKeypair.secretKey.asBase64(), keypair.secretKey.asBase64());
    expect(restoredKeypair.seed?.asBase64(), keypair.seed?.asBase64());
    crypto.dispose();
  });

  test('should encrypt and decrypt', () async {
    final crypto = OwlchatCrypto();
    await crypto.regenerateKeyPair();
    const message = 'Hello World';
    final encrypted = await crypto.encrypt(
      Uint8List.fromList(message.codeUnits),
    );
    final decrypted = await crypto.decrypt(encrypted);
    expect(String.fromCharCodes(decrypted), message);
    crypto.dispose();
  });

  test('should sign and verify', () async {
    final crypto = OwlchatCrypto();
    final keypair = await crypto.regenerateKeyPair();
    const message = 'Hello World';
    final signature = await crypto.sign(
      Uint8List.fromList(message.codeUnits),
    );
    final verified = await crypto.verify(
      theirPublicKey: keypair.publicKey,
      message: Uint8List.fromList(message.codeUnits),
      signature: signature,
    );
    expect(verified, isTrue);
    crypto.dispose();
  });

  test('conversation between two crypto', () async {
    final aliceCrypto = OwlchatCrypto();
    final bobCrypto = OwlchatCrypto();
    final aliceKeypair = await aliceCrypto.currentKeyPair;
    final bobKeypair = await bobCrypto.currentKeyPair;
    // sanity check
    expect(
      aliceKeypair.publicKey.asBase64() == bobKeypair.publicKey.asBase64(),
      isFalse,
    );
    // fist we do a handshake
    final aliceSharedSecret = await aliceCrypto.diffieHellmanKeyExchange(
      bobKeypair.publicKey,
    );
    // alice creates a message and encrypt it
    const message = 'Hello Bob!';
    final aliceEncrypted = await aliceCrypto.encrypt(
      Uint8List.fromList(message.codeUnits),
      sharedSecret: aliceSharedSecret,
    );
    // also we need to sign the message
    final aliceSignature = await aliceCrypto.sign(aliceEncrypted);

    // bob receives the message and the signature
    // first we verify the signature
    final bobVerified = await bobCrypto.verify(
      theirPublicKey: aliceKeypair.publicKey,
      message: aliceEncrypted,
      signature: aliceSignature,
    );
    expect(bobVerified, isTrue);
    // then we create our shared secret
    final bobSharedSecret = await bobCrypto.diffieHellmanKeyExchange(
      aliceKeypair.publicKey,
    );
    // both shared secret are the same
    expect(aliceSharedSecret.asBase64(), bobSharedSecret.asBase64());
    // bob decrypts the message
    final bobDecrypted = await bobCrypto.decrypt(
      aliceEncrypted,
      sharedSecret: bobSharedSecret,
    );
    // we should get the same message
    expect(String.fromCharCodes(bobDecrypted), message);
    aliceCrypto.dispose();
    bobCrypto.dispose();
  });

  test('should be able to hash simple string', () async {
    final crypto = OwlchatCrypto();
    final hash = await crypto.hash(Uint8List.fromList('Hello World'.codeUnits));
    expect(hash.length, 32);
    expect(
      hex.encode(hash),
      "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e",
    );
    crypto.dispose();
  });
}
