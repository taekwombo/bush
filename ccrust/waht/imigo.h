#ifndef IMIGO_H
#define IMIGO_H

#undef NERO_IMPLEMENTATION
#include "nero.h"
#include "png.h"
#include <stdio.h>

/**
 * Load png image into Mat.
 */
void png2mat(char* path, Mat* result);

#endif

// ------ IMPLEMENTATION -------

#ifdef IMIGO_IMPLEMENTATION

void png2mat(char *path, Mat* result) {
    FILE *file = fopen(path, "r");
    unsigned char header[9];

    assert(file);
    fread(header, 1, 8, file);
    assert(png_sig_cmp(header, 0, 8) == 0);

    png_structp png = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
    assert(png);
    png_infop info = png_create_info_struct(png);
    assert(info);

    png_init_io(png, file);
    png_set_sig_bytes(png, 8);
    png_read_info(png, info);

    png_uint_32 width, height;

    assert(png_get_IHDR(png, info, &width, &height, NULL, NULL, NULL, NULL, NULL));

    *result = mat_alloc(width, height);
}

#endif
