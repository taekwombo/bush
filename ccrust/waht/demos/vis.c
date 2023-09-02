#define NERO_IMPLEMENTATION
#include <stdio.h>
#include <time.h>
#include "raylib.h"
#include "nero.h"

#ifndef BITS
#define BITS 4
#endif

#ifndef EPOCHS
#define EPOCHS 1000
#endif

#ifndef SCR_W
#define SCR_W 720
#endif

#ifndef SCR_H
#define SCR_H 680
#endif

#ifndef RAD
#define RAD 10
#endif

#ifndef PLOT_ENTRIES
#define PLOT_ENTRIES 1000
#endif

void init_train_set(Mat m[2]) {
    size_t n = 1 << BITS;
    size_t rows = n * n;

    m[0] = mat_alloc(rows, 2 * BITS);
    m[1] = mat_alloc(rows, BITS + 1);

    for (size_t i = 0; i < rows; i++) {
        size_t x = i / n;
        size_t y = i % n;
        size_t z = x + y;
        size_t overflow = z >= n;

        for (size_t j = 0; j < BITS; j++) {
            M_AT(m[0], i, j) = (x >> j) & 1;
            M_AT(m[0], i, j + BITS) = (y >> j) & 1;
            M_AT(m[1], i, j) = (z >> j) & 1;
        }

        M_AT(m[1], i, BITS) = overflow;
    }

    M_AT(m[1], rows - 1, 0) = 1.0;
}

void verify_train_set(Nero add) {
    size_t n = 1 << BITS;

    for (size_t x = 0; x < n; x++) {
        printf("%2lu +\n", x);
        for (size_t d = 0; d < BITS; d++) {
            size_t y = d;
            while (y < n) {
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
                printf("   + %2lu = %2lu\t", y, z);
                y += BITS;
            }
            printf("\n");
        }
        printf("\n");
    }
}

void seed_rand() {
#ifdef RNDS
    srand(RNDS);
#else
    srand(time(NULL));
#endif
}

/**
 * Sort of like ring buffer (goes in circles), but this one has is single ended and push only.
 */
typedef struct {
    size_t capacity;
    size_t idx;
    size_t size;
    float* data;
} Plot;

void plot_alloc(Plot *plot, size_t size) {
    assert(size > 0);

    plot->capacity = size;
    plot->size = 0;
    plot->idx = 0;
    plot->data = malloc(sizeof(*plot->data) * size);

    assert(plot->data != NULL);
}

float plot_nth(Plot* plot, size_t at) {
    assert(plot->size > 0);
    assert(plot->size > at);

    if (plot->size == plot->capacity) {
        return plot->data[(plot->idx + at) % plot->capacity];
    }

    return plot->data[at];
}

void plot_push(Plot* plot, float value) {
    plot->data[plot->idx] = value;
    plot->idx = (plot->idx + 1) % plot->capacity;

    if (plot->size < plot->capacity) {
        plot->size += 1;
    }
}

float plot_max(Plot* plot) {
    assert(plot->size > 0);
    float max = 0.0;

    for (size_t i = 1; i < plot->size; i++) {
        float v = plot_nth(plot, i);
        max = fmax(max, v);
    }

    return max;
}

void plot_render(Plot* plot, int scr_w, int scr_h, int offset_x, int offset_y) {
    assert(plot->size > 0);
    // Draw axis lines
    int pad = 25;
    int width = scr_w - pad * 2;
    int height = scr_h - pad * 2;

    int axis_x = offset_x + pad;
    int axis_y = offset_y + pad;
    DrawLine(axis_x, axis_y - 2, axis_x, axis_y + height + 2, WHITE);
    DrawLine(axis_x - 2, axis_y + height, axis_x + width + 2, axis_y + height, WHITE);

    static float max = 0.0;
    max = fmax(plot_max(plot), max);

    float data_per_pix = (float)width / (float)plot->capacity;
    int lx = offset_x + pad;
    int ly = offset_y + pad + height - (int)(((float)height / max) * fmin(plot_nth(plot, 0), max));

    for (size_t i = 0; i < plot->size; i++) {
        float value = plot_nth(plot, i);
        int y = offset_y + pad + height - (int)(((float)height / max) * fmin(value, max));
        int x = offset_x + pad + (int)((float)i * data_per_pix);

        DrawLine(lx, ly, x, y, WHITE);

        lx = x;
        ly = y;
    }
}

