#ifndef NERO_H
#define NERO_H

#include <stdlib.h>
#include <stddef.h>
#include <stdio.h>
#include <assert.h>
#include <math.h>

#ifndef bool
typedef enum {
    false = 0,
    true = 1
} bool;
#endif

float rand_float();
float sigmoidf(float x);
float dsigmoidf(float x);
float tanhf(float x);

// ------ MATRIX -------

typedef struct {
    size_t rows;
    size_t cols;
    size_t stride;
    float  *els;
} Mat;

typedef float (*ActFn)(float x);

/** Return new Mat with enough memory allocated for rows * cols elements. */
Mat mat_alloc(size_t rows, size_t cols);
void mat_free(Mat *m);

/** Return new Mat representing single row of src matrix. */
Mat mat_row(Mat src, size_t row);
Mat mat_view(
    Mat src,
    size_t row_start,
    size_t col_start,
    size_t rows,
    size_t cols
);

bool mat_eq(Mat a, Mat b);
void mat_fill(Mat m, float value);
void mat_cpy(Mat dst, Mat src);
void mat_add(Mat dst, Mat src);
void mat_sub(Mat dst, Mat src);
void mat_mul(Mat dst, Mat a, Mat b);
void mat_mul_scalar(Mat dst, float value);
void mat_act(Mat m, ActFn fn);
void mat_rand(Mat m, float low, float high);
void mat_swap(Mat a, Mat b);

void mat_print(Mat m, const char *name, size_t pad);
void mat_debug(Mat m, const char *name, size_t pad);

#define M_AT(m, i, j) (m).els[(i) * (m).stride + (j)]
#define M_PRINT(m) mat_print(m, #m, 0)
#define M_DEBUG(m) mat_debug(m, #m, 0)

// ------ Nero -------

#define ARR_LEN(xs) sizeof((xs))/sizeof((xs)[0])

typedef enum {
    SIG,
} ActivationKind;

typedef struct {
    size_t depth;
    ActivationKind act_fn;
    Mat *weights;
    /** biases */
    Mat *biases;
    /** activations - depth + 1 elements */
    Mat *activations;
} Nero;

typedef struct {
    size_t batch_count;
    Mat orig_in;
    Mat orig_out;
    Mat shuffled_in;
    Mat shuffled_out;
    float cost;
} BatchConfig;

#define NERO_PRINT(n) nero_print(n, #n)
#define NERO_INPUT(n) n.activations[0]
#define NERO_OUTPUT(n) n.activations[n.depth]

#ifndef NERO_RELU_MUL
#define NERO_RELU_MUL 0.01f
#endif

Nero nero_alloc(size_t size, size_t layers[]);
void nero_free(Nero n);

BatchConfig nero_batch_config(size_t batch_count, Mat t_in, Mat t_out);
void nero_batch_config_free(BatchConfig bc);

void nero_rand(Nero n, float low, float high);
void nero_forward(Nero n);
float nero_cost(Nero n, Mat t_in, Mat t_out);
void nero_finite_diff(Nero n, Nero grad, float eps, Mat t_in, Mat t_out);
void nero_backprop(Nero n, Nero grad, Mat t_in, Mat t_out);
void nero_run_batches(Nero n, Nero grad, BatchConfig *bc, float learn_rate);
void nero_shuffle_train_data(Mat t_in, Mat t_out);
void nero_zero(Nero n);
void nero_learn(Nero n, Nero grad, float rate);

char* debug_act(ActFn fn);
void nero_print(Nero n, const char *name);

#endif

// ------ IMPLEMENTATION -------

#ifdef NERO_IMPLEMENTATION

float rand_float() {
    return (float) rand() / (float) RAND_MAX;
}

float sigmoidf(float x) {
    return 1.0f / (1.0f + expf(-x));
}

float dsigmoidf(float x) {
    return x * (1.0f - x);
}

float tanhf(float x) {
    float ep = expf(x);
    float em = expf(-x);
    return (ep - em) / (ep + em);
}

// ------ MATRIX -------

Mat mat_alloc(size_t rows, size_t cols) {
    Mat m;
    m.rows = rows;
    m.cols = cols;
    m.stride = cols;
    m.els = malloc(sizeof(*m.els) * rows * cols);
    assert(m.els != NULL);

    return m;
}

void mat_free(Mat *m) {
    free(m->els);
}

Mat mat_row(Mat src, size_t row) {
    assert(row < src.rows);

    return (Mat) {
        .rows = 1,
        .cols = src.cols,
        .stride = src.stride,
        .els = &M_AT(src, row, 0),
    };
}

