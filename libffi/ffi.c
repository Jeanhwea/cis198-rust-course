#include <stdint.h>

int32_t foo() {
    return 42;
}

int32_t fib(int32_t n) {
    if (n <= 1) {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