/**
 * Calculate space between padding when `available` space must fit
 * `num` chunks, each `size` big.
 */
int space_between_pad(int available, int num, int size) {
    return (available - (num * size)) / (num - 1);
}

void nero_render(Nero n, int scr_w, int scr_h, int offset_x, int offset_y) {
    Color low_color = { 0xBB, 0x45, 0x45, 0xFF };
    Color high_color = { 0x45, 0xFF, 0x45, 0xFF };
    Color input_color = low_color;

    int pad = 25;
    int width = scr_w - pad * 2;
    int height = scr_h - pad * 2;
    int pad_x = space_between_pad(width, n.depth + 1, RAD * 2);
    int node_dist_x = RAD * 2 + pad_x;

    // Iterate over all layers
    for (size_t l = 0; l <= n.depth; l++) {
        int cx = pad + RAD + l * node_dist_x + offset_x;
        int pad_y = space_between_pad(
            height,
            n.activations[l].cols,
            RAD * 2
        );
        int node_dist_y = RAD * 2 + pad_y;

        // Iterate layer activations
        for (size_t j = 0; j < n.activations[l].cols; j++) {
            int cy = pad + RAD + j * node_dist_y + offset_y;

            if (l == 0) {
                DrawCircle(cx, cy, RAD, input_color);
                continue;
            }

            high_color.a = floorf(255.0 * sigmoidf(M_AT(n.biases[l - 1], 1, j)));
            DrawCircle(cx, cy, RAD, ColorAlphaBlend(low_color, high_color, WHITE));

            // For input layer draw only circles
            if (l == 0) {
                continue;
            }

            for (size_t i = 0; i < n.activations[l - 1].cols; i++) {
                // Draw connection
                int pad_y = space_between_pad(height, n.activations[l - 1].cols, RAD * 2);
                int sx = cx - node_dist_x + RAD;
                int sy = pad + RAD + i * (RAD * 2 + pad_y) + offset_y;

                high_color.a = floorf(255.0 * sigmoidf(M_AT(n.weights[l - 1], i, j)));
                DrawLine(sx, sy, cx - RAD, cy, ColorAlphaBlend(low_color, high_color, WHITE));
            }
        }
    }
}

int main() {
    // Seed
    seed_rand();

    // NN
    Mat m[2];
    init_train_set(m);
    Mat train_input = m[0];
    Mat train_output = m[1];

    size_t layout[] = { 2 * BITS, 4 * BITS, BITS + 1 };
    Nero add = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);
    nero_rand(add, 0, 1);

    InitWindow(SCR_W, SCR_H, "Visualisation demo");
    SetTargetFPS(60);

    // Plot
    Plot plot;
    plot_alloc(&plot, PLOT_ENTRIES);

    size_t epoch = 0;
    float learning_rate = 1e-1;
    while (!WindowShouldClose()) {
        ClearBackground((Color){ 0x44, 0x44, 0x44, 0xFF });
        if (epoch < EPOCHS) {
            for (size_t i = 0; i < 10; i++) {
                nero_backprop(add, grad, train_input, train_output);
                nero_learn(add, grad, learning_rate);
            }
            epoch += 10;
        }

        plot_push(&plot, nero_cost(add, train_input, train_output));

        BeginDrawing();
        nero_render(add, SCR_W / 2, SCR_H, SCR_W / 2, 0);
        plot_render(&plot, SCR_W / 2, SCR_H, 0, 0);

        char buffer[256];
        snprintf(buffer, sizeof(buffer), "Epoch: %4lu/%4d Rate:%f Cost:%f", epoch, EPOCHS, learning_rate, plot_nth(&plot, plot.size - 1));
        DrawText(buffer, 0, 0, SCR_H * 0.025, WHITE);

        EndDrawing();
    }

    CloseWindow();

#ifdef DEBUG
    verify_train_set(add);
#endif

    return 0;
}

