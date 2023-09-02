#define NERO_IMPLEMENTATION
#include "nero.h"
#include <time.h>

#ifndef BITS
#define BITS 4
#endif

int main(void) {
#ifdef NORND
    srand(56);
#else
    srand(time(NULL));
#endif
#ifdef EPOCHS
    size_t epochs = EPOCHS;
#else
    size_t epochs = 100;
#endif

    float rate = 1e-1;
    size_t n = 1 << BITS;
    size_t rows = n * n;
    Mat train_input = mat_alloc(rows, 2 * BITS);
    Mat train_output = mat_alloc(rows, BITS + 1);

    for (size_t i = 0; i < rows; i++) {
        size_t x = i / n;
        size_t y = i % n;
        size_t z = x + y;
        size_t overflow = z >= n;
        for (size_t j = 0; j < BITS; j++) {
            M_AT(train_input, i, j) = (x >> j) & 1;
            M_AT(train_input, i, j + BITS) = (y >> j) & 1;
            M_AT(train_output, i, j) = (z >> j) & 1;
        }
        M_AT(train_output, i, BITS) = overflow;
    }

    size_t layout[] = { 2 * BITS, 4 * BITS, BITS + 1 };
    Nero add = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);
    nero_rand(add, 0, 1);

    NERO_PRINT(add);

    for (size_t e = 0; e < epochs; e++) {
#ifdef FIN
        nero_finite_diff(add, grad, 1e-1, train_input, train_output);
#else
        nero_backprop(add, grad, train_input, train_output);
#endif
        nero_learn(add, grad, rate);

        if (e % 50 == 0) {
            printf("[%lu/%lu] Cost: %f\n", e, epochs, nero_cost(add, train_input, train_output));
        }
    }

    for (size_t x = 0; x < n; x++) {
        for (size_t y = 0; y < n; y++) {
            for (size_t b = 0; b < BITS; b++) {
                M_AT(NERO_INPUT(add), 0, b) = (x >> b) & 1;
                M_AT(NERO_INPUT(add), 0, b + BITS) = (y >> b) & 1;
            }

            nero_forward(add);
            size_t z = 0;
            for (size_t i = 0; i < BITS; i++) {
                size_t bit = M_AT(NERO_OUTPUT(add), 0, i) >= 0.5f;
                z |= bit << i;
            }
            printf("%lu + %lu = %lu\n", x, y, z);
        }
    }
}
