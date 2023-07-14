#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>

float train[][3] = {
    {0, 0, 0},
    {1, 0, 0},
    {0, 1, 0},
    {1, 1, 1},
};

#define TRAIN_CNT ((sizeof train) / (sizeof train[0]))

float rand_float(void) {
    return (float) rand() / (float) RAND_MAX;
}

float sigmoidf(float x) {
    return 1.0f / (1.f + expf(-x));
}

void seed() {
    sranddev();
    rand_float();
}

/* AKA Loss function */
float cost_fn(float w1, float w2, float bias) {
    float result = 0;

    for (size_t i = 0; i < TRAIN_CNT; i++) {
        float input1 = train[i][0];
        float input2 = train[i][1];
        float output = sigmoidf(input1 * w1 + input2 * w2 + bias);
        float expected = train[i][2];
        float distance = output - expected;
        result += distance * distance;
    }

    return result / TRAIN_CNT;
}

int main(void) {
    seed();

    float w1 = rand_float();
    float w2 = rand_float();
    float bias = rand_float();
    float learning_rate = 1e-2;
    float eps = 1e-2;

    printf("%f -- %f \n", w1, w2);

    for (size_t i = 0; i < 100000; i++) {
        float cost = cost_fn(w1, w2, bias);
        printf("Cost: %f -- %f %f\n", cost, w1, w2);

        float dw1 = (cost_fn(w1 + eps, w2, bias) - cost) / eps;
        float dw2 = (cost_fn(w1, w2 + eps, bias) - cost) / eps;
        float dwb = (cost_fn(w1, w2, bias + eps) - cost) / eps;

        w1 -= dw1 * learning_rate;
        w2 -= dw2 * learning_rate;
        bias -= dwb * learning_rate;
    }

    for (size_t i = 0; i < 2; i++) {
        for (size_t j = 0; j < 2; j++) {
            printf("%zu | %zu = %f\n", i, j, sigmoidf(i * w1 + j * w2 + bias));
        }
    }

    printf("w1 = %f\nw2 = %f\nbias = %f\n", w1, w2, bias);
    printf("Cost: %f\n", cost_fn(w1, w2, bias));
}
