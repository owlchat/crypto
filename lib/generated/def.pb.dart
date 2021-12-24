///
//  Generated code. Do not modify.
//  source: def.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'google/protobuf/empty.pb.dart' as $0;

import 'def.pbenum.dart';

export 'def.pbenum.dart';

class KeyPair extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'KeyPair',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'publicKey',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'secretKey',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'seed',
        $pb.PbFieldType.OY)
    ..aInt64(
        4,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'rawPointer')
    ..hasRequiredFields = false;

  KeyPair._() : super();
  factory KeyPair({
    $core.List<$core.int>? publicKey,
    $core.List<$core.int>? secretKey,
    $core.List<$core.int>? seed,
    $fixnum.Int64? rawPointer,
  }) {
    final _result = create();
    if (publicKey != null) {
      _result.publicKey = publicKey;
    }
    if (secretKey != null) {
      _result.secretKey = secretKey;
    }
    if (seed != null) {
      _result.seed = seed;
    }
    if (rawPointer != null) {
      _result.rawPointer = rawPointer;
    }
    return _result;
  }
  factory KeyPair.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory KeyPair.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  KeyPair clone() => KeyPair()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  KeyPair copyWith(void Function(KeyPair) updates) =>
      super.copyWith((message) => updates(message as KeyPair))
          as KeyPair; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static KeyPair create() => KeyPair._();
  KeyPair createEmptyInstance() => create();
  static $pb.PbList<KeyPair> createRepeated() => $pb.PbList<KeyPair>();
  @$core.pragma('dart2js:noInline')
  static KeyPair getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<KeyPair>(create);
  static KeyPair? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get publicKey => $_getN(0);
  @$pb.TagNumber(1)
  set publicKey($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPublicKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPublicKey() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get secretKey => $_getN(1);
  @$pb.TagNumber(2)
  set secretKey($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSecretKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecretKey() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get seed => $_getN(2);
  @$pb.TagNumber(3)
  set seed($core.List<$core.int> v) {
    $_setBytes(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasSeed() => $_has(2);
  @$pb.TagNumber(3)
  void clearSeed() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get rawPointer => $_getI64(3);
  @$pb.TagNumber(4)
  set rawPointer($fixnum.Int64 v) {
    $_setInt64(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasRawPointer() => $_has(3);
  @$pb.TagNumber(4)
  void clearRawPointer() => clearField(4);
}

class InitKeyPair extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'InitKeyPair',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'secretKey',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  InitKeyPair._() : super();
  factory InitKeyPair({
    $core.List<$core.int>? secretKey,
  }) {
    final _result = create();
    if (secretKey != null) {
      _result.secretKey = secretKey;
    }
    return _result;
  }
  factory InitKeyPair.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory InitKeyPair.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  InitKeyPair clone() => InitKeyPair()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  InitKeyPair copyWith(void Function(InitKeyPair) updates) =>
      super.copyWith((message) => updates(message as InitKeyPair))
          as InitKeyPair; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static InitKeyPair create() => InitKeyPair._();
  InitKeyPair createEmptyInstance() => create();
  static $pb.PbList<InitKeyPair> createRepeated() => $pb.PbList<InitKeyPair>();
  @$core.pragma('dart2js:noInline')
  static InitKeyPair getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InitKeyPair>(create);
  static InitKeyPair? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get secretKey => $_getN(0);
  @$pb.TagNumber(1)
  set secretKey($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSecretKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecretKey() => clearField(1);
}

class RestoreKeyPair extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RestoreKeyPair',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'paperKey')
    ..hasRequiredFields = false;

  RestoreKeyPair._() : super();
  factory RestoreKeyPair({
    $core.String? paperKey,
  }) {
    final _result = create();
    if (paperKey != null) {
      _result.paperKey = paperKey;
    }
    return _result;
  }
  factory RestoreKeyPair.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RestoreKeyPair.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RestoreKeyPair clone() => RestoreKeyPair()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RestoreKeyPair copyWith(void Function(RestoreKeyPair) updates) =>
      super.copyWith((message) => updates(message as RestoreKeyPair))
          as RestoreKeyPair; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RestoreKeyPair create() => RestoreKeyPair._();
  RestoreKeyPair createEmptyInstance() => create();
  static $pb.PbList<RestoreKeyPair> createRepeated() =>
      $pb.PbList<RestoreKeyPair>();
  @$core.pragma('dart2js:noInline')
  static RestoreKeyPair getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RestoreKeyPair>(create);
  static RestoreKeyPair? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get paperKey => $_getSZ(0);
  @$pb.TagNumber(1)
  set paperKey($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPaperKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPaperKey() => clearField(1);
}

class BackupKeyPair extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'BackupKeyPair',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'maybeSeed',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  BackupKeyPair._() : super();
  factory BackupKeyPair({
    $core.List<$core.int>? maybeSeed,
  }) {
    final _result = create();
    if (maybeSeed != null) {
      _result.maybeSeed = maybeSeed;
    }
    return _result;
  }
  factory BackupKeyPair.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory BackupKeyPair.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  BackupKeyPair clone() => BackupKeyPair()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  BackupKeyPair copyWith(void Function(BackupKeyPair) updates) =>
      super.copyWith((message) => updates(message as BackupKeyPair))
          as BackupKeyPair; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BackupKeyPair create() => BackupKeyPair._();
  BackupKeyPair createEmptyInstance() => create();
  static $pb.PbList<BackupKeyPair> createRepeated() =>
      $pb.PbList<BackupKeyPair>();
  @$core.pragma('dart2js:noInline')
  static BackupKeyPair getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BackupKeyPair>(create);
  static BackupKeyPair? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get maybeSeed => $_getN(0);
  @$pb.TagNumber(1)
  set maybeSeed($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasMaybeSeed() => $_has(0);
  @$pb.TagNumber(1)
  void clearMaybeSeed() => clearField(1);
}

class ValidateMnemonic extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ValidateMnemonic',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'phrase')
    ..hasRequiredFields = false;

  ValidateMnemonic._() : super();
  factory ValidateMnemonic({
    $core.String? phrase,
  }) {
    final _result = create();
    if (phrase != null) {
      _result.phrase = phrase;
    }
    return _result;
  }
  factory ValidateMnemonic.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ValidateMnemonic.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ValidateMnemonic clone() => ValidateMnemonic()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ValidateMnemonic copyWith(void Function(ValidateMnemonic) updates) =>
      super.copyWith((message) => updates(message as ValidateMnemonic))
          as ValidateMnemonic; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ValidateMnemonic create() => ValidateMnemonic._();
  ValidateMnemonic createEmptyInstance() => create();
  static $pb.PbList<ValidateMnemonic> createRepeated() =>
      $pb.PbList<ValidateMnemonic>();
  @$core.pragma('dart2js:noInline')
  static ValidateMnemonic getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ValidateMnemonic>(create);
  static ValidateMnemonic? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get phrase => $_getSZ(0);
  @$pb.TagNumber(1)
  set phrase($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPhrase() => $_has(0);
  @$pb.TagNumber(1)
  void clearPhrase() => clearField(1);
}

class Encrypt extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Encrypt',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'plaintext',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'secretKey',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  Encrypt._() : super();
  factory Encrypt({
    $core.List<$core.int>? plaintext,
    $core.List<$core.int>? secretKey,
  }) {
    final _result = create();
    if (plaintext != null) {
      _result.plaintext = plaintext;
    }
    if (secretKey != null) {
      _result.secretKey = secretKey;
    }
    return _result;
  }
  factory Encrypt.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Encrypt.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Encrypt clone() => Encrypt()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Encrypt copyWith(void Function(Encrypt) updates) =>
      super.copyWith((message) => updates(message as Encrypt))
          as Encrypt; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Encrypt create() => Encrypt._();
  Encrypt createEmptyInstance() => create();
  static $pb.PbList<Encrypt> createRepeated() => $pb.PbList<Encrypt>();
  @$core.pragma('dart2js:noInline')
  static Encrypt getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Encrypt>(create);
  static Encrypt? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get plaintext => $_getN(0);
  @$pb.TagNumber(1)
  set plaintext($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPlaintext() => $_has(0);
  @$pb.TagNumber(1)
  void clearPlaintext() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get secretKey => $_getN(1);
  @$pb.TagNumber(2)
  set secretKey($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSecretKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecretKey() => clearField(2);
}

class Decrypt extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Decrypt',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'ciphertext',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'secretKey',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  Decrypt._() : super();
  factory Decrypt({
    $core.List<$core.int>? ciphertext,
    $core.List<$core.int>? secretKey,
  }) {
    final _result = create();
    if (ciphertext != null) {
      _result.ciphertext = ciphertext;
    }
    if (secretKey != null) {
      _result.secretKey = secretKey;
    }
    return _result;
  }
  factory Decrypt.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Decrypt.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Decrypt clone() => Decrypt()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Decrypt copyWith(void Function(Decrypt) updates) =>
      super.copyWith((message) => updates(message as Decrypt))
          as Decrypt; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Decrypt create() => Decrypt._();
  Decrypt createEmptyInstance() => create();
  static $pb.PbList<Decrypt> createRepeated() => $pb.PbList<Decrypt>();
  @$core.pragma('dart2js:noInline')
  static Decrypt getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Decrypt>(create);
  static Decrypt? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get ciphertext => $_getN(0);
  @$pb.TagNumber(1)
  set ciphertext($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasCiphertext() => $_has(0);
  @$pb.TagNumber(1)
  void clearCiphertext() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get secretKey => $_getN(1);
  @$pb.TagNumber(2)
  set secretKey($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSecretKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecretKey() => clearField(2);
}

class Sign extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Sign',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'msg',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  Sign._() : super();
  factory Sign({
    $core.List<$core.int>? msg,
  }) {
    final _result = create();
    if (msg != null) {
      _result.msg = msg;
    }
    return _result;
  }
  factory Sign.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Sign.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Sign clone() => Sign()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Sign copyWith(void Function(Sign) updates) =>
      super.copyWith((message) => updates(message as Sign))
          as Sign; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Sign create() => Sign._();
  Sign createEmptyInstance() => create();
  static $pb.PbList<Sign> createRepeated() => $pb.PbList<Sign>();
  @$core.pragma('dart2js:noInline')
  static Sign getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Sign>(create);
  static Sign? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get msg => $_getN(0);
  @$pb.TagNumber(1)
  set msg($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasMsg() => $_has(0);
  @$pb.TagNumber(1)
  void clearMsg() => clearField(1);
}

class Verify extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Verify',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'publicKey',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'msg',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'sig',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  Verify._() : super();
  factory Verify({
    $core.List<$core.int>? publicKey,
    $core.List<$core.int>? msg,
    $core.List<$core.int>? sig,
  }) {
    final _result = create();
    if (publicKey != null) {
      _result.publicKey = publicKey;
    }
    if (msg != null) {
      _result.msg = msg;
    }
    if (sig != null) {
      _result.sig = sig;
    }
    return _result;
  }
  factory Verify.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Verify.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Verify clone() => Verify()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Verify copyWith(void Function(Verify) updates) =>
      super.copyWith((message) => updates(message as Verify))
          as Verify; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Verify create() => Verify._();
  Verify createEmptyInstance() => create();
  static $pb.PbList<Verify> createRepeated() => $pb.PbList<Verify>();
  @$core.pragma('dart2js:noInline')
  static Verify getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Verify>(create);
  static Verify? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get publicKey => $_getN(0);
  @$pb.TagNumber(1)
  set publicKey($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPublicKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPublicKey() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get msg => $_getN(1);
  @$pb.TagNumber(2)
  set msg($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasMsg() => $_has(1);
  @$pb.TagNumber(2)
  void clearMsg() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get sig => $_getN(2);
  @$pb.TagNumber(3)
  set sig($core.List<$core.int> v) {
    $_setBytes(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasSig() => $_has(2);
  @$pb.TagNumber(3)
  void clearSig() => clearField(3);
}

class DiffieHellmanKeyExchange extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DiffieHellmanKeyExchange',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'theirPublicKey',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  DiffieHellmanKeyExchange._() : super();
  factory DiffieHellmanKeyExchange({
    $core.List<$core.int>? theirPublicKey,
  }) {
    final _result = create();
    if (theirPublicKey != null) {
      _result.theirPublicKey = theirPublicKey;
    }
    return _result;
  }
  factory DiffieHellmanKeyExchange.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DiffieHellmanKeyExchange.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DiffieHellmanKeyExchange clone() =>
      DiffieHellmanKeyExchange()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DiffieHellmanKeyExchange copyWith(
          void Function(DiffieHellmanKeyExchange) updates) =>
      super.copyWith((message) => updates(message as DiffieHellmanKeyExchange))
          as DiffieHellmanKeyExchange; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DiffieHellmanKeyExchange create() => DiffieHellmanKeyExchange._();
  DiffieHellmanKeyExchange createEmptyInstance() => create();
  static $pb.PbList<DiffieHellmanKeyExchange> createRepeated() =>
      $pb.PbList<DiffieHellmanKeyExchange>();
  @$core.pragma('dart2js:noInline')
  static DiffieHellmanKeyExchange getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DiffieHellmanKeyExchange>(create);
  static DiffieHellmanKeyExchange? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get theirPublicKey => $_getN(0);
  @$pb.TagNumber(1)
  set theirPublicKey($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTheirPublicKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearTheirPublicKey() => clearField(1);
}

class HashSha256 extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'HashSha256',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'input',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  HashSha256._() : super();
  factory HashSha256({
    $core.List<$core.int>? input,
  }) {
    final _result = create();
    if (input != null) {
      _result.input = input;
    }
    return _result;
  }
  factory HashSha256.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory HashSha256.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  HashSha256 clone() => HashSha256()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  HashSha256 copyWith(void Function(HashSha256) updates) =>
      super.copyWith((message) => updates(message as HashSha256))
          as HashSha256; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static HashSha256 create() => HashSha256._();
  HashSha256 createEmptyInstance() => create();
  static $pb.PbList<HashSha256> createRepeated() => $pb.PbList<HashSha256>();
  @$core.pragma('dart2js:noInline')
  static HashSha256 getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<HashSha256>(create);
  static HashSha256? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get input => $_getN(0);
  @$pb.TagNumber(1)
  set input($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasInput() => $_has(0);
  @$pb.TagNumber(1)
  void clearInput() => clearField(1);
}

class HashFileSha256 extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'HashFileSha256',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'path')
    ..hasRequiredFields = false;

  HashFileSha256._() : super();
  factory HashFileSha256({
    $core.String? path,
  }) {
    final _result = create();
    if (path != null) {
      _result.path = path;
    }
    return _result;
  }
  factory HashFileSha256.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory HashFileSha256.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  HashFileSha256 clone() => HashFileSha256()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  HashFileSha256 copyWith(void Function(HashFileSha256) updates) =>
      super.copyWith((message) => updates(message as HashFileSha256))
          as HashFileSha256; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static HashFileSha256 create() => HashFileSha256._();
  HashFileSha256 createEmptyInstance() => create();
  static $pb.PbList<HashFileSha256> createRepeated() =>
      $pb.PbList<HashFileSha256>();
  @$core.pragma('dart2js:noInline')
  static HashFileSha256 getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<HashFileSha256>(create);
  static HashFileSha256? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);
}

enum Request_Body {
  currentKeyPair,
  generateKeyPair,
  initKeyPair,
  restoreKeyPair,
  backupKeyPair,
  validateMnemonic,
  encrypt,
  decrypt,
  sign,
  verify,
  diffieHellmanKeyExchange,
  hashSha256,
  hashFileSha256,
  notSet
}

class Request extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Request_Body> _Request_BodyByTag = {
    1: Request_Body.currentKeyPair,
    2: Request_Body.generateKeyPair,
    3: Request_Body.initKeyPair,
    4: Request_Body.restoreKeyPair,
    5: Request_Body.backupKeyPair,
    6: Request_Body.validateMnemonic,
    7: Request_Body.encrypt,
    8: Request_Body.decrypt,
    9: Request_Body.sign,
    10: Request_Body.verify,
    11: Request_Body.diffieHellmanKeyExchange,
    12: Request_Body.hashSha256,
    13: Request_Body.hashFileSha256,
    0: Request_Body.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Request',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])
    ..aOM<$0.Empty>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'currentKeyPair',
        subBuilder: $0.Empty.create)
    ..aOM<$0.Empty>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'generateKeyPair',
        subBuilder: $0.Empty.create)
    ..aOM<InitKeyPair>(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'initKeyPair',
        subBuilder: InitKeyPair.create)
    ..aOM<RestoreKeyPair>(
        4,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'restoreKeyPair',
        subBuilder: RestoreKeyPair.create)
    ..aOM<BackupKeyPair>(
        5,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'backupKeyPair',
        subBuilder: BackupKeyPair.create)
    ..aOM<ValidateMnemonic>(
        6,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'validateMnemonic',
        subBuilder: ValidateMnemonic.create)
    ..aOM<Encrypt>(
        7,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'encrypt',
        subBuilder: Encrypt.create)
    ..aOM<Decrypt>(
        8,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'decrypt',
        subBuilder: Decrypt.create)
    ..aOM<Sign>(
        9,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'sign',
        subBuilder: Sign.create)
    ..aOM<Verify>(
        10,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'verify',
        subBuilder: Verify.create)
    ..aOM<DiffieHellmanKeyExchange>(
        11,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'diffieHellmanKeyExchange',
        subBuilder: DiffieHellmanKeyExchange.create)
    ..aOM<HashSha256>(
        12,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'hashSha256',
        subBuilder: HashSha256.create)
    ..aOM<HashFileSha256>(
        13,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'hashFileSha256',
        subBuilder: HashFileSha256.create)
    ..hasRequiredFields = false;

  Request._() : super();
  factory Request({
    $0.Empty? currentKeyPair,
    $0.Empty? generateKeyPair,
    InitKeyPair? initKeyPair,
    RestoreKeyPair? restoreKeyPair,
    BackupKeyPair? backupKeyPair,
    ValidateMnemonic? validateMnemonic,
    Encrypt? encrypt,
    Decrypt? decrypt,
    Sign? sign,
    Verify? verify,
    DiffieHellmanKeyExchange? diffieHellmanKeyExchange,
    HashSha256? hashSha256,
    HashFileSha256? hashFileSha256,
  }) {
    final _result = create();
    if (currentKeyPair != null) {
      _result.currentKeyPair = currentKeyPair;
    }
    if (generateKeyPair != null) {
      _result.generateKeyPair = generateKeyPair;
    }
    if (initKeyPair != null) {
      _result.initKeyPair = initKeyPair;
    }
    if (restoreKeyPair != null) {
      _result.restoreKeyPair = restoreKeyPair;
    }
    if (backupKeyPair != null) {
      _result.backupKeyPair = backupKeyPair;
    }
    if (validateMnemonic != null) {
      _result.validateMnemonic = validateMnemonic;
    }
    if (encrypt != null) {
      _result.encrypt = encrypt;
    }
    if (decrypt != null) {
      _result.decrypt = decrypt;
    }
    if (sign != null) {
      _result.sign = sign;
    }
    if (verify != null) {
      _result.verify = verify;
    }
    if (diffieHellmanKeyExchange != null) {
      _result.diffieHellmanKeyExchange = diffieHellmanKeyExchange;
    }
    if (hashSha256 != null) {
      _result.hashSha256 = hashSha256;
    }
    if (hashFileSha256 != null) {
      _result.hashFileSha256 = hashFileSha256;
    }
    return _result;
  }
  factory Request.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Request.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Request clone() => Request()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Request copyWith(void Function(Request) updates) =>
      super.copyWith((message) => updates(message as Request))
          as Request; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Request create() => Request._();
  Request createEmptyInstance() => create();
  static $pb.PbList<Request> createRepeated() => $pb.PbList<Request>();
  @$core.pragma('dart2js:noInline')
  static Request getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Request>(create);
  static Request? _defaultInstance;

  Request_Body whichBody() => _Request_BodyByTag[$_whichOneof(0)]!;
  void clearBody() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $0.Empty get currentKeyPair => $_getN(0);
  @$pb.TagNumber(1)
  set currentKeyPair($0.Empty v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasCurrentKeyPair() => $_has(0);
  @$pb.TagNumber(1)
  void clearCurrentKeyPair() => clearField(1);
  @$pb.TagNumber(1)
  $0.Empty ensureCurrentKeyPair() => $_ensure(0);

  @$pb.TagNumber(2)
  $0.Empty get generateKeyPair => $_getN(1);
  @$pb.TagNumber(2)
  set generateKeyPair($0.Empty v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasGenerateKeyPair() => $_has(1);
  @$pb.TagNumber(2)
  void clearGenerateKeyPair() => clearField(2);
  @$pb.TagNumber(2)
  $0.Empty ensureGenerateKeyPair() => $_ensure(1);

  @$pb.TagNumber(3)
  InitKeyPair get initKeyPair => $_getN(2);
  @$pb.TagNumber(3)
  set initKeyPair(InitKeyPair v) {
    setField(3, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasInitKeyPair() => $_has(2);
  @$pb.TagNumber(3)
  void clearInitKeyPair() => clearField(3);
  @$pb.TagNumber(3)
  InitKeyPair ensureInitKeyPair() => $_ensure(2);

  @$pb.TagNumber(4)
  RestoreKeyPair get restoreKeyPair => $_getN(3);
  @$pb.TagNumber(4)
  set restoreKeyPair(RestoreKeyPair v) {
    setField(4, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasRestoreKeyPair() => $_has(3);
  @$pb.TagNumber(4)
  void clearRestoreKeyPair() => clearField(4);
  @$pb.TagNumber(4)
  RestoreKeyPair ensureRestoreKeyPair() => $_ensure(3);

  @$pb.TagNumber(5)
  BackupKeyPair get backupKeyPair => $_getN(4);
  @$pb.TagNumber(5)
  set backupKeyPair(BackupKeyPair v) {
    setField(5, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasBackupKeyPair() => $_has(4);
  @$pb.TagNumber(5)
  void clearBackupKeyPair() => clearField(5);
  @$pb.TagNumber(5)
  BackupKeyPair ensureBackupKeyPair() => $_ensure(4);

  @$pb.TagNumber(6)
  ValidateMnemonic get validateMnemonic => $_getN(5);
  @$pb.TagNumber(6)
  set validateMnemonic(ValidateMnemonic v) {
    setField(6, v);
  }

  @$pb.TagNumber(6)
  $core.bool hasValidateMnemonic() => $_has(5);
  @$pb.TagNumber(6)
  void clearValidateMnemonic() => clearField(6);
  @$pb.TagNumber(6)
  ValidateMnemonic ensureValidateMnemonic() => $_ensure(5);

  @$pb.TagNumber(7)
  Encrypt get encrypt => $_getN(6);
  @$pb.TagNumber(7)
  set encrypt(Encrypt v) {
    setField(7, v);
  }

  @$pb.TagNumber(7)
  $core.bool hasEncrypt() => $_has(6);
  @$pb.TagNumber(7)
  void clearEncrypt() => clearField(7);
  @$pb.TagNumber(7)
  Encrypt ensureEncrypt() => $_ensure(6);

  @$pb.TagNumber(8)
  Decrypt get decrypt => $_getN(7);
  @$pb.TagNumber(8)
  set decrypt(Decrypt v) {
    setField(8, v);
  }

  @$pb.TagNumber(8)
  $core.bool hasDecrypt() => $_has(7);
  @$pb.TagNumber(8)
  void clearDecrypt() => clearField(8);
  @$pb.TagNumber(8)
  Decrypt ensureDecrypt() => $_ensure(7);

  @$pb.TagNumber(9)
  Sign get sign => $_getN(8);
  @$pb.TagNumber(9)
  set sign(Sign v) {
    setField(9, v);
  }

  @$pb.TagNumber(9)
  $core.bool hasSign() => $_has(8);
  @$pb.TagNumber(9)
  void clearSign() => clearField(9);
  @$pb.TagNumber(9)
  Sign ensureSign() => $_ensure(8);

  @$pb.TagNumber(10)
  Verify get verify => $_getN(9);
  @$pb.TagNumber(10)
  set verify(Verify v) {
    setField(10, v);
  }

  @$pb.TagNumber(10)
  $core.bool hasVerify() => $_has(9);
  @$pb.TagNumber(10)
  void clearVerify() => clearField(10);
  @$pb.TagNumber(10)
  Verify ensureVerify() => $_ensure(9);

  @$pb.TagNumber(11)
  DiffieHellmanKeyExchange get diffieHellmanKeyExchange => $_getN(10);
  @$pb.TagNumber(11)
  set diffieHellmanKeyExchange(DiffieHellmanKeyExchange v) {
    setField(11, v);
  }

  @$pb.TagNumber(11)
  $core.bool hasDiffieHellmanKeyExchange() => $_has(10);
  @$pb.TagNumber(11)
  void clearDiffieHellmanKeyExchange() => clearField(11);
  @$pb.TagNumber(11)
  DiffieHellmanKeyExchange ensureDiffieHellmanKeyExchange() => $_ensure(10);

  @$pb.TagNumber(12)
  HashSha256 get hashSha256 => $_getN(11);
  @$pb.TagNumber(12)
  set hashSha256(HashSha256 v) {
    setField(12, v);
  }

  @$pb.TagNumber(12)
  $core.bool hasHashSha256() => $_has(11);
  @$pb.TagNumber(12)
  void clearHashSha256() => clearField(12);
  @$pb.TagNumber(12)
  HashSha256 ensureHashSha256() => $_ensure(11);

  @$pb.TagNumber(13)
  HashFileSha256 get hashFileSha256 => $_getN(12);
  @$pb.TagNumber(13)
  set hashFileSha256(HashFileSha256 v) {
    setField(13, v);
  }

  @$pb.TagNumber(13)
  $core.bool hasHashFileSha256() => $_has(12);
  @$pb.TagNumber(13)
  void clearHashFileSha256() => clearField(13);
  @$pb.TagNumber(13)
  HashFileSha256 ensureHashFileSha256() => $_ensure(12);
}

class Response_Error extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Response.Error',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'message')
    ..e<Response_Error_Kind>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'kind',
        $pb.PbFieldType.OE,
        defaultOrMaker: Response_Error_Kind.UNKNOWN,
        valueOf: Response_Error_Kind.valueOf,
        enumValues: Response_Error_Kind.values)
    ..hasRequiredFields = false;

  Response_Error._() : super();
  factory Response_Error({
    $core.String? message,
    Response_Error_Kind? kind,
  }) {
    final _result = create();
    if (message != null) {
      _result.message = message;
    }
    if (kind != null) {
      _result.kind = kind;
    }
    return _result;
  }
  factory Response_Error.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Response_Error.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Response_Error clone() => Response_Error()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Response_Error copyWith(void Function(Response_Error) updates) =>
      super.copyWith((message) => updates(message as Response_Error))
          as Response_Error; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Response_Error create() => Response_Error._();
  Response_Error createEmptyInstance() => create();
  static $pb.PbList<Response_Error> createRepeated() =>
      $pb.PbList<Response_Error>();
  @$core.pragma('dart2js:noInline')
  static Response_Error getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Response_Error>(create);
  static Response_Error? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(1)
  set message($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => clearField(1);

  @$pb.TagNumber(2)
  Response_Error_Kind get kind => $_getN(1);
  @$pb.TagNumber(2)
  set kind(Response_Error_Kind v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasKind() => $_has(1);
  @$pb.TagNumber(2)
  void clearKind() => clearField(2);
}

enum Response_Body {
  error,
  keyPair,
  mnemonic,
  validMnemonic,
  encryptedMessage,
  decryptedMessage,
  signature,
  validSignature,
  sharedSecret,
  sha256Hash,
  notSet
}

class Response extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, Response_Body> _Response_BodyByTag = {
    1: Response_Body.error,
    2: Response_Body.keyPair,
    3: Response_Body.mnemonic,
    4: Response_Body.validMnemonic,
    5: Response_Body.encryptedMessage,
    6: Response_Body.decryptedMessage,
    7: Response_Body.signature,
    8: Response_Body.validSignature,
    9: Response_Body.sharedSecret,
    10: Response_Body.sha256Hash,
    0: Response_Body.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'Response',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'owlchat'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    ..aOM<Response_Error>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'error',
        subBuilder: Response_Error.create)
    ..aOM<KeyPair>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'keyPair',
        subBuilder: KeyPair.create)
    ..aOS(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'mnemonic')
    ..aOB(
        4,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'validMnemonic')
    ..a<$core.List<$core.int>>(
        5,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'encryptedMessage',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        6,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'decryptedMessage',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        7,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'signature',
        $pb.PbFieldType.OY)
    ..aOB(
        8,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'validSignature')
    ..a<$core.List<$core.int>>(
        9,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'sharedSecret',
        $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        10,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'sha256Hash',
        $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  Response._() : super();
  factory Response({
    Response_Error? error,
    KeyPair? keyPair,
    $core.String? mnemonic,
    $core.bool? validMnemonic,
    $core.List<$core.int>? encryptedMessage,
    $core.List<$core.int>? decryptedMessage,
    $core.List<$core.int>? signature,
    $core.bool? validSignature,
    $core.List<$core.int>? sharedSecret,
    $core.List<$core.int>? sha256Hash,
  }) {
    final _result = create();
    if (error != null) {
      _result.error = error;
    }
    if (keyPair != null) {
      _result.keyPair = keyPair;
    }
    if (mnemonic != null) {
      _result.mnemonic = mnemonic;
    }
    if (validMnemonic != null) {
      _result.validMnemonic = validMnemonic;
    }
    if (encryptedMessage != null) {
      _result.encryptedMessage = encryptedMessage;
    }
    if (decryptedMessage != null) {
      _result.decryptedMessage = decryptedMessage;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    if (validSignature != null) {
      _result.validSignature = validSignature;
    }
    if (sharedSecret != null) {
      _result.sharedSecret = sharedSecret;
    }
    if (sha256Hash != null) {
      _result.sha256Hash = sha256Hash;
    }
    return _result;
  }
  factory Response.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Response.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Response clone() => Response()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Response copyWith(void Function(Response) updates) =>
      super.copyWith((message) => updates(message as Response))
          as Response; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Response create() => Response._();
  Response createEmptyInstance() => create();
  static $pb.PbList<Response> createRepeated() => $pb.PbList<Response>();
  @$core.pragma('dart2js:noInline')
  static Response getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Response>(create);
  static Response? _defaultInstance;

  Response_Body whichBody() => _Response_BodyByTag[$_whichOneof(0)]!;
  void clearBody() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Response_Error get error => $_getN(0);
  @$pb.TagNumber(1)
  set error(Response_Error v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasError() => $_has(0);
  @$pb.TagNumber(1)
  void clearError() => clearField(1);
  @$pb.TagNumber(1)
  Response_Error ensureError() => $_ensure(0);

  @$pb.TagNumber(2)
  KeyPair get keyPair => $_getN(1);
  @$pb.TagNumber(2)
  set keyPair(KeyPair v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasKeyPair() => $_has(1);
  @$pb.TagNumber(2)
  void clearKeyPair() => clearField(2);
  @$pb.TagNumber(2)
  KeyPair ensureKeyPair() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get mnemonic => $_getSZ(2);
  @$pb.TagNumber(3)
  set mnemonic($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasMnemonic() => $_has(2);
  @$pb.TagNumber(3)
  void clearMnemonic() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get validMnemonic => $_getBF(3);
  @$pb.TagNumber(4)
  set validMnemonic($core.bool v) {
    $_setBool(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasValidMnemonic() => $_has(3);
  @$pb.TagNumber(4)
  void clearValidMnemonic() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get encryptedMessage => $_getN(4);
  @$pb.TagNumber(5)
  set encryptedMessage($core.List<$core.int> v) {
    $_setBytes(4, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasEncryptedMessage() => $_has(4);
  @$pb.TagNumber(5)
  void clearEncryptedMessage() => clearField(5);

  @$pb.TagNumber(6)
  $core.List<$core.int> get decryptedMessage => $_getN(5);
  @$pb.TagNumber(6)
  set decryptedMessage($core.List<$core.int> v) {
    $_setBytes(5, v);
  }

  @$pb.TagNumber(6)
  $core.bool hasDecryptedMessage() => $_has(5);
  @$pb.TagNumber(6)
  void clearDecryptedMessage() => clearField(6);

  @$pb.TagNumber(7)
  $core.List<$core.int> get signature => $_getN(6);
  @$pb.TagNumber(7)
  set signature($core.List<$core.int> v) {
    $_setBytes(6, v);
  }

  @$pb.TagNumber(7)
  $core.bool hasSignature() => $_has(6);
  @$pb.TagNumber(7)
  void clearSignature() => clearField(7);

  @$pb.TagNumber(8)
  $core.bool get validSignature => $_getBF(7);
  @$pb.TagNumber(8)
  set validSignature($core.bool v) {
    $_setBool(7, v);
  }

  @$pb.TagNumber(8)
  $core.bool hasValidSignature() => $_has(7);
  @$pb.TagNumber(8)
  void clearValidSignature() => clearField(8);

  @$pb.TagNumber(9)
  $core.List<$core.int> get sharedSecret => $_getN(8);
  @$pb.TagNumber(9)
  set sharedSecret($core.List<$core.int> v) {
    $_setBytes(8, v);
  }

  @$pb.TagNumber(9)
  $core.bool hasSharedSecret() => $_has(8);
  @$pb.TagNumber(9)
  void clearSharedSecret() => clearField(9);

  @$pb.TagNumber(10)
  $core.List<$core.int> get sha256Hash => $_getN(9);
  @$pb.TagNumber(10)
  set sha256Hash($core.List<$core.int> v) {
    $_setBytes(9, v);
  }

  @$pb.TagNumber(10)
  $core.bool hasSha256Hash() => $_has(9);
  @$pb.TagNumber(10)
  void clearSha256Hash() => clearField(10);
}
