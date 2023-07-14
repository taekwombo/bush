#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define NN_IMPLEMENTATION
#include "../nn.h"

float train[][2] = {
    {0, 0},
    {1, 2},
    {2, 4},
    {3, 6},
    {4, 8},
    {5, 10}
};

#define TRAIN_CNT ((sizeof train) / (sizeof train[0]))

float rand_float(void) {
    return (float) rand() / (float) RAND_MAX;
}

void seed() {
    srand(100);
    rand_float();
}

/* AKA Loss function */
float cost(float weight, float bias) {
    float result = 0;

    for (size_t i = 0; i < TRAIN_CNT; i++) {
        float input = train[i][0];
        float output = input * weight + bias;
        float expected = train[i][1];
        float distance = output - expected;
        result += distance * distance;
    }

    return result / TRAIN_CNT;
}

int main(void) {
    seed();
    printf("Hello %f\n", rand_float());

    float weight = rand_float() * 10.0;
    float bias = rand_float() * 20.0;
    float learning_rate = 1e-3;
    float eps = 1e-3;

    for (size_t i = 0; i < 10000; i++) {
        float weight_dist = (cost(weight + eps, bias) - cost(weight, bias)) / eps;
        float bias_dist = (cost(weight, bias + eps) - cost(weight, bias)) / eps;
        printf("%f -- %f\n", bias_dist, bias_dist);
        weight -= weight_dist * learning_rate;
        bias -= bias_dist * learning_rate;
    }

    printf("%f\n", weight);
}
