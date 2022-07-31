import type { Vector } from '../vector/mod.js';

/**
 * point: p₀ = (x₁, y₁, z₁)
 * normal: n⃗ = (a, b, c)
 *
 * ### Equation
 * #### Parametric
 * (p - p₀) ∙ n⃗ = 0
 *
 * #### General
 * ax + by + cz + d = 0
 *
 * d = -(n⃗ ∙ p₀) = -(ax₀ + by₀ + cz₀)
 */
export class Plane {
    public static new(...p: ConstructorParameters<typeof Plane>): Plane {
        return new Plane(...p);
    }

    public point: Vector;
    public normal: Vector;

    public constructor(p: Vector, n: Vector) {
        this.point = p;
        this.normal = n;
    }
}
