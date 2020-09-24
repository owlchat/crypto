#include "../binding.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void println(const char *msg) { printf("%s\n", msg); }

void print_fixed32_array(const Fixed32Array arr) {
  for (size_t i = 0; i < 32; i++) {
    uint8_t e = arr.data[i];
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
    println("assert faild len(a) != len(b)");
    exit(1);
  }
  for (uintptr_t i = 0; i < a->len; i++) {
    if (a->buf[i] != b->buf[i]) {
      printf("a[%llu] != b[%llu]\n", i, i);
      exit(1);
    }
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