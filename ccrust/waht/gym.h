#ifndef GYM_H
#define GYM_H

#include "raylib.h"
#include "nero.h"
#include "gym_render.h"
#include "gym_image.h"
#include <string.h>
#include <stdlib.h>
#include <time.h>

#define GYM_STR_OPT_LEN 512

typedef struct {
    size_t width;
    size_t height;
    size_t neuron_radius;
    size_t plot_entries;
    size_t fps;
    char* title;
} GymRender;

typedef struct {
    float learning_rate;
    size_t epochs;
    size_t learn_per_frame;
    size_t seed;
} GymTrain;

typedef struct {
    Nero net;    
    Nero grad;
    Mat t_in;
    Mat t_out;
} GymInput;

typedef void (*ParseFn)(const char* in, void *out, const char* name);

void find_option(const int argc, const char** argv, const char* name, void *out, ParseFn parse);
void parse_integer(const char* in, void* out, const char* name);
void parse_float(const char* in, void* out, const char* name);
void get_string_opt_value(const char* in, void* out, const char* name);

void gym(GymRender render, GymTrain t, GymInput in);

GymRender read_render_config(const int argc, const char **argv);
GymTrain read_train_config(const int argc, const char **argv);
void print_train_config(GymTrain* t);
void print_render_config(GymRender* r);

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

void gym(GymRender render, GymTrain t, GymInput in) {
    srand(t.seed);
    nero_rand(in.net, 0, 1);

    SetConfigFlags(FLAG_MSAA_4X_HINT);
    printf("Title: %s\n", render.title);
    InitWindow(render.width, render.height, render.title);
    SetTargetFPS(render.fps);

    Plot plot;
    plot_alloc(&plot, render.plot_entries);

    size_t epoch = 0;
    bool running = false;
    bool ignore_epochs_bound = false;
    int key;
    float saved_rate = t.learning_rate;

    while (!WindowShouldClose()) {
        ClearBackground((Color){ 0x44, 0x44, 0x44, 0xFF });

        while ((key = GetKeyPressed())) {
            switch (key) {
                case KEY_SPACE:
                    running = !running;
                    break;

                case KEY_R:
                    epoch = 0;
                    nero_rand(in.net, 0, 1);
                    plot_reset(&plot);
                    break;

                case KEY_EQUAL:
                    t.learning_rate += saved_rate;
                    break;

                case KEY_MINUS:
                    t.learning_rate = fmax(saved_rate, t.learning_rate - saved_rate);
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
                nero_backprop(in.net, in.grad, in.t_in, in.t_out);
                nero_learn(in.net, in.grad, t.learning_rate);
            }
            epoch += t.learn_per_frame;
            plot_push(&plot, nero_cost(in.net, in.t_in, in.t_out));
        }

        BeginDrawing();

        plot_render_text(epoch, t.epochs, t.learning_rate);
        plot_render(&plot, render.width / 2, render.height, 0, 0);
        nero_render(in.net, render.width / 2, render.height, render.width / 2, 0, render.neuron_radius);

        EndDrawing();
    }

    CloseWindow();
}

const char* OPT_RENDER_WIDTH    = "--width";
const char* OPT_RENDER_HEIGHT   = "--height";
const char* OPT_NEURON_RADIUS   = "--radius";
const char* OPT_PLOT_ENTIRES    = "--plot-entries";
const char* OPT_FPS             = "--fps";
const char* OPT_TITLE           = "--title";
const char* OPT_LEARNING_RATE   = "--rate";
const char* OPT_EPOCHS          = "--epochs";
const char* OPT_LEARN_PER_FRAME = "--learn-per-frame";
const char* OPT_SEED            = "--seed";

void parse_float(const char* in, void* out, const char* name) {
    float val = strtof(in, NULL);

    if (val) {
        printf("Parset float value for option '%s': %f\n", name, val);
        *(float*)out = val;
        return;
    }

    printf("Invalid float value for option '%s': %s\n", name, in);
}

void parse_integer(const char* in, void* out, const char* name) {
    long val = strtol(in, NULL, 10);
    
    if (val) {
        printf("Parsed size_t value for option '%s': %lu\n", name, val);
        *(size_t*)out = (size_t)val;
        return;
    }

    printf("Invalid size_t value for option '%s': %s\n", name, in);
}

void get_string_opt_value(const char* in, void* out, const char* name) {
    printf("Assigning value for option '%s': '%s'\n", name, in);
    assert(strlen(in) < GYM_STR_OPT_LEN);
    strcpy((char*)out, in);
}

void find_option(const int argc, const char** argv, const char* name, void *out, ParseFn parse) {
    for (int i = 1; i < argc; i++) {
        // Option must be <opt>=<val>
        // val: at least one character
        size_t opt_len = strlen(argv[i]);
        if (opt_len < strlen(name) + 2) {
            continue;
        }

        if (strncmp(argv[i], name, strlen(name))) {
            continue;
        }

        // Should point at the first character after =.
        const char* opt = argv[i] + strlen(name) + 1;
        parse(opt, out, name);
        return;
    }
}

GymTrain read_train_config(const int argc, const char** argv) {
    float learning_rate = 1e-1;
    size_t epochs = 10000,
           learn_per_frame = 10,
           seed = time(NULL);

    find_option(argc, argv, OPT_EPOCHS, &epochs, parse_integer);
    find_option(argc, argv, OPT_LEARN_PER_FRAME, &learn_per_frame, parse_integer);
    find_option(argc, argv, OPT_LEARNING_RATE, &learning_rate, parse_float);
    find_option(argc, argv, OPT_SEED, &seed, parse_integer);

    return (GymTrain){
        .epochs = epochs,
        .seed = seed,
        .learn_per_frame = learn_per_frame,
        .learning_rate = learning_rate
    };
}

GymRender read_render_config(const int argc, const char** argv) {
    size_t width = 1080,
           height = 720,
           neuron_radius = 15,
           plot_entries = 400,
           fps = 60;
    char* title = malloc(sizeof(char) * GYM_STR_OPT_LEN);
    strcpy(title, "Demo");

    find_option(argc, argv, OPT_RENDER_WIDTH, &width, parse_integer);
    find_option(argc, argv, OPT_RENDER_HEIGHT, &height, parse_integer);
    find_option(argc, argv, OPT_NEURON_RADIUS, &neuron_radius, parse_integer);
    find_option(argc, argv, OPT_PLOT_ENTIRES, &plot_entries, parse_integer);
    find_option(argc, argv, OPT_FPS, &fps, parse_integer);
    find_option(argc, argv, OPT_TITLE, &title, get_string_opt_value);

    return (GymRender){
        .width = width,
        .height = height,
        .neuron_radius = neuron_radius,
        .plot_entries = plot_entries,
        .fps = fps,
        .title = title
    };
}

void print_render_config(GymRender* r) {
    printf(
        "GymRender {\n"
        "   .width: %lu\n"
        "   .height: %lu\n"
        "   .neuron_radius: %lu\n"
        "   .plot_entries: %lu\n"
        "   .fps: %lu\n"
        "   .title: %s\n"
        "}\n",
         r->width,
         r->height,
         r->neuron_radius,
         r->plot_entries,
         r->fps,
         r->title
     );
}

void print_train_config(GymTrain* t) {
    printf(
        "GymTrain {\n"
        "  .learning_rate: %f\n"
        "  .learn_per_frame: %lu\n"
        "  .epochs: %lu\n"
        "  .seed: %lu\n"
        "}\n",
        t->learning_rate,
        t->learn_per_frame,
        t->epochs,
        t->seed
    );
}

#endif
