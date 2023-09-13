#include <stdio.h>
#include <time.h>
#include "raylib.h"
#define NERO_IMPLEMENTATION
#include "nero.h"
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

void verify_train_set(size_t bits, Nero add) {
    size_t n = 1 << bits;

    for (size_t x = 0; x < n; x++) {
        printf("%2lu +\n", x);
        for (size_t d = 0; d < bits; d++) {
            size_t y = d;
            while (y < n) {
                for (size_t b = 0; b < bits; b++) {
                    M_AT(NERO_INPUT(add), 0, b) = (x >> b) & 1;
                    M_AT(NERO_INPUT(add), 0, b + bits) = (y >> b) & 1;
                }

                nero_forward(add);
                size_t z = 0;
                for (size_t i = 0; i < bits; i++) {
                    size_t bit = M_AT(NERO_OUTPUT(add), 0, i) >= 0.5f;
                    z |= bit << i;
                }
                printf("   + %2lu = %2lu\t", y, z);
                y += bits;
            }
            printf("\n");
        }
        printf("\n");
    }
}

void seed_rand() {
#ifdef SEED
    srand(SEED);
#else
    srand(time(NULL));
#endif
}

int main(const int argc, const char** argv) {
    // Seed
    seed_rand();

    size_t bits = 4;
    find_option(argc, argv, "--bits", &bits, parse_integer);

    // NN
    Mat m[2];
    init_train_set(bits, m);

    Mat train_input = m[0];
    Mat train_output = m[1];

    size_t layout[] = { 2 * bits, 4 * bits, bits + 1 };
    Nero add = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);

    GymRender render_config = read_render_config(argc, argv);
    GymTrain train_config = read_train_config(argc, argv);

    print_render_config(&render_config);
    print_train_config(&train_config);

    gym(render_config, train_config, (GymInput){
        .net = add,
        .grad = grad,
        .t_in = train_input,
        .t_out = train_output
    });

#ifdef DEBUG
    verify_train_set(bits, add);
#endif

    return 0;
}

