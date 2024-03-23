// https://gafferongames.com/post/integration_basics/

#define VM_IMPL
#include "vm.h"

#include "raylib.h"
#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

int getUIntArg(const int argc, const char **argv, const char* name, unsigned int *value) {
    // Option: <name>=<val>
    size_t name_len = strlen(name);
    for (int i = 0; i < argc; i++) {
        size_t opt_len = strlen(argv[i]);

        if (opt_len < name_len) {
            continue;
        }

        if (strncmp(argv[i], name, name_len)) {
            continue;
        }

        const char *opt = argv[i] + name_len + /* = */1;
        int val = strtoumax(opt, NULL, 10);

        if (errno) {
            printf("Failed to parse %s into unsigned integer: errno %d\n", argv[i], errno);
            return 1;
        }

        *value = val;
        break;
    }

    return 0;
}

typedef struct {
    float mass;
    float radius;
    float angle;
    float angle_max;
    float gravity;
    float freq;
    float period;
    Vector2 position;
    Vector2 momentum;
} Pendulum;

typedef struct {
    Vector2 x;
    Vector2 y;
} BoundingBox2D;

typedef enum {
    SOLVE_VEC,
    SOLVE_ANG,
} SOLVE;

typedef void (*Solver)(int64_t dt, Pendulum *p);

typedef struct {
    Solver solver;
    Pendulum p;
    BoundingBox2D box;
    SOLVE kind;
} View;

void VecSolve(int64_t dt, Pendulum *p) {
    Vector2 pos = p->position;
    float angle = asinf(pos.x / p->radius) * (180.0 / PI);
    float a_rad = angle * (PI / 180.0);

    float delta = (float)dt / 1000.0;

    Vector2 gravity = { 0.0, p->gravity };
    Vector2 pull = v2rotate(v2scale(gravity, cosf(a_rad)), -angle);
    Vector2 acc_d = v2sub(gravity, pull);

    Vector2 right = v2nor(v2rotate(pull, 90));
    float mag_d = v2mag(acc_d);

    // Subtract if both have the same sign.
    if (pos.x < 0 && p->momentum.x < 0) {
        mag_d *= -1.0; 
    }
    if (pos.x > 0 && p->momentum.x > 0) {
        mag_d *= -1.0;
    }
    if (p->momentum.x > 0) {
        right = v2rotate(right, 180);
    }
    right = v2scale(right, v2mag(p->momentum) + mag_d);

    pos = v2add(pos, v2scale(p->momentum, delta));
    pos = v2scale(v2nor(pos), p->radius);

    p->momentum = right;
    p->position = pos;
}

void VecSolveSI(int64_t dt, Pendulum *p) {
    Vector2 pos = p->position;
    float angle = asinf(pos.x / p->radius) * (180.0 / PI);
    float a_rad = angle * (PI / 180.0);

    float delta = (float)dt / 1000.0;

    Vector2 gravity = { 0.0, p->gravity };
    Vector2 pull = v2rotate(v2scale(gravity, cosf(a_rad)), -angle);
    Vector2 acc_d = v2sub(gravity, pull);

    Vector2 right = v2nor(v2rotate(pull, 90));
    float mag_d = v2mag(acc_d);

    // Subtract if both have the same sign.
    if (pos.x < 0 && p->momentum.x < 0) {
        mag_d *= -1.0; 
    }
    if (pos.x > 0 && p->momentum.x > 0) {
        mag_d *= -1.0;
    }
    if (p->momentum.x > 0) {
        right = v2rotate(right, 180);
    }
    right = v2scale(right, v2mag(p->momentum) + mag_d);
    p->momentum = right;

    pos = v2add(pos, v2scale(p->momentum, delta));
    pos = v2scale(v2nor(pos), p->radius);

    p->position = pos;
}

/* Mathematical pendulum */
void AngleSolve(int64_t dt, Pendulum *p) {
    static float pi2 = 2.0 * PI;
    static float t = 0;
    t += (float) dt * pi2 / 1000.0;

    float T = 2.0 * PI * sqrtf(p->radius / p->gravity);
    float res = p->angle_max * cosf(t * (pi2 / T));

    p->angle = res;
}

Pendulum NewPendulum(float angle, float mass, float radius, float gravity) {
    return (Pendulum) {
        .mass = mass,
        .radius = radius,
        .angle = angle,
        .angle_max = angle,
        .gravity = gravity,
        .freq = sqrtf(gravity / radius),
        .period = 2 * PI * sqrtf(radius / gravity),
        .position = v2rotate(v2scale((Vector2){ 0.0, 1.0 }, radius), angle),
        .momentum = { 0.0, 1.0 },
    };
}

