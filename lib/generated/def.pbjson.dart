///
//  Generated code. Do not modify.
//  source: def.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use keyPairDescriptor instead')
const KeyPair$json = const {
  '1': 'KeyPair',
  '2': const [
    const {'1': 'public_key', '3': 1, '4': 1, '5': 12, '10': 'publicKey'},
    const {'1': 'secret_key', '3': 2, '4': 1, '5': 12, '10': 'secretKey'},
    const {'1': 'seed', '3': 3, '4': 1, '5': 12, '10': 'seed'},
    const {'1': 'raw_pointer', '3': 4, '4': 1, '5': 3, '10': 'rawPointer'},
  ],
};

/// Descriptor for `KeyPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List keyPairDescriptor = $convert.base64Decode(
    'CgdLZXlQYWlyEh0KCnB1YmxpY19rZXkYASABKAxSCXB1YmxpY0tleRIdCgpzZWNyZXRfa2V5GAIgASgMUglzZWNyZXRLZXkSEgoEc2VlZBgDIAEoDFIEc2VlZBIfCgtyYXdfcG9pbnRlchgEIAEoA1IKcmF3UG9pbnRlcg==');
@$core.Deprecated('Use initKeyPairDescriptor instead')
const InitKeyPair$json = const {
  '1': 'InitKeyPair',
  '2': const [
    const {'1': 'secret_key', '3': 1, '4': 1, '5': 12, '10': 'secretKey'},
  ],
};

/// Descriptor for `InitKeyPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List initKeyPairDescriptor = $convert.base64Decode(
    'CgtJbml0S2V5UGFpchIdCgpzZWNyZXRfa2V5GAEgASgMUglzZWNyZXRLZXk=');
@$core.Deprecated('Use restoreKeyPairDescriptor instead')
const RestoreKeyPair$json = const {
  '1': 'RestoreKeyPair',
  '2': const [
    const {'1': 'paper_key', '3': 1, '4': 1, '5': 9, '10': 'paperKey'},
  ],
};

/// Descriptor for `RestoreKeyPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List restoreKeyPairDescriptor = $convert.base64Decode(
    'Cg5SZXN0b3JlS2V5UGFpchIbCglwYXBlcl9rZXkYASABKAlSCHBhcGVyS2V5');
@$core.Deprecated('Use backupKeyPairDescriptor instead')
const BackupKeyPair$json = const {
  '1': 'BackupKeyPair',
  '2': const [
    const {'1': 'maybe_seed', '3': 1, '4': 1, '5': 12, '10': 'maybeSeed'},
  ],
};

/// Descriptor for `BackupKeyPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List backupKeyPairDescriptor = $convert.base64Decode(
    'Cg1CYWNrdXBLZXlQYWlyEh0KCm1heWJlX3NlZWQYASABKAxSCW1heWJlU2VlZA==');
@$core.Deprecated('Use validateMnemonicDescriptor instead')
const ValidateMnemonic$json = const {
  '1': 'ValidateMnemonic',
  '2': const [
    const {'1': 'phrase', '3': 1, '4': 1, '5': 9, '10': 'phrase'},
  ],
};

/// Descriptor for `ValidateMnemonic`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List validateMnemonicDescriptor = $convert
    .base64Decode('ChBWYWxpZGF0ZU1uZW1vbmljEhYKBnBocmFzZRgBIAEoCVIGcGhyYXNl');
@$core.Deprecated('Use encryptDescriptor instead')
const Encrypt$json = const {
  '1': 'Encrypt',
  '2': const [
    const {'1': 'plaintext', '3': 1, '4': 1, '5': 12, '10': 'plaintext'},
    const {'1': 'secret_key', '3': 2, '4': 1, '5': 12, '10': 'secretKey'},
  ],
};

/// Descriptor for `Encrypt`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List encryptDescriptor = $convert.base64Decode(
    'CgdFbmNyeXB0EhwKCXBsYWludGV4dBgBIAEoDFIJcGxhaW50ZXh0Eh0KCnNlY3JldF9rZXkYAiABKAxSCXNlY3JldEtleQ==');
@$core.Deprecated('Use decryptDescriptor instead')
const Decrypt$json = const {
  '1': 'Decrypt',
  '2': const [
    const {'1': 'ciphertext', '3': 1, '4': 1, '5': 12, '10': 'ciphertext'},
    const {'1': 'secret_key', '3': 2, '4': 1, '5': 12, '10': 'secretKey'},
  ],
};

