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
    Vector2 m   = p->momentum;

    if (v2zero(pos)) {
        float a_rad = p->angle * (PI / 180.0);
        pos = (Vector2){ sinf(a_rad) * p->radius, cosf(a_rad) * p->radius };
    }

    Vector2 g = { 0.0, p->mass * p->gravity };
    Vector2 f = v2scale(v2nor(pos), g.y);
    Vector2 acceleration = v2sub(g, f);

    float delta = (float)dt / 1000.0;
    pos = v2add(pos, v2scale(p->momentum, delta));

    p->position = v2scale(v2nor(pos), p->radius);
    p->momentum = v2add(m, v2scale(acceleration, delta));
}

void VecSolveSI(int64_t dt, Pendulum *p) {
    Vector2 pos = p->position;
    Vector2 m   = p->momentum;

    if (v2zero(pos)) {
        float a_rad = p->angle * (PI / 180.0);
        pos = (Vector2){ sinf(a_rad) * p->radius, cosf(a_rad) * p->radius };
    }

    Vector2 g = { 0.0, p->mass * p->gravity };
    Vector2 f = v2scale(v2nor(pos), g.y);
    Vector2 acceleration = v2sub(g, f);

    float delta = (float)dt / 1000.0;
    p->momentum = v2add(m, v2scale(acceleration, delta));

    pos = v2add(pos, v2scale(p->momentum, delta));
    p->position = v2scale(v2nor(pos), p->radius);
}

void AngleSolve(int64_t dt, Pendulum *p) {
    static float pi2 = 2 * PI;
    static float t = 0;
    t += (float) dt * pi2 / 1000.0;

    float T = pi2 * sqrtf(p->radius / p->gravity);
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
        .position = { 0.0, 0.0 },
        .momentum = { 0.0, 0.0 },
    };
}

void DrawPendulum(View *view) {
    BoundingBox2D box = view->box;

    DrawLine(box.x.y, box.y.x, box.x.y, box.y.y, WHITE);

    Vector2 center_top = {
        (box.x.y - box.x.x) * 0.5 + box.x.x,
        box.y.x,
    };
    Vector2 pos;

    if (view->kind != SOLVE_VEC) {
        float ar = view->p.angle * (PI / 180.0);
        float x = sinf(ar) * view->p.radius;
        float y = cosf(ar) * view->p.radius;

        pos = (Vector2) { x, y };
    } else {
        pos = view->p.position;
    }

    Vector2 g = { 0.0, view->p.mass * view->p.gravity };
    Vector2 r = v2scale(v2nor(pos), g.y);

    pos = v2add(pos, center_top);

    DrawLineV(center_top, pos, WHITE);
    DrawLineV(pos, v2add(pos, g), SKYBLUE);
    DrawLineV(pos, v2add(pos, r), GREEN);
    DrawLineV(pos, v2add(pos, v2sub(g, r)), RED);

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
    float angle = (float)a_angle * -1.0;
    float gravity = a_gravity;

    float box_w = fabs(1.6 * len * sinf(angle * (PI / 180.0)));
    Vector2 box_xo = { box_w, box_w };
    Vector2 box_x = { 0.0, box_w };
    Vector2 box_y = { 0.0, height };

    Pendulum template = NewPendulum(angle, weight, len, gravity);

    View views[3];
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

    while (!WindowShouldClose()) {
        int64_t dt = diff(&time);

        ClearBackground((Color){ 0, 0, 0, 1 });

        for (int i = 0; i < 3; i++) {
            views[i].solver(dt, &views[i].p);
            DrawPendulum(&views[i]);
        }

        EndDrawing();
    }

    CloseWindow();

    return 0;
}
