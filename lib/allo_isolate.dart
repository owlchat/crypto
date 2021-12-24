import 'dart:ffi';
import 'ffi.dart' as ffi;

/// `allo-isolate` Rust crate bindings.
class AlloIsolate {
  final ffi.RawOwlchatCrypto crypto;

  AlloIsolate(DynamicLibrary lib) : crypto = ffi.RawOwlchatCrypto(lib);

  void hook() {
    crypto.store_dart_post_cobject(NativeApi.postCObject.cast());
  }
}