/// Descriptor for `Decrypt`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List decryptDescriptor = $convert.base64Decode(
    'CgdEZWNyeXB0Eh4KCmNpcGhlcnRleHQYASABKAxSCmNpcGhlcnRleHQSHQoKc2VjcmV0X2tleRgCIAEoDFIJc2VjcmV0S2V5');
@$core.Deprecated('Use signDescriptor instead')
const Sign$json = const {
  '1': 'Sign',
  '2': const [
    const {'1': 'msg', '3': 1, '4': 1, '5': 12, '10': 'msg'},
  ],
};

/// Descriptor for `Sign`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signDescriptor =
    $convert.base64Decode('CgRTaWduEhAKA21zZxgBIAEoDFIDbXNn');
@$core.Deprecated('Use verifyDescriptor instead')
const Verify$json = const {
  '1': 'Verify',
  '2': const [
    const {'1': 'public_key', '3': 1, '4': 1, '5': 12, '10': 'publicKey'},
    const {'1': 'msg', '3': 2, '4': 1, '5': 12, '10': 'msg'},
    const {'1': 'sig', '3': 3, '4': 1, '5': 12, '10': 'sig'},
  ],
};

/// Descriptor for `Verify`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyDescriptor = $convert.base64Decode(
    'CgZWZXJpZnkSHQoKcHVibGljX2tleRgBIAEoDFIJcHVibGljS2V5EhAKA21zZxgCIAEoDFIDbXNnEhAKA3NpZxgDIAEoDFIDc2ln');
@$core.Deprecated('Use diffieHellmanKeyExchangeDescriptor instead')
const DiffieHellmanKeyExchange$json = const {
  '1': 'DiffieHellmanKeyExchange',
  '2': const [
    const {
      '1': 'their_public_key',
      '3': 1,
      '4': 1,
      '5': 12,
      '10': 'theirPublicKey'
    },
  ],
};

/// Descriptor for `DiffieHellmanKeyExchange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List diffieHellmanKeyExchangeDescriptor =
    $convert.base64Decode(
        'ChhEaWZmaWVIZWxsbWFuS2V5RXhjaGFuZ2USKAoQdGhlaXJfcHVibGljX2tleRgBIAEoDFIOdGhlaXJQdWJsaWNLZXk=');
@$core.Deprecated('Use hashSha256Descriptor instead')
const HashSha256$json = const {
  '1': 'HashSha256',
  '2': const [
    const {'1': 'input', '3': 1, '4': 1, '5': 12, '10': 'input'},
  ],
};

/// Descriptor for `HashSha256`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List hashSha256Descriptor =
    $convert.base64Decode('CgpIYXNoU2hhMjU2EhQKBWlucHV0GAEgASgMUgVpbnB1dA==');