Mat mat_view(
    Mat src,
    size_t row_start,
    size_t col_start,
    size_t rows,
    size_t cols
) {
    assert(row_start < src.rows);
    assert(col_start < src.cols);
    assert((src.rows - row_start) >= rows);
    assert((src.cols - col_start) >= cols);

    return (Mat) {
        .rows = rows,
        .cols = cols,
        .stride = src.stride,
        .els = &M_AT(src, row_start, col_start)
    };
}

bool mat_eq(Mat a, Mat b) {
    assert(a.rows == b.rows);
    assert(a.cols == b.cols);

    for (size_t i = 0; i < a.rows; i++) {
        for (size_t j = 0; j < a.cols; j++) {
            if (M_AT(a, i, j) != M_AT(b, i, j)) {
                return false;
            }
        }
    }

    return true;
}

void mat_fill(Mat m, float value) {
    for (size_t i = 0; i < m.rows; i++) {
        for (size_t j = 0; j < m.cols; j++) {
            M_AT(m, i, j) = value;
        }
    }
}

void mat_cpy(Mat dst, Mat src) {
    assert(dst.rows == src.rows);
    assert(dst.cols == src.cols);

    for (size_t i = 0; i < dst.rows; i++) {
        for (size_t j = 0; j < dst.cols; j++) {
            M_AT(dst, i, j) = M_AT(src, i, j);
        }
    }
}

void mat_add(Mat dst, Mat src) {
    assert(dst.rows == src.rows);
    assert(dst.cols == src.cols);

    for (size_t i = 0; i < dst.rows; i++) {
        for (size_t j = 0; j < dst.cols; j++) {
            M_AT(dst, i, j) += M_AT(src, i, j);
        }
    }
}

void mat_sub(Mat dst, Mat src) {
    assert(dst.rows == src.rows);
    assert(dst.cols == src.cols);

    for (size_t i = 0; i < dst.rows; i++) {
        for (size_t j = 0; j < dst.cols; j++) {
            M_AT(dst, i, j) -= M_AT(src, i, j);
        }
    }
}

void mat_mul(Mat dst, Mat a, Mat b) {
    assert(a.cols == b.rows);
    assert(dst.rows == a.rows);
    assert(dst.cols == b.cols);

    size_t inner_size = a.cols;

    for (size_t di = 0; di < dst.rows; di++) {
        for (size_t dj = 0; dj < dst.cols; dj++) {
            M_AT(dst, di, dj) = 0;

            for (size_t k = 0; k < inner_size; k++) {
                M_AT(dst, di, dj) += M_AT(a, di, k) * M_AT(b, k, dj);
            }
        }
    }
}

void mat_mul_scalar(Mat dst, float value) {
    for (size_t di = 0; di < dst.rows; di++) {
        for (size_t dj = 0; dj < dst.cols; dj++) {
            M_AT(dst, di, dj) *= value;
        }
    }
}

void mat_act(Mat m, ActFn fn) {
    for (size_t i = 0; i < m.rows; i++) {
        for (size_t j = 0; j < m.cols; j++) {
            M_AT(m, i, j) = fn(M_AT(m, i, j));
        }
    }
}

void mat_rand(Mat m, float low, float high) {
    for (size_t i = 0; i < m.rows; i++) {
        for (size_t j = 0; j < m.cols; j++) {
            M_AT(m, i, j) = low + (rand_float() * high - low);
        }
    }
}

void mat_swap(Mat a, Mat b) {
    assert(a.rows == b.rows);
    assert(a.cols == b.cols);
    for (size_t i = 0; i < a.rows; i++) {
        for (size_t j = 0; j < b.cols; j++) {
            float tmp = M_AT(a, i, j);
            M_AT(a, i, j) = M_AT(b, i, j);
            M_AT(b, i, j) = tmp;
        }
    }
}

void mat_print(Mat m, const char* name, size_t pad) {
    printf("%*s%s[%zux%zu] = [\n", (int)pad, "", name, m.rows, m.cols);
    for (size_t i = 0; i < m.rows; i++) {
        printf("  %*s", (int)pad, "");
        for (size_t j = 0; j < m.cols; j++) {
            printf("%.4f  ", M_AT(m, i, j));
        }
        printf("\n");
    }
    printf("%*s]\n", (int)pad, "");
}

void mat_debug(Mat m, const char* name, size_t pad) {
    printf("%*s %s {\t rows:%4lu,  cols:%4lu, stride: %lu, els: %p }\n",
            (int)pad, "", name, m.rows, m.cols, m.stride, m.els);
}

// ------ Nero -------

