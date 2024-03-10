#ifndef GYM_CONFIG_H
#define GYM_CONFIG_H

#define GYM_STR_OPT_LEN 512

typedef void (*PreviewFn)(const Nero net, int width, int height, int offset_x, int offset_y);

typedef struct {
    size_t width;
    size_t height;
    size_t fps;
    char* title;
} GymWindowConfig;

typedef struct {
    float min_learning_rate;
    float learning_rate_mul;
    float learning_rate;
    size_t learn_per_frame;
    size_t batch_count;
    size_t epochs;
    size_t seed;
} GymTrainConfig;

typedef struct {
    Nero* net;    
    Nero* grad;
    size_t* layout;
    size_t layout_size;
    Mat t_in;
    Mat t_out;
} GymInput;

typedef struct {
    GymWindowConfig window;
    GymTrainConfig  train;
    GymInput        input;
    size_t neuron_radius;
    size_t plot_entries;
    PreviewFn preview;
} GymConfig;

void parse_integer(const char* in, void* out, const char* name);
void parse_float(const char* in, void* out, const char* name);
void get_string_opt_value(const char* in, void* out, const char* name);

typedef void (*ParseFn)(const char* in, void *out, const char* name);

void find_option(const int argc, const char** argv, const char* name, void *out, ParseFn parse);

GymInput layout_input(size_t* layout, size_t layout_size, Mat train_input, Mat train_output);
GymInput net_input(Nero* net, Nero* grad, Mat train_input, Mat train_output);
void read_config(const int argc, const char** argv, GymConfig* config);
void print_config(GymConfig* config);

#endif

#ifdef GYM_CONFIG_IMPLEMENTATION

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
        printf("Parsed size_t value for option '%s': %zu\n", name, val);
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

const char* OPT_WIDTH           = "--width";
const char* OPT_HEIGHT          = "--height";
const char* OPT_FPS             = "--fps";
const char* OPT_TITLE           = "--title";
const char* OPT_NEURON_RADIUS   = "--radius";
const char* OPT_PLOT_ENTIRES    = "--plot-entries";

const char* OPT_MIN_LR          = "--lr-min";
const char* OPT_LEARN_RATE_MUL  = "--lr-mul";
const char* OPT_LEARNING_RATE   = "--lr";
const char* OPT_LEARN_PER_FRAME = "--learn-per-frame";
const char* OPT_BATCH_COUNT     = "--batch-count";
const char* OPT_EPOCHS          = "--epochs";
const char* OPT_SEED            = "--seed";

void read_train_config(const int argc, const char** argv, GymTrainConfig* config) {
    size_t epochs = 10000,
           seed = time(NULL),
           learn_per_frame = 20,
           batch_count = 1;
    float min_learning_rate = 1e-2,
          learning_rate_mul = 2.0,
          learning_rate = 1e-1;

    find_option(argc, argv, OPT_EPOCHS, &epochs, parse_integer);
    find_option(argc, argv, OPT_SEED, &seed, parse_integer);
    find_option(argc, argv, OPT_LEARN_PER_FRAME, &learn_per_frame, parse_integer);
    find_option(argc, argv, OPT_BATCH_COUNT, &batch_count, parse_integer);
    assert(epochs > 1);
    assert(learn_per_frame > 1 && learn_per_frame < 1000);
    assert(batch_count >= 1);

    find_option(argc, argv, OPT_LEARNING_RATE, &learning_rate, parse_float);
    find_option(argc, argv, OPT_MIN_LR, &min_learning_rate, parse_float);
    find_option(argc, argv, OPT_LEARN_RATE_MUL, &learning_rate_mul, parse_float);

    assert(learning_rate && isfinite(learning_rate));
    assert(min_learning_rate && isfinite(min_learning_rate));
    assert(learning_rate_mul && isfinite(learning_rate_mul));

    config->seed = seed;
    config->epochs = epochs;
    config->learn_per_frame = learn_per_frame;
    config->batch_count = batch_count;

    config->learning_rate = learning_rate;
    config->learning_rate_mul = learning_rate_mul;
    config->min_learning_rate = min_learning_rate;
}

void read_window_config(const int argc, const char** argv, GymWindowConfig* config) {
    size_t width = 1080,
           height = 720,
           fps = 60;

    char* title = malloc(sizeof(char) * GYM_STR_OPT_LEN);
    strcpy(title, "Demo");

    find_option(argc, argv, OPT_WIDTH, &width, parse_integer);
    find_option(argc, argv, OPT_HEIGHT, &height, parse_integer);
    find_option(argc, argv, OPT_FPS, &fps, parse_integer);
    find_option(argc, argv, OPT_TITLE, &title, get_string_opt_value);

    assert(width > 360);
    assert(height > 240);
    assert(fps > 1 && fps < 240);

    config->width = width;
    config->height = height;
    config->fps = fps;
    config->title = title;
}

void read_widget_config(const int argc, const char** argv, GymConfig* config) {
    size_t neuron_radius = 15,
           plot_entries = 400;

    find_option(argc, argv, OPT_NEURON_RADIUS, &neuron_radius, parse_integer);
    find_option(argc, argv, OPT_NEURON_RADIUS, &plot_entries, parse_integer);

    assert(neuron_radius > 1 && neuron_radius < 100);
    assert(plot_entries > 100 && plot_entries < 9000);

    config->neuron_radius = neuron_radius;
    config->plot_entries = plot_entries;
}

GymInput net_input(Nero* net, Nero* grad, Mat train_input, Mat train_output) {
    return (GymInput){
        .layout = NULL,
        .layout_size = 0,
        .net = net,
        .grad = grad,
        .t_in = train_input,
        .t_out = train_output
    };
}

GymInput layout_input(size_t* layout, size_t layout_size, Mat train_input, Mat train_output) {
    return (GymInput){
        .layout = layout,
        .layout_size = layout_size,
        .net = NULL,
        .grad = NULL,
        .t_in = train_input,
        .t_out = train_output
    };
}

void read_config(const int argc, const char** argv, GymConfig* config) {
    config->preview = NULL;
    read_train_config(argc, argv, &config->train);
    read_window_config(argc, argv, &config->window);
    read_widget_config(argc, argv, config);
}

void print_config(GymConfig* config) {
    printf(
        "GymConfig {\n"
        "   neuron_radius: %zu\n"
        "   plot_entries: %zu\n"
        "   window: GymWindowConfig {\n"
        "       width: %zu\n"
        "       height: %zu\n"
        "       fps: %zu\n"
        "       title: '%s'\n"
        "   }\n"
        "   train: GymTrainConfig {\n"
        "       min_learning_rate: %.04f\n"
        "       learning_rate: %.04f\n"
        "       learning_rate_mul: %.04f\n"
        "       learn_per_frame: %zu\n"
        "       batch_count: %zu\n"
        "       epochs: %zu\n"
        "       seed: %zu\n"
        "   }\n"
        "}\n",
        config->neuron_radius,
        config->plot_entries,
        config->window.width,
        config->window.height,
        config->window.fps,
        config->window.title,
        config->train.min_learning_rate,
        config->train.learning_rate,
        config->train.learning_rate_mul,
        config->train.learn_per_frame,
        config->train.batch_count,
        config->train.epochs,
        config->train.seed
    );
}

#endif
