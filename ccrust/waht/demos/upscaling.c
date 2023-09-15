#include "raylib.h"
#define GYM_IMPLEMENTATION
#include "gym.h"
#include <time.h>

int main(const int argc, const char** argv) {
    Mat pixels;
    img2mat("./8.png", &pixels);
    Mat t_in = mat_view(pixels, 0, 0, pixels.rows, pixels.cols - 1);
    Mat t_out = mat_view(pixels, 0, 2, pixels.rows, 1);

    GymRender render_config = read_render_config(argc, argv);
    GymTrain train_config = read_train_config(argc, argv);

    size_t layout[] = { 2, 28, 1 };
    Nero net = nero_alloc(ARR_LEN(layout), layout);
    Nero grad = nero_alloc(ARR_LEN(layout), layout);

    gym(render_config, train_config, (GymInput){
        .net = net,
        .grad = grad,
        .t_in = t_in,
        .t_out = t_out
    });
}
