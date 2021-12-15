#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../binding.h"
#include "def.pb.h"

#pragma clang diagnostic ignored "-Wformat"
#pragma clang diagnostic ignored \
    "-Wincompatible-pointer-types-discards-qualifiers"
// NOLINTNEXTLINE
void println(const char *msg) { printf("%s\n", msg); }
