import type { Vector } from '../vector/mod.js';

/**
 * point: p₀ = (x₁, y₁, z₁)
 * direction: n⃗ = (a, b, c)
 *
 * ### Equation
 * #### Parametric
 * p = p₀ + tn⃗
 *
 * (x - x₁) / a = (y - y₁) / b = (z - z₁) / c
 *
 * #### Intersection of two planes
 * a₁x + b₁y + c₁z + d₁ = 0
 * a₂x + b₂y + c₂z + d₂ = 0
 */
export class Line {
    public static new(...args: ConstructorParameters<typeof Line>): Line {
        return new Line(...args);
    }

    public point: Vector;
    public direction: Vector;

    public constructor(p: Vector, d: Vector) {
        this.point = p;
        this.direction = d;
    }
}
