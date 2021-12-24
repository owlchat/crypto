import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'allo_isolate.dart';
import 'generated/def.pb.dart';
import 'generated/google/protobuf/empty.pb.dart';
import 'utils.dart';
import 'ffi_ext.dart';

import 'ffi.dart' as ffi;

/// Owlchat Crypto Bining.
class OwlchatCrypto {
  final ffi.RawOwlchatCrypto _raw;
  late ffi.RawKeyPair _keyPair;

  /// Loads `Owlchat Crypto`'s [DynamicLibrary] depending on the current [Platform]
  ///
  /// Maybe throws [UnsupportedError] if the current [Platform]
  /// is not supported.
  OwlchatCrypto() : _raw = _load() {
    _keyPair = _raw.owlchat_crypto_keypair_new();
  }

  /// Returns the current [KeyPair] stored in the current [OwlchatCrypto] instance.
  Future<KeyPair> get currentKeyPair async {
    final req = Request(
      currentKeyPair: Empty(),
    );
    final res = await _dispatch(req);
    final keypair = res.ensureKeyPair();
    final maybeSeed = keypair.seed.isNotEmpty
        ? Seed(
            Uint8List.fromList(keypair.seed),
          )
        : null;
    assert(keypair.rawPointer.toInt() == _keyPair.address);
    return KeyPair(
      publicKey: PublicKey(Uint8List.fromList(keypair.publicKey)),
      secretKey: SecretKey(Uint8List.fromList(keypair.secretKey)),
      seed: maybeSeed,
    );
  }

  /// Initializes the [OwlchatCrypto] library with the given [SecretKey].
  ///
  /// You can access the [KeyPair] of the [OwlchatCrypto] instance using [currentKeyPair].
  Future<KeyPair> withSecretKey(SecretKey secretKey) async {
    final req = Request(
      initKeyPair: InitKeyPair(
        secretKey: secretKey.expose().cast(),
      ),
    );
    final res = await _dispatch(req);
    final keypair = res.ensureKeyPair();
    // free the original old keypair
    _raw.owlchat_crypto_keypair_drop(_keyPair);
    _keyPair = ffi.RawKeyPair.fromAddress(keypair.rawPointer.toInt());
    return KeyPair(
      publicKey: PublicKey(Uint8List.fromList(keypair.publicKey)),
      secretKey: SecretKey(Uint8List.fromList(keypair.secretKey)),
    );
  }

  /// Regenrates an new [KeyPair] and replaces the current [KeyPair] with it.
  Future<KeyPair> regenerateKeyPair() async {
    final req = Request(generateKeyPair: Empty());
    final res = await _dispatch(req);
    final keypair = res.ensureKeyPair();
    final publicKey = PublicKey(Uint8List.fromList(keypair.publicKey));
    final secretKey = SecretKey(Uint8List.fromList(keypair.secretKey));
    final seed = Seed(Uint8List.fromList(keypair.seed));
    // free the original old keypair
    _raw.owlchat_crypto_keypair_drop(_keyPair);
    _keyPair = ffi.RawKeyPair.fromAddress(keypair.rawPointer.toInt());
    return KeyPair(
      publicKey: publicKey,
      secretKey: secretKey,
      seed: seed,
    );
  }

  /// Restore the [KeyPair] of the [OwlchatCrypto] using the paper key (seed phrase).
  Future<KeyPair> restoreKeyPair(String paperKey) async {
    final req = Request(
      restoreKeyPair: RestoreKeyPair(
        paperKey: paperKey,
      ),
    );
    final res = await _dispatch(req);
    final keypair = res.ensureKeyPair();
    final publicKey = PublicKey(Uint8List.fromList(keypair.publicKey));
    final secretKey = SecretKey(Uint8List.fromList(keypair.secretKey));
    final maybeSeed = keypair.seed.isNotEmpty
        ? Seed(
            Uint8List.fromList(keypair.seed),
          )
        : null;
    // free the original old keypair
    _raw.owlchat_crypto_keypair_drop(_keyPair);
    _keyPair = ffi.RawKeyPair.fromAddress(keypair.rawPointer.toInt());
    return KeyPair(
      publicKey: publicKey,
      secretKey: secretKey,
      seed: maybeSeed,
    );
  }

  /// Backup the current [KeyPair] to Mnemonic Paperkey (Seed phrase).
  Future<String> backupKeyPair({Seed? seed}) async {
    final req = Request(
      backupKeyPair: BackupKeyPair(
        maybeSeed: seed?.expose().toList(growable: false),
      ),
    );
    final res = await _dispatch(req);
    return res.mnemonic;
  }