Nero nero_alloc(size_t size, size_t layers[]) {
    assert(size != 0);

    Nero n;
    n.act_fn = SIG;
    n.depth = size - 1;

    n.activations = malloc(sizeof(*n.activations) * size);
    assert(n.activations != NULL);
    n.weights = malloc(sizeof(*n.weights) * n.depth);
    assert(n.weights != NULL);
    n.biases = malloc(sizeof(*n.biases) * n.depth);
    assert(n.biases != NULL);

    for (size_t i = 0; i < size; i++) {
        n.activations[i] = mat_alloc(1, layers[i]);

        if (i == 0) {
            continue;
        }

        n.weights[i - 1] = mat_alloc(layers[i - 1], layers[i]);
        n.biases[i - 1] = mat_alloc(1, layers[i]);
    }

    return n;
}
void nero_free(Nero n) {
    mat_free(&n.activations[0]);
    for (size_t i = 0; i < n.depth - 1; i++) {
        mat_free(&n.activations[i + 1]);
        mat_free(&n.biases[i]);
        mat_free(&n.weights[i]);
    }
}

BatchConfig nero_batch_config(size_t batch_count, Mat t_in, Mat t_out) {
    BatchConfig b;

    b.cost = 0;
    b.batch_count = batch_count;
    b.orig_in = t_in;
    b.orig_out = t_out;
    b.shuffled_in = mat_alloc(t_in.rows, t_in.cols);
    b.shuffled_out = mat_alloc(t_out.rows, t_out.cols);

    mat_cpy(b.shuffled_in, t_in);
    mat_cpy(b.shuffled_out, t_out);

    return b;
}

void nero_batch_config_free(BatchConfig bc) {
    mat_free(&bc.shuffled_in);
    mat_free(&bc.shuffled_out);
}

ActFn nero_get_afn(Nero n) {
    switch (n.act_fn) {
        case SIG:
            return sigmoidf;
        default:
            assert(0 && "Unreachable");
    }
}

ActFn nero_get_dafn(Nero n) {
    switch (n.act_fn) {
        case SIG:
            return dsigmoidf;
        default:
            assert(0 && "Unreachable");
    }
}

void nero_rand(Nero n, float low, float high) {
    for (size_t i = 0; i < n.depth; i++) {
        mat_rand(n.weights[i], low, high);
        mat_rand(n.biases[i], low, high);
    }
}

void nero_forward(Nero n) {
    ActFn fn = nero_get_afn(n);
    for (size_t i = 0; i < n.depth; i++) {
        mat_mul(n.activations[i + 1], n.activations[i], n.weights[i]);
        mat_add(n.activations[i + 1], n.biases[i]);
        mat_act(n.activations[i + 1], fn);
    }
}

float nero_cost(Nero n, Mat t_in, Mat t_out) {
    assert(t_in.rows == t_out.rows);
    assert(t_out.cols == NERO_OUTPUT(n).cols);

    float cost = 0.0;
    size_t train_count = t_in.rows;

    for (size_t i = 0; i < train_count; i++) {
        Mat input = mat_row(t_in, i);
        Mat output = mat_row(t_out, i);

        mat_cpy(NERO_INPUT(n), input);
        nero_forward(n);

        for (size_t j = 0; j < t_out.cols; j++) {
            float dist = M_AT(NERO_OUTPUT(n), 0, j) - M_AT(output, 0, j);
            cost += dist * dist;
        }
    }

    return cost / train_count;
}

void nero_finite_diff(Nero n, Nero grad, float eps, Mat t_in, Mat t_out) {
    assert(n.depth == grad.depth);

    float saved = 0;
    float cost = nero_cost(n, t_in, t_out);

    for (size_t l = 0; l < n.depth; l++) {
        for (size_t i = 0; i < n.weights[l].rows; i++) {
            for (size_t j = 0; j < n.weights[l].cols; j++) {
                saved = M_AT(n.weights[l], i, j);
                M_AT(n.weights[l], i, j) += eps;
                M_AT(grad.weights[l], i, j) = (nero_cost(n, t_in, t_out) - cost) / eps;
                M_AT(n.weights[l], i, j) = saved;
            }
        }
        for (size_t i = 0; i < n.biases[l].rows; i++) {
            for (size_t j = 0; j < n.biases[l].cols; j++) {
                saved = M_AT(n.biases[l], i, j);
                M_AT(n.biases[l], i, j) += eps;
                M_AT(grad.biases[l], i, j) = (nero_cost(n, t_in, t_out) - cost) / eps;
                M_AT(n.biases[l], i, j) = saved;
            }
        }
    }
}

