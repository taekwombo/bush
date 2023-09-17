#include "raylib.h"
#define GYM_IMPLEMENTATION
#include "gym.h"
#include <time.h>
#include "stb_image_write.h"

int main(const int argc, const char** argv) {
    size_t out_width = 120, out_height = 120;
    char* out_path = malloc(sizeof(char) * GYM_STR_OPT_LEN);
    strcpy(out_path, "./out.png");

    find_option(argc, argv, "--out-width", &out_width, parse_integer);
    find_option(argc, argv, "--out-height", &out_height, parse_integer);
    find_option(argc, argv, "--out-path", &out_path, get_string_opt_value);

    int width, height;
    Mat pixels;
    img2mat("./8.png", &pixels, &width, &height);
    Mat t_in = mat_view(pixels, 0, 0, pixels.rows, pixels.cols - 1);
    Mat t_out = mat_view(pixels, 0, 2, pixels.rows, 1);

    size_t layout[] = { 2, 8, 4, 1 };
    Nero net = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);

    GymConfig config;
    read_config(argc, argv, &config);
    config.input = net_input(&net, &grad, t_in, t_out);

    gym(config);

    uint8_t* out = malloc(sizeof(*out) * out_width * out_height);

    for (size_t x = 0; x < out_width; x++) {
        for (size_t y = 0; y < out_height; y++) {
            M_AT(NERO_INPUT(net), 0, 0) = (float)x / (float)(out_width - 1);
            M_AT(NERO_INPUT(net), 0, 1) = (float)y / (float)(out_height - 1);
            nero_forward(net);

            uint8_t pix = M_AT(NERO_OUTPUT(net), 0, 0) * 255.0;
            out[y * out_width + x] = pix;
        }
    }

    if (!stbi_write_png(out_path, out_width, out_height, 1, out, out_width * sizeof(*out))) {
        printf("Couldn't save a file at '%s'.\n", out_path);
    }
}
