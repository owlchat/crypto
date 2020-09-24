#include "../binding.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#pragma clang diagnostic ignored "-Wformat"
#pragma clang diagnostic ignored                                               \
    "-Wincompatible-pointer-types-discards-qualifiers"

void println(const char *msg) { printf("%s\n", msg); }

void print_fixed32_array(const Fixed32Array *arr) {
  for (size_t i = 0; i < 32; i++) {
    uint8_t e = arr->buf[i];
    printf("%x", e);
  }
  printf("\n");
}

void print_shared_buffer(const SharedBuffer *vec) {
  for (size_t i = 0; i < vec->len; i++) {
    uint8_t e = vec->buf[i];
    printf("%s", &e);
  }
  printf("\n");
}

void hex_dump_shared_buffer(const SharedBuffer *vec) {
  printf("0x");
  for (size_t i = 0; i < vec->len; i++) {
    uint8_t e = vec->buf[i];
    printf("%x", e);
  }
  printf("\n");
}

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

void assert_fixed32_array_eq(const Fixed32Array *a, const Fixed32Array *b) {
  for (uintptr_t i = 0; i < 32; i++) {
    if (a->buf[i] != b->buf[i]) {
      printf("assert failed a[%llu] != b[%llu]\n", i, i);
      exit(1);
    }
  }
}
void assert_not_null(void *ptr) {
  if (ptr == NULL) {
    println("assert failed ptr is null");
    exit(1);
  }
}

void assert_ok(OperationStatus status, const char *op_name) {
  if (status != OK) {
    printf("assert failed status is not OK (got %d) for %s\n", status, op_name);
    exit(1);
  }
}

SharedBuffer *shared_buffer_new(size_t cap) {
  SharedBuffer *shared_buffer = malloc(sizeof(SharedBuffer));
  uint8_t *data = malloc(cap);
  memset(data, 0, cap);
  shared_buffer->buf = data;
  shared_buffer->len = 0;
  shared_buffer->cap = cap;
  return shared_buffer;
}

// dose not reallocate!
void shared_buffer_append(SharedBuffer *shared_buffer, const uint8_t *data,
                          size_t data_len) {
  memcpy(shared_buffer->buf, data, data_len);
  shared_buffer->len += data_len;
}

void shared_buffer_free(SharedBuffer *shared_buffer) {
  free(shared_buffer->buf);
  free(shared_buffer);
}

Fixed32Array *fixed32_array_empty() {
  Fixed32Array *arr = malloc(sizeof(Fixed32Array));
  arr->buf = malloc(32);
  memset(arr->buf, 0, 32);
  return arr;
}

Fixed32Array *fixed32_array_new(const uint8_t data[32]) {
  Fixed32Array *arr = malloc(sizeof(Fixed32Array));
  arr->buf = malloc(32 * sizeof(uint8_t));
  memcpy(arr->buf, data, 32);
  return arr;
}

void fixed32_array_free(Fixed32Array *arr) {
  free(arr->buf);
  free(arr);
}
