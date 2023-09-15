#define GYM_IMPLEMENTATION
#include "gym.h"

int main(const int argc, const char** argv) {
    GymRender render_config = read_render_config(argc, argv);
    GymTrain train_config = read_train_config(argc, argv);

    size_t bits = 4;
    find_option(argc, argv, "--bits", &bits, parse_integer);

    size_t n = 1 << bits;
    size_t rows = n * n;
    Mat train_input = mat_alloc(rows, 2 * bits);
    Mat train_output = mat_alloc(rows, bits + 1);

    for (size_t i = 0; i < rows; i++) {
        size_t x = i / n;
        size_t y = i % n;
        size_t z = x + y;
        size_t overflow = z >= n;
        for (size_t j = 0; j < bits; j++) {
            M_AT(train_input, i, j) = (x >> j) & 1;
            M_AT(train_input, i, j + bits) = (y >> j) & 1;
            M_AT(train_output, i, j) = (z >> j) & 1;
        }
        M_AT(train_output, i, bits) = overflow;
    }

    size_t layout[] = { 2 * bits, 4 * bits, bits + 1 };
    Nero add = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);

    gym(render_config, train_config, (GymInput){
        .net = add,
        .grad = grad,
        .t_in = train_input,
        .t_out = train_output
    });

    // Verify output.
    for (size_t x = 0; x < n; x++) {
        for (size_t y = 0; y < n; y++) {
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
            printf("%lu + %lu = %lu\n", x, y, z);
        }
    }
}
