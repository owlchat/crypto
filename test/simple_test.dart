import 'package:flutter/services.dart';
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
    final keypair = await owlchatCrypto.generateKeyPair();
    expect(keypair.publicKey, isA<PublicKey>());
    expect(keypair.publicKey.expose().length, 32);
    expect(keypair.secretKey.expose().length, 32);
    expect(keypair.seed, isA<Seed>());
  });
}
