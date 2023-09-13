#include "raylib.h"
#define NERO_IMPLEMENTATION
#include "nero.h"
#define IMIGO_IMPLEMENTATION
#include "imigo.h"

void seed_rand() {
#ifdef RNDS
    srand(RNDS);
#else
    srand(time(NULL));
#endif
}

int main() {
    seed_rand();

    Mat test;
    png2mat("./8.png", &test);

    M_PRINT(test);
}
