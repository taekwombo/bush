#ifndef RENDER_H
#define RENDER_H

#include "raylib.h"
#undef NERO_IMPLEMENTATION
#include "nero.h"

typedef struct {
    size_t capacity;
    size_t idx;
    size_t size;
    float* data;
} Plot;

void plot_alloc(Plot *plot, size_t size);
void plot_reset(Plot *plot);
void plot_push(Plot *plot, float value);
void plot_render_text(size_t epoch, size_t epochs, float rate);
void plot_render(Plot* plot, int scr_w, int scr_h, int offset_x, int offset_y);

void nero_render(Nero n, int scr_w, int scr_h, int offset_x, int offset_y);

#endif
#ifdef RENDER_IMPLEMENTATION

void plot_alloc(Plot *plot, size_t size) {
    assert(size > 0);

    plot->capacity = size;
    plot->size = 0;
    plot->idx = 0;
    plot->data = malloc(sizeof(*plot->data) * size);

    assert(plot->data != NULL);
}

float plot_nth(Plot* plot, size_t at) {
    assert(plot->size > at);

    if (plot->size == plot->capacity) {
        return plot->data[(plot->idx + at) % plot->capacity];
    }

    return plot->data[at];
}

void plot_grow(Plot* plot) {
    float *old = plot->data;
    plot->capacity *= 2;
    plot->data = malloc(sizeof(*plot->data) * plot->capacity);

    free(old);
}

void plot_reset(Plot *plot) {
    plot->idx = 0;
    plot->size = 0;
}

void plot_push(Plot* plot, float value) {
    plot->data[plot->idx] = value;
    plot->idx = (plot->idx + 1) % plot->capacity;

    if (plot->size < plot->capacity) {
        plot->size += 1;
    }
}

float plot_max(Plot* plot) {
    float max = 0.0;

    for (size_t i = 0; i < plot->size; i++) {
        float v = plot_nth(plot, i);
        max = fmax(max, v);
    }

    return max;
}

void plot_render_text(size_t epoch, size_t epochs, float rate) {
    static char buffer[256];

    snprintf(
        buffer, sizeof(buffer),
        "Epoch: %4lu/%4lu Rate:%4f",
        epoch, epochs, rate
    );

    DrawText(buffer, 0, 0, 10, WHITE);
}

void plot_render(Plot* plot, int scr_w, int scr_h, int offset_x, int offset_y) {
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

    char buffer[16];
    snprintf(buffer, sizeof(buffer), "%.4f", plot_nth(plot, plot->size - 1));

    DrawText(buffer, lx - 20, ly + 5, 10, WHITE);
}

/**
 * Calculate space between padding when `available` space must fit
 * `num` chunks, each `size` big.
 */
int space_between_pad(int available, int num, int size) {
    return (available - (num * size)) / (num - 1);
}

Color interpolate_color(Color low, Color high, Color mid, float sig) {
    float l = fmax(0.0, 0.5 - sig) / 0.5;
    float h = fmax(0.0, sig - 0.5) / 0.5;
    float m = fmax(1.0 - (l + h), 0.0);

    char red =      l * low.r +     h * high.r +   m * mid.r;
    char green =    l * low.g +     h * high.g +   m * mid.g;
    char blue =     l * low.b +     h * high.b +   m * mid.b;

    return (Color){ red, green, blue, 0xFF };
}

void nero_render(Nero n, int scr_w, int scr_h, int offset_x, int offset_y) {
    Color low_color = { 0xE6, 0xA3, 0x9E, 0xFF };
    Color high_color = { 0x20, 0xF5, 0x4E, 0xFF };
    Color input_color = { 0x70, 0x70, 0x70, 0xFF };

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

            float sig = sigmoidf(M_AT(n.biases[l - 1], 1, j));
            DrawCircle(cx, cy, RAD, interpolate_color(low_color, high_color, input_color, sig));

            for (size_t i = 0; i < n.activations[l - 1].cols; i++) {
                // Draw connection
                int pad_y = space_between_pad(height, n.activations[l - 1].cols, RAD * 2);
                int sx = cx - node_dist_x + RAD;
                int sy = pad + RAD + i * (RAD * 2 + pad_y) + offset_y;

                float sig = sigmoidf(M_AT(n.weights[l - 1], i, j));
                DrawLineEx(
                    (Vector2){ .x = sx, .y = sy },
                    (Vector2){ .x = cx - RAD, .y = cy },
                    1.5,
                    interpolate_color(low_color, high_color, input_color, sig)
                );
            }
        }
    }
}

#endif
