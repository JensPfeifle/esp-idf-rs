#include <stdio.h>

#include "CApi.h"

bool hello_c(int param, int value) {
  printf("Hello from C: param #%d = %d\n", param, value);
  return true;
}