  /// Does Diffie-Hellman key exchange with the given [PublicKey]
  /// using the current [KeyPair] of the [OwlchatCrypto] instance.
  /// Returns the [SharedSecret].
  Future<SharedSecret> diffieHellmanKeyExchange(PublicKey publicKey) async {
    final req = Request(
      diffieHellmanKeyExchange: DiffieHellmanKeyExchange(
        theirPublicKey: publicKey.expose().cast(),
      ),
    );
    final res = await _dispatch(req);
    return SharedSecret(Uint8List.fromList(res.sharedSecret));
  }

  /// Encrypts the given [message] using the [KeyPair] of the [OwlchatCrypto] instance.
  /// Returns the encrypted [message] as a [Uint8List].
  Future<Uint8List> encrypt(
    Uint8List message, {
    SharedSecret? sharedSecret,
  }) async {
    final req = Request(
      encrypt: Encrypt(
        plaintext: message,
        secretKey: sharedSecret?.expose().toList(growable: false),
      ),
    );
    final res = await _dispatch(req);
    return Uint8List.fromList(res.encryptedMessage);
  }

  /// Decrypts the given [message] using the [KeyPair] of the [OwlchatCrypto] instance.
  /// Returns the decrypted [message] as a [Uint8List].
  Future<Uint8List> decrypt(
    Uint8List message, {
    SharedSecret? sharedSecret,
  }) async {
    final req = Request(
      decrypt: Decrypt(
        ciphertext: message,
        secretKey: sharedSecret?.expose().toList(growable: false),
      ),
    );
    final res = await _dispatch(req);
    return Uint8List.fromList(res.decryptedMessage);
  }

  /// Signs the given [message] using the [KeyPair] of the [OwlchatCrypto] instance.
  /// Returns the signature as a [Uint8List].
  Future<Uint8List> sign(Uint8List message) async {
    final req = Request(
      sign: Sign(
        msg: message,
      ),
    );
    final res = await _dispatch(req);
    return Uint8List.fromList(res.signature);
  }

  /// Verifies the given [signature] for the given [message] using the [Publickey]
  /// Returns true if the signature is valid and this [message] if signed with this public key, false otherwise.
  Future<bool> verify({
    required PublicKey theirPublicKey,
    required Uint8List message,
    required Uint8List signature,
  }) async {
    final req = Request(
      verify: Verify(
        msg: message,
        sig: signature,
        publicKey: theirPublicKey.expose().cast(),
      ),
    );
    final res = await _dispatch(req);
    return res.validSignature;
  }

  /// Checks if the given [paperKey] is a valid Mnemonic Paperkey (Seed phrase).
  /// Returns `true` if the [paperKey] is valid, `false` otherwise.
  Future<bool> validatePaperKey(String paperKey) async {
    final req = Request(
      validateMnemonic: ValidateMnemonic(
        phrase: paperKey,
      ),
    );
    final res = await _dispatch(req);
    return res.validMnemonic;
  }

  /// Hashes the given [input] using SHA256 and returns the result as a [Uint8List].
  Future<Uint8List> hash(Uint8List input) async {
    final req = Request(
      hashSha256: HashSha256(
        input: input,
      ),
    );
    final res = await _dispatch(req);
    return Uint8List.fromList(res.sha256Hash);
  }

  /// Hashes the given file stored at [path] using SHA256 and returns the result as a [Uint8List].
  Future<Uint8List> hashFile(Uri path) async {
    final req = Request(
      hashFileSha256: HashFileSha256(
        path: path.toString(),
      ),
    );
    final res = await _dispatch(req);
    return Uint8List.fromList(res.sha256Hash);
  }

  /// Dispose everything, clears the current [OwlchatCrypto] instance.
  void dispose() {
    _raw.owlchat_crypto_keypair_drop(_keyPair);
    // set the current keypair to null, we are done with it
    _keyPair = nullptr;
  }

  Future<Response> _dispatch(Request req) async {
    final completer = Completer<Uint8List>();
    final port = singleCompletePort(completer);
    final buffer = req.writeToBuffer();
    final data = buffer.asPtr();
    final len = buffer.length;
    final result = _raw.owlchat_crypto_dispatch(
      _keyPair,
      port.nativePort,
      data,
      len,
    );
    _assertOk(result);
    final resBytes = await completer.future;
    final res = Response.fromBuffer(resBytes);
    data.free();
    if (res.hasError()) {
      throw res.error;
    }
    return res;
  }
}

/// A [KeyPair] is a simple class that holds
/// [PublicKey], [SecretKey] and [Seed] if available.
class KeyPair {
  KeyPair({
    required this.publicKey,
    required this.secretKey,
    this.seed,
  });

  /// The Current `KeyStore`'s `PublicKey`.
  final PublicKey publicKey;

  /// The Current `KeyStore`'s `SecretKey`.
  final SecretKey secretKey;

