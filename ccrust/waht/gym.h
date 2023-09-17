#ifndef GYM_H
#define GYM_H

#include "raylib.h"
#include "nero.h"
#include "gym_render.h"
#include "gym_image.h"
#include "gym_config.h"
#include <string.h>
#include <stdlib.h>
#include <time.h>

void gym(GymConfig config);

#endif

#ifdef GYM_IMPLEMENTATION

#define NERO_IMPLEMENTATION
#include "nero.h"
#undef NERO_IMPLEMENTATION

#define RENDER_IMPLEMENTATION
#include "gym_render.h"
#undef RENDER_IMPLEMENTATION

#define GYM_IMAGE_IMPLEMENTATION
#include "gym_image.h"
#undef GYM_IMAGE_IMPLEMENTATION

#define GYM_CONFIG_IMPLEMENTATION
#include "gym_config.h"
#undef GYM_CONFIG_IMPLEMENTATION

void init_nero(GymInput input, Nero* n, Nero* g) {
    if (input.net != NULL) {
        assert(input.grad != NULL);

        *n = *input.net;
        *g = *input.grad;
        return;
    }

    assert(input.layout != NULL);
    assert(input.layout_size > 0);

    *n = nero_alloc(input.layout_size, input.layout);
    *g = nero_alloc(input.layout_size, input.layout);
}

void gym(GymConfig config) {
    Nero net, grad;
    init_nero(config.input, &net, &grad);

    GymTrainConfig t = config.train;
    GymWindowConfig w = config.window;
    GymInput in = config.input;

    srand(config.train.seed);
    nero_rand(net, 0, 1);

    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(w.width, w.height, w.title);
    SetTargetFPS(w.fps);

    Plot plot;
    plot_alloc(&plot, config.plot_entries);

    size_t epoch = 0;
    bool running = false;
    bool ignore_epochs_bound = false;
    int key;
    float saved_rate = t.learning_rate;

    while (!WindowShouldClose()) {
        while ((key = GetKeyPressed())) {
            switch (key) {
                case KEY_SPACE:
                    running = !running;
                    break;

                case KEY_R:
                    epoch = 0;
                    nero_rand(net, 0, 1);
                    plot_reset(&plot);
                    break;

                case KEY_EQUAL:
                    t.learning_rate *= 2.0;
                    break;

                case KEY_MINUS:
                    t.learning_rate = fmax(t.min_learning_rate, t.learning_rate * 0.5);
                    break;

                case KEY_ZERO:
                    t.learning_rate = saved_rate;
                    break;

                case KEY_C:
                    if (epoch == t.epochs) {
                        ignore_epochs_bound = true;
                    }
                    break;

                default:
                    break;
            }
        }

        if (running && (ignore_epochs_bound || epoch < t.epochs)) {
            for (size_t i = 0; i < t.learn_per_frame; i++) {
                nero_backprop(net, grad, in.t_in, in.t_out);
                nero_learn(net, grad, t.learning_rate);
            }
            epoch += t.learn_per_frame;
            plot_push(&plot, nero_cost(net, in.t_in, in.t_out));
        }

        ClearBackground((Color){ 0x44, 0x44, 0x44, 0xFF });
        BeginDrawing();

        plot_render_text(epoch, t.epochs, t.learning_rate);
        plot_render(&plot, w.width / 2, w.height, 0, 0);
        nero_render(net, w.width / 2, w.height, w.width / 2, 0, config.neuron_radius);

        EndDrawing();
    }

    CloseWindow();
}

#endif
