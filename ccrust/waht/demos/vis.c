#include <stdio.h>
#include <time.h>
#define GYM_IMPLEMENTATION
#include "gym.h"

void init_train_set(size_t bits, Mat m[2]) {
    size_t n = 1 << bits;
    size_t rows = n * n;

    m[0] = mat_alloc(rows, 2 * bits);
    m[1] = mat_alloc(rows, bits + 1);

    for (size_t i = 0; i < rows; i++) {
        size_t x = i / n;
        size_t y = i % n;
        size_t z = x + y;
        size_t overflow = z >= n;

        for (size_t j = 0; j < bits; j++) {
            M_AT(m[0], i, j) = (x >> j) & 1;
            M_AT(m[0], i, j + bits) = (y >> j) & 1;
            M_AT(m[1], i, j) = (z >> j) & 1;
        }

        M_AT(m[1], i, bits) = overflow;
    }

    M_AT(m[1], rows - 1, 0) = 1.0;
}

int main(const int argc, const char** argv) {
    size_t bits = 4;
    find_option(argc, argv, "--bits", &bits, parse_integer);

    // NN
    Mat m[2];
    init_train_set(bits, m);

    Mat train_input = m[0];
    Mat train_output = m[1];

    size_t layout[] = { 2 * bits, 4 * bits, bits + 1 };

    GymConfig config;
    read_config(argc, argv, &config);
    config.input = layout_input(layout, ARR_LEN(layout), train_input, train_output);

    gym(config);

    return 0;
}

