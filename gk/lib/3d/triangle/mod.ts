import { Vector } from '../vector/mod.js';
import type { Color } from '../../color.js';
import type { Mat4 } from '../matrix/mod.js';

export type ArrayParams = [
    aX: number, aY: number, aZ: number,
    bX: number, bY: number, bZ: number,
    cX: number, cY: number, cZ: number,
    color?: Color,
];

type NewParams = ConstructorParameters<typeof Triangle>;

export class Triangle {
    public static normal(a: Vector, b: Vector, c: Vector): Vector {
        const ba = b.clone().sub(a);
        const ca = c.clone().sub(a);
        const normal = ca.cross(ba).normalize();

        return normal;
    }

    public static new(...args: NewParams | ArrayParams): Triangle {
        if (args[0] instanceof Vector) {
            return new Triangle(...args as NewParams);
        }

        const n = args as ArrayParams;
        const a = Vector.new(n[0], n[1], n[2]);
        const b = Vector.new(n[3], n[4], n[5]);
        const c = Vector.new(n[6], n[7], n[8]);

        return new Triangle(a, b, c, n[9]);
    }

    public a: Vector;
    public b: Vector;
    public c: Vector;
    public normal: Vector;
    public color?: Color;

    public constructor(a: Vector, b: Vector, c: Vector, color?: Color) {
        this.a = a;
        this.b = b;
        this.c = c;
        this.normal = Triangle.normal(a, b, c);
        this.color = color;
    }

    public col(this: Triangle, color: Color): Triangle {
        this.color = color;
        
        return this;
    }

    public clone(this: Triangle): Triangle {
        return Triangle.new(
            this.a.clone(),
            this.b.clone(),
            this.c.clone(),
            this.color,
        );
    }

    public transform(this: Triangle, matrix: Mat4): Triangle {
        this.a.t(matrix);
        this.b.t(matrix);
        this.c.t(matrix);

        // TODO: Maybe transform normals as well?
        // Using inverse of the transpose of the matrix.
        this.normal = Triangle.normal(this.a, this.b, this.c);

        return this;
    }

    public center(this: Triangle): Vector {
        return Vector.new(
            (this.a.x + this.b.x + this.c.x) / 3,
            (this.a.y + this.b.y + this.c.y) / 3,
            (this.a.z + this.b.z + this.c.z) / 3,
        );
    }

    public toString(this: Triangle): string {
        return `A=${this.a} B=${this.b} C=${this.c}`;
    }
}
