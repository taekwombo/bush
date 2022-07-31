import { num } from '../../utils.js';

export class Vector {
    public static new(...args: ConstructorParameters<typeof Vector>): Vector {
        return new Vector(...args);
    }

    public x: number;
    public y: number;
    public z: number;

    public constructor(x: number, y: number, z: number) {
        num(x, y, z);

        this.x = x;
        this.y = y;
        this.z = z;
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
        const len = this.mag();

        if (len !== 0) {
            this.mul(1.0 / len);
        }

        return this;
    }

    public toString(this: Vector): string {
        return `(${this.x}, ${this.y}, ${this.z})`;
    }
}

