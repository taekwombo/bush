import type { Vector } from '../vector/mod.js';

export class Triangle {
    public static normal(a: Vector, b: Vector, c: Vector): Vector {
        const ba = b.clone().sub(a);
        const ca = c.clone().sub(a);
        const normal = ca.cross(ba);

        return normal;
    }

    public static new(...args: ConstructorParameters<typeof Triangle>): Triangle {
        return new Triangle(...args);
    }

    public a: Vector;
    public b: Vector;
    public c: Vector;
    public n: Vector;

    public constructor(a: Vector, b: Vector, c: Vector) {
        this.a = a;
        this.b = b;
        this.c = c;
        this.n = Triangle.normal(a, b, c);
    }
}
