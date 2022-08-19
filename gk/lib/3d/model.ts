import { Triangle } from './triangle/mod.js';
import { Vector } from './vector/mod.js';
import type { Mat4 } from './matrix/mod.js';
import type { ArrayParams } from './triangle/mod.js';

type NewParams = Triangle[] | ArrayParams[];

export class Model {
    public static cube(): Model {
        const triangles = cubeVertices.map((v) => Triangle.new(...v));

        return new Model(triangles);
    }

    public static new(...args: NewParams): Model {
        if (args[0] instanceof Triangle) {
            return new Model(args as Triangle[]);
        }

        const triangles = (args as ArrayParams[])
            .map((t) => Triangle.new(...t));

        return new Model(triangles);
    }

    public triangles: Triangle[];

    public constructor(f: Triangle[]) {
        this.triangles = f;
    }

    public clone(this: Model): Model {
        const triangles = this.triangles.map((t) => t.clone());

        return Model.new(...triangles);
    }

    public transform(this: Model, matrix: Mat4): Model {
        for (const t of this.triangles) {
            t.transform(matrix);
        }

        return this;
    }

    public getTrianglesToDraw(this: Model): Triangle[] {
        return this.triangles
            // Discard triangles facing away from the camera. It is assumed that
            // camera is at (0, 0, 0).
            .filter((t) => t.normal.dot(Vector.new(0, 0, 0).sub(t.a)) >= 0)
            // Sort triangles by mean Z coordinate in Camera space.
            // So the smaller the Z - the further away it is from the camera.
            .map((t) => [t, (t.a.z + t.b.z + t.c.z) / 3] as const)
            .sort((a, b) => a[1] - b[1])
            .map(([t]) => t);
    }
}

/**
 * Basically the same as World coordinates.
 * Except the model takes all the space.
 *
 * X -1 ←─────────────→ 1
 * Y -1 ←─────────────→ 1
 * Z -1 ←─────────────→ 1
 *
 *    UP       FAR (visible)
 *    Y 1      Z 1
 *    ↑        ↑
 *    │        │
 *    │        │    LEFT                  RIGHT
 *    │        │    X -1 ←──────────────→ X 1
 *    │        │
 *    │        │
 *    ↓        ↓
 *    Y -1     Z -1
 *    DOWN     BEHIND (hidden)
 *
 * Behind is anything that is behind the origin along
 * the Z axis if youre looking at positive Z direction.
 */
const cubeVertices = [
    // far face
    [
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
         1.0,  1.0, -1.0,
    ],
    [
        -1.0, -1.0, -1.0,
         1.0,  1.0, -1.0,
         1.0, -1.0, -1.0,
    ],

    // right face
    [
         1.0, -1.0, -1.0,
         1.0,  1.0, -1.0,
         1.0,  1.0,  1.0,
    ],
    [
         1.0, -1.0, -1.0,
         1.0,  1.0,  1.0,
         1.0, -1.0,  1.0,
    ],

    // behind face
    [
         1.0, -1.0,  1.0,
         1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0
    ],
    [
         1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,
    ],

    // left face
    [
        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,
    ],
    [
        -1.0, -1.0,  1.0,
        -1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,
    ],

    // top face
    [
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
    ],
    [
        -1.0,  1.0, -1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0, -1.0 
    ],

    // bottom face
    [
         1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,
    ],
    [
         1.0, -1.0 , 1.0,
        -1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
    ],
] as const;