void DrawPendulum(View *view) {
    BoundingBox2D box = view->box;

    // DrawLine(box.x.y, box.y.x, box.x.y, box.y.y, WHITE);

    Vector2 center_top = {
        (box.x.y - box.x.x) * 0.5 + box.x.x,
        box.y.x,
    };

    Vector2 pos = view->p.position;

    if (view->kind == SOLVE_ANG) {
        float angle = view->p.angle * (PI / 180.0);
        pos.x = sinf(angle) * view->p.radius;
        pos.y = cosf(angle) * view->p.radius;
    }

    float angle = v2angle(pos);
    float ar = angle * (PI / 180.0);

    // Downwards - hypothenouse 
    Vector2 g = { 0.0, view->p.mass * view->p.gravity };
    // Adjacent
    Vector2 a = v2rotate(v2scale(g, cosf(ar)), -angle);
    // Opposite
    Vector2 o = v2sub(g, a);

    pos = v2add(pos, center_top);
    Vector2 right = v2add(pos, v2rotate(a, 90));
    {
        Vector2 end = v2add(pos, g);
        DrawLineV(pos, end, GREEN);
        DrawCircleV(end, 3.0, GREEN);
    }
    {
        Vector2 end = v2add(pos, a);
        DrawLineV(pos, end, RED);
        DrawCircleV(end, 3.0, RED);
    }
    {
        Vector2 end = v2add(pos, o);
        DrawLineV(pos, end, YELLOW);
        DrawCircleV(end, 3.0, YELLOW);
    }
    {
        DrawLineV(pos, right, BLUE);
        DrawCircleV(right, 3.0, BLUE);
    }
    {
        DrawLineV(pos, v2add(pos, view->p.momentum), ORANGE);
        DrawCircleV(v2add(pos, view->p.momentum), 3.0, ORANGE);
    }

    DrawLineV(center_top, pos, WHITE);
    Vector2 rest = { center_top.x, center_top.y + view->p.radius };
    DrawLineV(center_top, rest, YELLOW);
}

int64_t diff(struct timespec *start) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);

    int64_t millis = (now.tv_sec - start->tv_sec) * 1000;
    int64_t nanos = (now.tv_nsec - start->tv_nsec);
    int64_t result = millis + nanos / 1000000;

    *start = now;
    return result;
}

int main(const int argc, const char **argv) {
    char *title = "Integration";
    uint32_t width = 990,
             height = 480,
             fps = 30,
             a_len = 250,
             a_weight = 35,
             a_angle = 35,
             a_gravity = 10.0;

    if (getUIntArg(argc, argv, "--width", &width)
        || getUIntArg(argc, argv, "--height", &height)
        || getUIntArg(argc, argv, "--fps", &fps)
        || getUIntArg(argc, argv, "--len", &a_len)
        || getUIntArg(argc, argv, "--weight", &a_weight)
        || getUIntArg(argc, argv, "--angle", &a_angle)
        || getUIntArg(argc, argv, "--gravity", &a_gravity)
    ) {
        return 1;
    }

    float len = a_len;
    float weight = a_weight;
    float angle = (float)a_angle;
    float gravity = a_gravity;

    float box_w = fabs(1.6 * len * sinf(angle * (PI / 180.0)));
    Vector2 box_xo = { box_w, box_w };
    Vector2 box_x = v2add((Vector2){ 0.0, box_w }, box_xo);
    Vector2 box_y = { 0.0, height };

    Pendulum template = NewPendulum(angle, weight, len, gravity);

#define CNT 3
    View views[CNT];
        views[0].p = template;
        views[0].box = (BoundingBox2D) { .x = box_x, .y = box_y };
        views[0].solver = VecSolve;
        views[0].kind = SOLVE_VEC;

        views[1].p = template;
        views[1].box = (BoundingBox2D) { .x = v2add(box_x, box_xo), .y = box_y };
        views[1].solver = VecSolveSI;
        views[1].kind = SOLVE_VEC;

        views[2].p = template;
        views[2].box = (BoundingBox2D) { .x = v2add(box_x, v2scale(box_xo, 2.0)), .y = box_y };
        views[2].solver = AngleSolve;
        views[2].kind = SOLVE_ANG;

    SetConfigFlags(FLAG_MSAA_4X_HINT);
    SetTargetFPS(fps);
    InitWindow(width, height, title);

    struct timespec time;
    clock_gettime(CLOCK_MONOTONIC, &time);

    int drawing = 0;
    int key;

    while (!WindowShouldClose()) {
        int64_t dt = diff(&time);

        while ((key = GetKeyPressed())) {
            if (key == KEY_SPACE) {
                drawing = !drawing;
            }
            if (key == KEY_R) {
                for (int i = 0; i < CNT; i++) {
                    views[i].p = template;
                }
            }
        }

        ClearBackground((Color){ 0, 0, 0, 1 });

            for (int i = 0; i < CNT; i++) {
                if (drawing) {
                    views[i].solver(dt, &views[i].p);
                }
                DrawPendulum(&views[i]);
            }

        EndDrawing();
    }

    CloseWindow();

    return 0;
}