void nero_backprop(Nero n, Nero grad, Mat t_in, Mat t_out) {
    assert(t_in.rows == t_out.rows);
    assert(NERO_OUTPUT(n).cols == t_out.cols);

    nero_zero(grad);

    size_t t = t_in.rows;
    ActFn dact = nero_get_dafn(n);

    for (size_t s/*ample*/ = 0; s < t; s++) {
        mat_cpy(NERO_INPUT(n), mat_row(t_in, s));
        nero_forward(n);

        for (size_t l = 0; l <= grad.depth; l++) {
            mat_fill(grad.activations[l], 0);
        }

        for (size_t c = 0; c < NERO_OUTPUT(n).cols; c++) {
            M_AT(NERO_OUTPUT(grad), 0, c) = M_AT(NERO_OUTPUT(n), 0, c) - M_AT(t_out, s, c);
        }

        for (size_t l/*ayer*/ = n.depth; l > 0; l--) {
            for (size_t i = 0; i < n.activations[l].cols; i++) {
                float a = M_AT(n.activations[l], 0, i);
                float d_a = M_AT(grad.activations[l], 0, i);
                float d = 2 * d_a * dact(a);

                M_AT(grad.biases[l - 1], 0, i) += d;

                for (size_t j = 0; j < n.activations[l - 1].cols; j++) {
                    M_AT(grad.weights[l - 1], j, i)     += d * M_AT(n.activations[l - 1], 0, j);
                    M_AT(grad.activations[l - 1], 0, j) += d * M_AT(n.weights[l - 1], j, i);
                }
            }
        }
    }

    float grad_mul = 1.0 / (float)t;
    assert(!isnan(grad_mul) && "nero_backprop");
    for (size_t l/*ayer*/ = n.depth; l > 0; l--) {
        mat_mul_scalar(grad.weights[l - 1], grad_mul);
        mat_mul_scalar(grad.biases[l - 1], grad_mul);
    }
}

void nero_run_batches(Nero n, Nero grad, BatchConfig *bc, float learn_rate) {
    size_t rows = bc->shuffled_in.rows;
    size_t count = bc->batch_count;
    size_t start = 0;
    size_t batch_size = (rows + count - 1) / count;
    assert(batch_size * count >= rows);

    float cost = 0;
    Mat in = bc->shuffled_in;
    Mat out = bc->shuffled_out;
    nero_shuffle_train_data(in, out);

    while (start * batch_size <= rows) {
        size_t batch_rows = start >= rows ? rows - start : batch_size;
        assert(batch_rows + start <= rows);

        Mat bin = mat_view(in, start, 0, batch_rows, in.cols);
        Mat bout = mat_view(out, start, 0, batch_rows, out.cols);

        nero_backprop(n, grad, bin, bout);
        nero_learn(n, grad, learn_rate);

        cost += nero_cost(n, bin, bout);
        start += batch_size;
    }

    bc->cost = cost / (float)count;
    assert(!isnan(bc->cost));
}

void nero_shuffle_train_data(Mat t_in, Mat t_out) {
    assert(t_in.rows == t_out.rows);

    for (size_t i = 0; i < t_in.rows; i++) {
        size_t idx = i + (rand() % (t_in.rows - i));
        assert(idx < t_in.rows);

        if (idx == i) {
            continue;
        }

        // Swap input
        mat_swap(mat_row(t_in, i), mat_row(t_in, idx));
        // Swap output
        mat_swap(mat_row(t_out, i), mat_row(t_out, idx));
    }
}

void nero_zero(Nero n) {
    for (size_t l = 0; l < n.depth; l++) {
        mat_fill(n.activations[l], 0);
        mat_fill(n.weights[l], 0);
        mat_fill(n.biases[l], 0);
    }
    mat_fill(n.activations[n.depth], 0);
}

void nero_learn(Nero n, Nero grad, float rate) {
    assert(n.depth == grad.depth);

    for (size_t i = 0; i < n.depth; i++) {
        mat_mul_scalar(grad.weights[i], rate);
        mat_mul_scalar(grad.biases[i], rate);

        mat_sub(n.weights[i], grad.weights[i]);
        mat_sub(n.biases[i], grad.biases[i]);
    }
}

char* debug_act(ActFn fn) {
    if (fn == sigmoidf) {
        return "sigmoid";
    } else {
        return "unknown";
    }
}

void nero_print(Nero n, const char *name) {
    printf("%s = {\n", name);
    printf("    .depth = %lu\n", n.depth);
    printf("    .act = %s\n", debug_act(nero_get_afn(n)));

    printf("    .layout = { ");
    for (size_t i = 0; i < n.depth; i++) {
        printf("%lu, ", n.weights[i].rows);
        if (i == n.depth - 1) {
            printf("%lu ", n.weights[i].cols);
        }
    }

    printf("}\n}\n");
}

#endif
