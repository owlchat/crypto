#include "../binding.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#pragma clang diagnostic ignored "-Wformat"
#pragma clang diagnostic ignored                                               \
    "-Wincompatible-pointer-types-discards-qualifiers"
// NOLINTNEXTLINE
void println(const char *msg) { printf("%s\n", msg); }

// NOLINTNEXTLINE
void print_fixed32_array(const Fixed32Array *arr) {
  for (size_t i = 0; i < 32; i++) {
    uint8_t e = arr->buf[i];
    printf("%x", e);
  }
  printf("\n");
}

// NOLINTNEXTLINE
void print_shared_buffer(const SharedBuffer *vec) {
  for (size_t i = 0; i < vec->len; i++) {
    uint8_t e = vec->buf[i];
    printf("%s", &e);
  }
  printf("\n");
}

// NOLINTNEXTLINE
void hex_dump_shared_buffer(const SharedBuffer *vec) {
  printf("0x");
  for (size_t i = 0; i < vec->len; i++) {
    uint8_t e = vec->buf[i];
    printf("%x", e);
  }
  printf("\n");
}

// NOLINTNEXTLINE
void assert_shared_buffer_eq(const SharedBuffer *a, const SharedBuffer *b) {
  if (a->len != b->len) {
    printf("assert failed len(a, %llu) != len(b, %llu)\n", a->len, b->len);
    exit(1);
  }
  for (uintptr_t i = 0; i < a->len; i++) {
    if (a->buf[i] != b->buf[i]) {
      printf("assert failed a[%llu] != b[%llu]\n", i, i);
      exit(1);
    }
  }
}

// NOLINTNEXTLINE
void assert_fixed32_array_eq(const Fixed32Array *a, const Fixed32Array *b) {
  for (uintptr_t i = 0; i < 32; i++) {
    if (a->buf[i] != b->buf[i]) {
      printf("assert failed a[%llu] != b[%llu]\n", i, i);
      exit(1);
    }
  }
}

// NOLINTNEXTLINE
void assert_not_null(void *ptr) {
  if (ptr == NULL) {
    println("assert failed ptr is null");
    exit(1);
  }
}

// NOLINTNEXTLINE
void assert_ok(OperationStatus status, const char *op_name) {
  if (status != Ok) {
    printf("assert failed status is not OK (got %d) for %s\n", status, op_name);
    exit(1);
  }
}

// NOLINTNEXTLINE
SharedBuffer *shared_buffer_new(size_t cap) {
  SharedBuffer *shared_buffer = (SharedBuffer *)malloc(sizeof(SharedBuffer));
  uint8_t *data = (uint8_t *)malloc(cap);
  memset(data, 0, cap);
  shared_buffer->buf = data;
  shared_buffer->len = 0;
  shared_buffer->cap = cap;
  return shared_buffer;
}

// dose not reallocate!
// NOLINTNEXTLINE
void shared_buffer_append(SharedBuffer *shared_buffer, const uint8_t *data,
                          size_t data_len) {
  memcpy(shared_buffer->buf, data, data_len);
  shared_buffer->len += data_len;
}

// NOLINTNEXTLINE
void shared_buffer_free(SharedBuffer *shared_buffer) {
  free(shared_buffer->buf);
  free(shared_buffer);
}

// NOLINTNEXTLINE
Fixed32Array *fixed32_array_empty() {
  Fixed32Array *arr = (Fixed32Array *)malloc(sizeof(Fixed32Array));
  arr->buf = (uint8_t *)malloc(32);
  memset(arr->buf, 0, 32);
  return arr;
}

// NOLINTNEXTLINE
Fixed32Array *fixed32_array_new(const uint8_t data[32]) {
  Fixed32Array *arr = (Fixed32Array *)malloc(sizeof(Fixed32Array));
  arr->buf = (uint8_t *)malloc(32 * sizeof(uint8_t));
  memcpy(arr->buf, data, 32);
  return arr;
}

// NOLINTNEXTLINE
void fixed32_array_free(Fixed32Array *arr) {
  free(arr->buf);
  free(arr);
}

// NOLINTNEXTLINE
Fixed64Array *fixed64_array_empty() {
  Fixed64Array *arr = (Fixed64Array *)malloc(sizeof(Fixed64Array));
  arr->buf = (uint8_t *)malloc(64);
  memset(arr->buf, 0, 64);
  return arr;
}

// NOLINTNEXTLINE
Fixed64Array *fixed64_array_new(const uint8_t data[64]) {
  Fixed64Array *arr = (Fixed64Array *)malloc(sizeof(Fixed64Array));
  arr->buf = (uint8_t *)malloc(64 * sizeof(uint8_t));
  memcpy(arr->buf, data, 64);
  return arr;
}

// NOLINTNEXTLINE
void fixed64_array_free(Fixed64Array *arr) {
  free(arr->buf);
  free(arr);
}
