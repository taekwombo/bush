#ifndef GYM_IMAGE_H
#define GYM_IMAGE_H

#include "nero.h"
#include "stb_image.h"

/**
 * Load 8 bit png image into Mat.
 */
void png2mat(char* path, Mat* result);

#endif

// ------ IMPLEMENTATION -------

#ifdef GYM_IMAGE_IMPLEMENTATION

#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#undef STB_IMAGE_IMPLEMENTATION

void img2mat(char *path, Mat* result) {
    int width, height, channels;
    uint8_t *pixels = stbi_load(path, &width, &height, &channels, 0);

    assert(pixels != NULL /* Could not laod image. */);
    assert(channels == 1 /* Only 8 bit images are supported. */);

    *result = mat_alloc(width * height, 3);

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            size_t idx = y * width + x;

            M_AT(*result, idx, 0) = (float)x / (float)(width - 1);
            M_AT(*result, idx, 1) = (float)y / (float)(height - 1);
            M_AT(*result, idx, 2) = (float)pixels[idx] / 255.0;
        }
    }
}

#endif
