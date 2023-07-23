#define NERO_IMPLEMENTATION
#include "../nero.h"

int main(void) {
    sranddev();

    float data[] = {
        0, 0, 0,
        1, 0, 1,
        0, 1, 1,
        1, 1, 0
    };
    Mat train_data = (Mat) {
        .rows = 4,
        .cols = 3,
        .stride = 3,
        .els = data
    };

    Mat train_input = mat_view(train_data, 0, 0, 4, 2);
    Mat train_output = mat_view(train_data, 0, 2, 4, 1);

    size_t epochs = 50 * 1000;
    float rate = 1e-1;
    size_t arch[] = { 2, 2, 1 };

    Nero nero = nero_alloc(ARR_LEN(arch), arch);
    Nero grad = nero_alloc(ARR_LEN(arch), arch);
    nero_rand(nero, 0, 1);

    printf("Cost: %f\n", nero_cost(nero, train_input, train_output));
    for (size_t i = 0; i < epochs; i++) {
        nero_backprop(nero, grad, train_input, train_output);
        nero_learn(nero, grad, rate);
        printf("Cost: %f\n", nero_cost(nero, train_input, train_output));
    }

    for (size_t i = 0; i < 2; i++) {
        for (size_t j = 0; j < 2; j++) {
            M_AT(NERO_INPUT(nero), 0, 0) = (float)i;
            M_AT(NERO_INPUT(nero), 0, 1) = (float)j;
            nero_forward(nero);
            
            float y = M_AT(NERO_OUTPUT(nero), 0, 0);
            printf("%f ^ %f = %f\n", (float)i, (float)j, y);
        }
    }
}
