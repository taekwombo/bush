import { nz, num } from '../../utils.js';
import type { Mat4 } from '../matrix/mod.js';

export class Vector {
    public static new(...args: ConstructorParameters<typeof Vector>): Vector {
        return new Vector(...args);
    }

    public spherical(): Vector {
        // TODO: create a vector from spherical coordinates.
        // TODO: add vector method that will return spherical coordinates of a vector.
        throw new Error('Unimplemented');
    }

    public x: number;
    public y: number;
    public z: number;

    public constructor(x: number, y: number, z: number, w: number = 1.0) {
        num(x, y, z);
        nz(w);

        this.x = x;
        this.y = y;
        this.z = z;

        if (w !== 1) {
            this.x /= w;
            this.y /= w;
            this.z /= w;
        }
    }

    public add(this: Vector, rhs: Vector): Vector {
        this.x += rhs.x;
        this.y += rhs.y;
        this.z += rhs.z;

        return this;
    }

    public sub(this: Vector, rhs: Vector): Vector {
        this.x -= rhs.x;
        this.y -= rhs.y;
        this.z -= rhs.z;

        return this;
    }

    public clone(this: Vector): Vector {
        return Vector.new(this.x, this.y, this.z);
    }

    public dot(this: Vector, other: Vector): number {
        const dot = this.x * other.x + this.y * other.y + this.z * other.z;

        // Get rid of -0.
        return dot || 0;
    }

    public cross(this: Vector, other: Vector): Vector {
        return Vector.new(
            this.y * other.z - this.z * other.y,
            this.z * other.x - this.x * other.z,
            this.x * other.y - this.y * other.x,
        );
    }

    public mag(this: Vector, value?: number): number {
        if (!value) {
            return Math.sqrt(this.x ** 2 + this.y ** 2 + this.z ** 2);
        }

        this.normalize().mul(value);

        return value;
    }

    public mul(this: Vector, value: number): Vector {
        this.x *= value;
        this.y *= value;
        this.z *= value;

        return this;
    }

    public normalize(this: Vector): Vector {
        const mag = this.mag();

        if (mag !== 0) {
            this.mul(1.0 / mag);
        }

        return this;
    }

    public t(this: Vector, mat: Mat4): Vector {
        const { x, y, z } = this;
        const m = mat.v;
        
        const xp = x * m[0][0] + y * m[1][0] + z * m[2][0] + m[3][0];
        const yp = x * m[0][1] + y * m[1][1] + z * m[2][1] + m[3][1];
        const zp = x * m[0][2] + y * m[1][2] + z * m[2][2] + m[3][2];
        const w  = x * m[0][3] + y * m[1][3] + z * m[2][3] + m[3][3];

        this.x = xp / w;
        this.y = yp / w;
        this.z = zp / w;

        return this;
    }

    public toString(this: Vector): string {
        return `(${this.x}, ${this.y}, ${this.z})`;
    }
}