@$core.Deprecated('Use hashFileSha256Descriptor instead')
const HashFileSha256$json = const {
  '1': 'HashFileSha256',
  '2': const [
    const {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `HashFileSha256`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List hashFileSha256Descriptor =
    $convert.base64Decode('Cg5IYXNoRmlsZVNoYTI1NhISCgRwYXRoGAEgASgJUgRwYXRo');
@$core.Deprecated('Use requestDescriptor instead')
const Request$json = const {
  '1': 'Request',
  '2': const [
    const {
      '1': 'current_key_pair',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.google.protobuf.Empty',
      '9': 0,
      '10': 'currentKeyPair'
    },
    const {
      '1': 'generate_key_pair',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.google.protobuf.Empty',
      '9': 0,
      '10': 'generateKeyPair'
    },
    const {
      '1': 'init_key_pair',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.owlchat.InitKeyPair',
      '9': 0,
      '10': 'initKeyPair'
    },
    const {
      '1': 'restore_key_pair',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.owlchat.RestoreKeyPair',
      '9': 0,
      '10': 'restoreKeyPair'
    },
    const {
      '1': 'backup_key_pair',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.owlchat.BackupKeyPair',
      '9': 0,
      '10': 'backupKeyPair'
    },
    const {
      '1': 'validate_mnemonic',
      '3': 6,
      '4': 1,
      '5': 11,
      '6': '.owlchat.ValidateMnemonic',
      '9': 0,
      '10': 'validateMnemonic'
    },
    const {
      '1': 'encrypt',
      '3': 7,
      '4': 1,
      '5': 11,
      '6': '.owlchat.Encrypt',
      '9': 0,
      '10': 'encrypt'
    },
    const {
      '1': 'decrypt',
      '3': 8,
      '4': 1,
      '5': 11,
      '6': '.owlchat.Decrypt',
      '9': 0,
      '10': 'decrypt'
    },
    const {
      '1': 'sign',
      '3': 9,
      '4': 1,
      '5': 11,
      '6': '.owlchat.Sign',
      '9': 0,
      '10': 'sign'
    },
    const {
      '1': 'verify',
      '3': 10,
      '4': 1,
      '5': 11,
      '6': '.owlchat.Verify',
      '9': 0,
      '10': 'verify'
    },
    const {
      '1': 'diffie_hellman_key_exchange',
      '3': 11,
      '4': 1,
      '5': 11,
      '6': '.owlchat.DiffieHellmanKeyExchange',
      '9': 0,
      '10': 'diffieHellmanKeyExchange'
    },
    const {
      '1': 'hash_sha256',
      '3': 12,
      '4': 1,
      '5': 11,
      '6': '.owlchat.HashSha256',
      '9': 0,
      '10': 'hashSha256'
    },
    const {
      '1': 'hash_file_sha256',
      '3': 13,
      '4': 1,
      '5': 11,
      '6': '.owlchat.HashFileSha256',
      '9': 0,
      '10': 'hashFileSha256'
    },
  ],
  '8': const [
    const {'1': 'body'},
  ],
};

/// Descriptor for `Request`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestDescriptor = $convert.base64Decode(
    'CgdSZXF1ZXN0EkIKEGN1cnJlbnRfa2V5X3BhaXIYASABKAsyFi5nb29nbGUucHJvdG9idWYuRW1wdHlIAFIOY3VycmVudEtleVBhaXISRAoRZ2VuZXJhdGVfa2V5X3BhaXIYAiABKAsyFi5nb29nbGUucHJvdG9idWYuRW1wdHlIAFIPZ2VuZXJhdGVLZXlQYWlyEjoKDWluaXRfa2V5X3BhaXIYAyABKAsyFC5vd2xjaGF0LkluaXRLZXlQYWlySABSC2luaXRLZXlQYWlyEkMKEHJlc3RvcmVfa2V5X3BhaXIYBCABKAsyFy5vd2xjaGF0LlJlc3RvcmVLZXlQYWlySABSDnJlc3RvcmVLZXlQYWlyEkAKD2JhY2t1cF9rZXlfcGFpchgFIAEoCzIWLm93bGNoYXQuQmFja3VwS2V5UGFpckgAUg1iYWNrdXBLZXlQYWlyEkgKEXZhbGlkYXRlX21uZW1vbmljGAYgASgLMhkub3dsY2hhdC5WYWxpZGF0ZU1uZW1vbmljSABSEHZhbGlkYXRlTW5lbW9uaWMSLAoHZW5jcnlwdBgHIAEoCzIQLm93bGNoYXQuRW5jcnlwdEgAUgdlbmNyeXB0EiwKB2RlY3J5cHQYCCABKAsyEC5vd2xjaGF0LkRlY3J5cHRIAFIHZGVjcnlwdBIjCgRzaWduGAkgASgLMg0ub3dsY2hhdC5TaWduSABSBHNpZ24SKQoGdmVyaWZ5GAogASgLMg8ub3dsY2hhdC5WZXJpZnlIAFIGdmVyaWZ5EmIKG2RpZmZpZV9oZWxsbWFuX2tleV9leGNoYW5nZRgLIAEoCzIhLm93bGNoYXQuRGlmZmllSGVsbG1hbktleUV4Y2hhbmdlSABSGGRpZmZpZUhlbGxtYW5LZXlFeGNoYW5nZRI2CgtoYXNoX3NoYTI1NhgMIAEoCzITLm93bGNoYXQuSGFzaFNoYTI1NkgAUgpoYXNoU2hhMjU2EkMKEGhhc2hfZmlsZV9zaGEyNTYYDSABKAsyFy5vd2xjaGF0Lkhhc2hGaWxlU2hhMjU2SABSDmhhc2hGaWxlU2hhMjU2QgYKBGJvZHk=');
@$core.Deprecated('Use responseDescriptor instead')
const Response$json = const {
  '1': 'Response',
  '2': const [
    const {
      '1': 'error',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.owlchat.Response.Error',
      '9': 0,
      '10': 'error'
    },
    const {
      '1': 'key_pair',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.owlchat.KeyPair',
      '9': 0,
      '10': 'keyPair'
    },
    const {'1': 'mnemonic', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'mnemonic'},
    const {
      '1': 'valid_mnemonic',
      '3': 4,
      '4': 1,
      '5': 8,
      '9': 0,
      '10': 'validMnemonic'
    },
    const {
      '1': 'encrypted_message',
      '3': 5,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'encryptedMessage'
    },
    const {
      '1': 'decrypted_message',
      '3': 6,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'decryptedMessage'
    },
    const {
      '1': 'signature',
      '3': 7,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'signature'
    },
    const {
      '1': 'valid_signature',
      '3': 8,
      '4': 1,
      '5': 8,
      '9': 0,
      '10': 'validSignature'
    },
    const {
      '1': 'shared_secret',
      '3': 9,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'sharedSecret'
    },
    const {
      '1': 'sha256_hash',
      '3': 10,
      '4': 1,
      '5': 12,
      '9': 0,
      '10': 'sha256Hash'
    },
  ],
  '3': const [Response_Error$json],
  '8': const [
    const {'1': 'body'},
  ],
};

@$core.Deprecated('Use responseDescriptor instead')
const Response_Error$json = const {
  '1': 'Error',
  '2': const [
    const {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
    const {
      '1': 'kind',
      '3': 2,
      '4': 1,
      '5': 14,
      '6': '.owlchat.Response.Error.Kind',
      '10': 'kind'
    },
  ],
  '4': const [Response_Error_Kind$json],
};

@$core.Deprecated('Use responseDescriptor instead')
const Response_Error_Kind$json = const {
  '1': 'Kind',
  '2': const [
    const {'1': 'UNKNOWN', '2': 0},
    const {'1': 'MISSING_REQUEST_BODY', '2': 1},
    const {'1': 'INVALID_PUBLIC_KEY', '2': 2},
    const {'1': 'INVALID_SECRET_KEY', '2': 3},
    const {'1': 'INVALID_SIGNATURE', '2': 4},
    const {'1': 'INVALID_SEED', '2': 5},
    const {'1': 'INVALID_PAPER_KEY', '2': 6},
    const {'1': 'NOT_INITIALIZED', '2': 7},
  ],
};

/// Descriptor for `Response`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List responseDescriptor = $convert.base64Decode(
    'CghSZXNwb25zZRIvCgVlcnJvchgBIAEoCzIXLm93bGNoYXQuUmVzcG9uc2UuRXJyb3JIAFIFZXJyb3ISLQoIa2V5X3BhaXIYAiABKAsyEC5vd2xjaGF0LktleVBhaXJIAFIHa2V5UGFpchIcCghtbmVtb25pYxgDIAEoCUgAUghtbmVtb25pYxInCg52YWxpZF9tbmVtb25pYxgEIAEoCEgAUg12YWxpZE1uZW1vbmljEi0KEWVuY3J5cHRlZF9tZXNzYWdlGAUgASgMSABSEGVuY3J5cHRlZE1lc3NhZ2USLQoRZGVjcnlwdGVkX21lc3NhZ2UYBiABKAxIAFIQZGVjcnlwdGVkTWVzc2FnZRIeCglzaWduYXR1cmUYByABKAxIAFIJc2lnbmF0dXJlEikKD3ZhbGlkX3NpZ25hdHVyZRgIIAEoCEgAUg52YWxpZFNpZ25hdHVyZRIlCg1zaGFyZWRfc2VjcmV0GAkgASgMSABSDHNoYXJlZFNlY3JldBIhCgtzaGEyNTZfaGFzaBgKIAEoDEgAUgpzaGEyNTZIYXNoGogCCgVFcnJvchIYCgdtZXNzYWdlGAEgASgJUgdtZXNzYWdlEjAKBGtpbmQYAiABKA4yHC5vd2xjaGF0LlJlc3BvbnNlLkVycm9yLktpbmRSBGtpbmQisgEKBEtpbmQSCwoHVU5LTk9XThAAEhgKFE1JU1NJTkdfUkVRVUVTVF9CT0RZEAESFgoSSU5WQUxJRF9QVUJMSUNfS0VZEAISFgoSSU5WQUxJRF9TRUNSRVRfS0VZEAMSFQoRSU5WQUxJRF9TSUdOQVRVUkUQBBIQCgxJTlZBTElEX1NFRUQQBRIVChFJTlZBTElEX1BBUEVSX0tFWRAGEhMKD05PVF9JTklUSUFMSVpFRBAHQgYKBGJvZHk=');
