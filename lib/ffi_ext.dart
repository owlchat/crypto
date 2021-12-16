import 'dart:ffi';
import 'dart:typed_data';

import 'package:ffi/ffi.dart' as ffi;

extension Uint8ListAsPtr on Uint8List {
  Pointer<Uint8> asPtr() {
    return ffi.malloc.allocate<Uint8>(length)
      ..asTypedList(length).setAll(0, this);
  }
}

extension FreeUint8ListAsPtr on Pointer<Uint8> {
  void free() {
    ffi.malloc.free(this);
  }
}
