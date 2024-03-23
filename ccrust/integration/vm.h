#ifndef VM
#define VM

#include "raylib.h"
#include <math.h>
#include <stdio.h>

Vector2 v2add(Vector2 a, Vector2 b);
Vector2 v2sub(Vector2 a, Vector2 b);
Vector2 v2neg(Vector2 a);
Vector2 v2nor(Vector2 a);
Vector2 v2scale(Vector2 a, float scale);
int v2zero(Vector2 a);
float v2mag(Vector2 a);
void v2print(const char *name, Vector2 a);

#endif

#ifdef VM_IMPL

Vector2 v2add(Vector2 a, Vector2 b) {
    return (Vector2) { a.x + b.x, a.y + b.y };
}

Vector2 v2sub(Vector2 a, Vector2 b) {
    return (Vector2) { a.x - b.x, a.y - b.y };
}

Vector2 v2neg(Vector2 a) {
    return (Vector2) { -a.x, -a.y };
}

Vector2 v2scale(Vector2 a, float scale) {
    return (Vector2) { a.x * scale, a.y * scale };
}

Vector2 v2nor(Vector2 a) {
    float mag = v2mag(a);
    return (Vector2) { a.x / mag, a.y / mag };
}

int v2zero(Vector2 a) {
    return a.x == 0.0 && a.y == 0.0;
}

float v2mag(Vector2 a) {
    return sqrtf(
        powf(fabsf(a.x), 2.0) + powf(fabsf(a.y), 2.0)
    );
}

void v2print(const char *name, Vector2 a) {
    printf("%s:(%f, %f)\n", name, a.x, a.y);
}

#endif