  /// The Current `Secret`'s `Seed`.
  ///
  /// ### Note
  /// The Seed will be `null` in the case this `KeyPair` is
  /// is Inialized with `SecretKey`.
  final Seed? seed;
  // implemnt hash and =

}

/// A [Key] is an abstract class that shares functinality between
/// [PublicKey], [SecretKey], [SharedSecret] and [Seed].
abstract class Key {
  const Key(Uint8List key) : _inner = key;

  final Uint8List _inner;

  /// Convert The Key into base64 encoded string.
  String asBase64() {
    return base64.encode(_inner);
  }

  /// Expose the underlaying bytes.
  Uint8List expose() {
    return _inner;
  }

  @override
  int get hashCode => _inner.hashCode;

  @override
  bool operator ==(Object other) {
    if (other is Key) {
      return _inner == other._inner;
    } else {
      return false;
    }
  }
}

class PublicKey extends Key {
  const PublicKey(Uint8List key) : super(key);
  factory PublicKey.fromBase64(String b64) {
    final bytes = base64.decode(b64);
    return PublicKey(bytes);
  }

  // override hashCode and ==
  @override
  int get hashCode => _inner.hashCode;

  @override
  bool operator ==(Object other) {
    if (other is PublicKey) {
      return _inner == other._inner;
    } else {
      return false;
    }
  }
}

class SecretKey extends Key {
  const SecretKey(Uint8List key) : super(key);
  factory SecretKey.fromBase64(String b64) {
    final bytes = base64.decode(b64);
    return SecretKey(bytes);
  }

  // override hashCode and ==
  @override
  int get hashCode => _inner.hashCode;
  @override
  bool operator ==(Object other) {
    if (other is SecretKey) {
      return _inner == other._inner;
    } else {
      return false;
    }
  }
}

class SharedSecret extends Key {
  const SharedSecret(Uint8List key) : super(key);
  factory SharedSecret.fromBase64(String b64) {
    final bytes = base64.decode(b64);
    return SharedSecret(bytes);
  }

  // override hashCode and ==
  @override
  int get hashCode => _inner.hashCode;
  @override
  bool operator ==(Object other) {
    if (other is SharedSecret) {
      return _inner == other._inner;
    } else {
      return false;
    }
  }
}

class Seed extends Key {
  const Seed(Uint8List key) : super(key);

  factory Seed.fromBase64(String b64) {
    final bytes = base64.decode(b64);
    return Seed(bytes);
  }

  // override hashCode and ==
  @override
  int get hashCode => _inner.hashCode;
  @override
  bool operator ==(Object other) {
    if (other is Seed) {
      return _inner == other._inner;
    } else {
      return false;
    }
  }
}

void _assertOk(int result) {
  if (result != ffi.OwlchatResult.OwlchatResult_Ok) {
    throw Exception('Owlchat Crypto Error: $result');
  } else if (result == ffi.OwlchatResult.OwlchatResult_NullPointerDetected) {
    throw ArgumentError.notNull('_keyPair');
  } else if (result == ffi.OwlchatResult.OwlchatResult_InvalidProtobuf) {
    throw ArgumentError.value('<...>', 'protobuf', 'Invalid Protobuf data');
  }
}

/// Loads the Owlchat [DynamicLibrary] depending on the [Platform]
/// and creates new [ffi.RawOwlchatCrypto].
///
/// This will also Hook up the allo-isolate rust crate.
ffi.RawOwlchatCrypto _load() {
  late final DynamicLibrary lib;
  // Load the library depending on the platform
  // Supported platforms:
  // * Linux
  // * MacOS
  // * Windows
  // * Android
  // * iOS
  // * Fuchsia
  if (Platform.isLinux) {
    lib = DynamicLibrary.open('target/debug/libowlchat_crypto.so');
  } else if (Platform.isMacOS) {
    lib = DynamicLibrary.open('target/debug/libowlchat_crypto.dylib');
  } else if (Platform.isWindows) {
    lib = DynamicLibrary.open('target/debug/owlchat_crypto.dll');
  } else if (Platform.isAndroid) {
    lib = DynamicLibrary.open('libowlchat_crypto.so');
  } else if (Platform.isIOS) {
    lib = DynamicLibrary.executable();
  } else if (Platform.isFuchsia) {
    lib = DynamicLibrary.open('target/libowlchat_crypto.so');
  } else {
    throw UnsupportedError(
      'Platform ${Platform.operatingSystem} is not supported',
    );
  }
  // then hook up the allo-isolate rust crate
  // this is needed to be able to use the rust-ffi
  AlloIsolate(lib).hook();
  // finally create the ffi.RawOwlchatCrypto
  return ffi.RawOwlchatCrypto(lib);
}
