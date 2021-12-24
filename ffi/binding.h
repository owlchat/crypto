#include <stdint.h>
#include <stdbool.h>

// A Dart_CObject is used for representing Dart objects as native C
// data outside the Dart heap. These objects are totally detached from
// the Dart heap. Only a subset of the Dart objects have a
// representation as a Dart_CObject.
//
// The string encoding in the 'value.as_string' is UTF-8.
//
// All the different types from dart:typed_data are exposed as type
// kTypedData. The specific type from dart:typed_data is in the type
// field of the as_typed_data structure. The length in the
// as_typed_data structure is always in bytes.
//
// The data for kTypedData is copied on message send and ownership remains with
// the caller. The ownership of data for kExternalTyped is passed to the VM on
// message send and returned when the VM invokes the
// Dart_WeakPersistentHandleFinalizer callback; a non-NULL callback must be
// provided.
//
// https://github.com/dart-lang/sdk/blob/main/runtime/include/dart_native_api.h
enum DartCObjectType {
  DartCObjectType_DartNull = 0,
  DartCObjectType_DartBool = 1,
  DartCObjectType_DartInt32 = 2,
  DartCObjectType_DartInt64 = 3,
  DartCObjectType_DartDouble = 4,
  DartCObjectType_DartString = 5,
  DartCObjectType_DartArray = 6,
  DartCObjectType_DartTypedData = 7,
  DartCObjectType_DartExternalTypedData = 8,
  DartCObjectType_DartSendPort = 9,
  DartCObjectType_DartCapability = 10,
  DartCObjectType_DartUnsupported = 11,
  DartCObjectType_DartNumberOfTypes = 12,
};
typedef int32_t DartCObjectType;

enum DartTypedDataType {
  DartTypedDataType_ByteData = 0,
  DartTypedDataType_Int8 = 1,
  DartTypedDataType_Uint8 = 2,
  DartTypedDataType_Uint8Clamped = 3,
  DartTypedDataType_Int16 = 4,
  DartTypedDataType_Uint16 = 5,
  DartTypedDataType_Int32 = 6,
  DartTypedDataType_Uint32 = 7,
  DartTypedDataType_Int64 = 8,
  DartTypedDataType_Uint64 = 9,
  DartTypedDataType_Float32 = 10,
  DartTypedDataType_Float64 = 11,
  DartTypedDataType_Float32x4 = 12,
  DartTypedDataType_Invalid = 13,
};
typedef int32_t DartTypedDataType;

typedef enum OwlchatResult {
  OwlchatResult_Ok = 1,
  OwlchatResult_NullPointerDetected = 2,
  OwlchatResult_InvalidProtobuf = 3,
} OwlchatResult;

// A Opaque pointer to a [`cryoto::KeyPair`]
typedef const void *RawKeyPair;

#if !defined(DART)
typedef struct Buffer {
  uint8_t *data;
  uintptr_t len;
} Buffer;
#endif

// A port is used to send or receive inter-isolate messages
typedef int64_t DartPort;

typedef struct DartNativeSendPort {
  DartPort id;
  DartPort origin_id;
} DartNativeSendPort;

typedef struct DartNativeCapability {
  int64_t id;
} DartNativeCapability;

typedef struct DartNativeArray {
  intptr_t length;
  struct DartCObject **values;
} DartNativeArray;

typedef struct DartNativeTypedData {
  DartTypedDataType ty;
  intptr_t length;
  uint8_t *values;
} DartNativeTypedData;

// https://github.com/dart-lang/sdk/blob/main/runtime/include/dart_api.h
typedef void (*DartHandleFinalizer)(void *isolate_callback_data, void *peer);

typedef struct DartNativeExternalTypedData {
  DartTypedDataType ty;
  intptr_t length;
  uint8_t *data;
  void *peer;
  DartHandleFinalizer callback;
} DartNativeExternalTypedData;

typedef union DartCObjectValue {
  bool as_bool;
  int32_t as_int32;
  int64_t as_int64;
  double as_double;
  char *as_string;
  struct DartNativeSendPort as_send_port;
  struct DartNativeCapability as_capability;
  struct DartNativeArray as_array;
  struct DartNativeTypedData as_typed_data;
  struct DartNativeExternalTypedData as_external_typed_data;
  uint64_t _bindgen_union_align[5];
} DartCObjectValue;

typedef struct DartCObject {
  DartCObjectType ty;
  union DartCObjectValue value;
} DartCObject;

//  Posts a message on some port. The message will contain the
//  Dart_CObject object graph rooted in 'message'.
//
//  While the message is being sent the state of the graph of
//  Dart_CObject structures rooted in 'message' should not be accessed,
//  as the message generation will make temporary modifications to the
//  data. When the message has been sent the graph will be fully
//  restored.
//
//  `port_id` The destination port.
//  `message` The message to send.
//
//  return true if the message was posted.
typedef bool (*DartPostCObjectFnType)(DartPort port_id, struct DartCObject *message);

// Creates a new [crypto::KeyPair] and return an opaque pointer to it.
// # Safety
//
// You must call [owlchat_crypto_keypair_drop] once you are done with it.
RawKeyPair owlchat_crypto_keypair_new(void);

// Drops a [crypto::KeyPair]
//
// # Safety
// Make sure that the pointer is valid.
void owlchat_crypto_keypair_drop(RawKeyPair pair);

#if defined(DART)
// This a Dart FFI interface to be called inside an Isolate.
//
// Passing a Isolate Port, along with some Protobuf payload, to this function will
// process the payload and return the result to the isolate over the port.
//
// # Errors
//
// This function will return an error if the provided payload is not valid.
//
// # Safety
//
// This function is unsafe because it deals with raw pointers.
enum OwlchatResult owlchat_crypto_dispatch(RawKeyPair keypair, int64_t port, const uint8_t *data, uintptr_t len);
#endif

#if !defined(DART)
// Main function to handle the request.
//
// # Errors
//
// This function will return null if the provided payload is not valid.
//
// # Safety
// you should free the returned buffer using [owlchat_crypto_free_buffer] after you are done with it.
struct Buffer owlchat_crypto_dispatch(RawKeyPair keypair, const uint8_t *data, uintptr_t len);
#endif

#if !defined(DART)
// Free the buffer returned by [owlchat_crypto_dispatch].
//
// # Examples
//
// ```
// use owlchat_crypto::owlchat_crypto_free_buffer;
//
// unsafe { owlchat_crypto_free_buffer(buffer) };
// ```
//
// # Safety
//
// This function is unsafe because it deals with raw pointers.
void owlchat_crypto_free_buffer(struct Buffer buffer);
#endif

// Stores the function pointer of `Dart_PostCObject`, this only should be
// called once at the start up of the Dart/Flutter Application. it is exported
// and marked as `#[no_mangle]`.
//
// you could use it from Dart as following:
//
// #### Safety
// This function should only be called once at the start up of the Dart
// application.
//
// ### Example
// ```dart,ignore
// import 'dart:ffi';
//
// typedef dartPostCObject = Pointer Function(
//         Pointer<NativeFunction<Int8 Function(Int64,
// Pointer<Dart_CObject>)>>);
//
// // assumes that _dl is the `DynamicLibrary`
// final storeDartPostCObject =
//     _dl.lookupFunction<dartPostCObject, dartPostCObject>(
// 'store_dart_post_cobject',
// );
//
// // then later call
//
// storeDartPostCObject(NativeApi.postCObject);
// ```
void store_dart_post_cobject(DartPostCObjectFnType ptr);
