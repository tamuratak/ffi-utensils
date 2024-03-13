#include <stdbool.h>
#include <stdint.h>
#include <Foundation/Foundation.h>

bool check_addition(int32_t a, int32_t b, int32_t (^add)(int32_t, int32_t)) {
    return add(a, b) == a + b;
}
