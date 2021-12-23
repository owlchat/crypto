import 'package:flutter_test/flutter_test.dart';
import 'package:owlchat_crypto/owlchat_crypto.dart';

void main() {
  late final OwlchatCrypto owlchatCrypto;
  setUp(() {
    owlchatCrypto = OwlchatCrypto();
  });

  tearDown(() {
    owlchatCrypto.dispose();
  });

  test('should generate new keypair', () async {
    final keypair = await owlchatCrypto.regenerateKeyPair();
    expect(keypair.publicKey, isA<PublicKey>());
    expect(keypair.publicKey.expose().length, 32);
    expect(keypair.secretKey.expose().length, 32);
    expect(keypair.seed, isA<Seed>());
  });

  test('should backup and restore the same key', () async {
    final keypair = await owlchatCrypto.regenerateKeyPair();
    final paperKey = await owlchatCrypto.backupKeyPair(seed: keypair.seed);
    final restoredKeypair = await owlchatCrypto.restoreKeyPair(paperKey);
    expect(restoredKeypair.publicKey.asBase64(), keypair.publicKey.asBase64());
    expect(restoredKeypair.secretKey.asBase64(), keypair.secretKey.asBase64());
    expect(restoredKeypair.seed?.asBase64(), keypair.seed?.asBase64());
  });
}
