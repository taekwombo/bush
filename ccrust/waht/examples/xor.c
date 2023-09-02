#include <stdlib.h>
#include <math.h>
#include <stdio.h>
#include <time.h>

// XOR: A | B & ~(A & B)

typedef struct {
    float or_w1;
    float or_w2;
    float or_b;

    float and_w1;
    float and_w2;
    float and_b;

    float nand_w1;
    float nand_w2;
    float nand_b;
} Xor;

float sigmoidf(float x) {
    return 1.0f / (1.f + expf(-x));
}

float forward(Xor model, float x1, float x2) {
    // A | B
    float a = sigmoidf(model.or_w1 * x1 + model.or_w2 * x2 + model.or_b);
    // ~(A & B)
    float b = sigmoidf(model.nand_w1 * x1 + model.nand_w2 * x2 + model.nand_b);

    return sigmoidf(a * model.and_w1 + b * model.and_w2 + model.and_b);
}

typedef float sample[3];

sample train[] = {
    {0, 0, 0},
    {1, 0, 1},
    {0, 1, 1},
    {1, 1, 0},
};

#define TRAIN_CNT 4

float cost_fn(Xor model) {
    float result = 0;

    for (size_t i = 0; i < TRAIN_CNT; i++) {
        float x1 = train[i][0];
        float x2 = train[i][1];
        float output = forward(model, x1, x2);
        float expected = train[i][2];
        float distance = output - expected;
        result += distance * distance;
    }

    return result / TRAIN_CNT;
}


float rand_float(void) {
    return (float) rand() / (float) RAND_MAX;
}

void seed(void) {
    srand(time(NULL));
    rand_float();
}

Xor rand_xor() {
    return (Xor) {
        .or_w1 = rand_float(),
        .or_w2 = rand_float(),
        .or_b = rand_float(),

        .and_w1 = rand_float(),
        .and_w2 = rand_float(),
        .and_b = rand_float(),

        .nand_w1 = rand_float(),
        .nand_w2 = rand_float(),
        .nand_b = rand_float(),
    };
}

void print_xor(Xor *model) {
    printf("Xor {\n");
    printf("\tor_w1 = %f\n", model->or_w1);
    printf("\tor_w2 = %f\n", model->or_w2);
    printf("\tor_b = %f\n", model->or_b);

    printf("\tand_w1 = %f\n", model->and_w1);
    printf("\tand_w2 = %f\n", model->and_w2);
    printf("\tand_b = %f\n", model->and_b);

    printf("\tnand_w1 = %f\n", model->nand_w1);
    printf("\tnand_w2 = %f\n", model->nand_w2);
    printf("\tnand_b = %f\n", model->nand_b);
    printf("}\n");
}

Xor finite_difference(Xor model, float eps) {
#define SAV_RES(field)  \
    saved = model.field;  \
    model.field += eps;   \
    res.field = (cost_fn(model) - cost) / eps;        \
    model.field = saved;

    Xor res;
    float cost = cost_fn(model);
    float saved;

    SAV_RES(or_w1);
    SAV_RES(or_w2);
    SAV_RES(or_b);
    SAV_RES(and_w1);
    SAV_RES(and_w2);
    SAV_RES(and_b);
    SAV_RES(nand_w1);
    SAV_RES(nand_w2);
    SAV_RES(nand_b);

    return res;
}

void learn(Xor *model, Xor gradient, float rate) {
    model->or_w1 	-= gradient.or_w1 * rate;
    model->or_w2 	-= gradient.or_w2 * rate;
    model->or_b 	-= gradient.or_b * rate;

    model->and_w1 	-= gradient.and_w1 * rate;
    model->and_w2 	-= gradient.and_w2 * rate;
    model->and_b 	-= gradient.and_b * rate;

    model->nand_w1 	-= gradient.nand_w1 * rate;
    model->nand_w2 	-= gradient.nand_w2 * rate;
    model->nand_b 	-= gradient.nand_b * rate;
}

int main(void) {
    Xor model = rand_xor();

    print_xor(&model);

    float eps = 1e-2;
    float learning_rate = 1e-1;

    for (size_t i = 0; i < 100*1000; i++) {
        Xor diff = finite_difference(model, eps);
        learn(&model, diff, learning_rate);
    }

    print_xor(&model);
    printf("Cost: %f\n", cost_fn(model));

    for (size_t i = 0; i < 2; i++) {
        for (size_t j = 0; j < 2; j++) {
            printf("Forward %zu ^ %zu = %f\n", i, j, forward(model, i, j));
        }
    }
}
