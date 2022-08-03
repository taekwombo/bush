import type { Vector } from '../vector/mod.js';

type Row = [x: number, y: number, z: number, w: number];
type M4 = [Row, Row, Row, Row];

const EMPTY = (): M4 => [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

/**
 * Multiplying matrices.
 *
 * A[rows][cols]
 * Aₘₓₚ * Bₚₓₙ = Cₘₓₙ
 *
 * Column-major order: colums of the matrix describe base vectors.
 * Row-major order: rows of the matrix describe base vectors.
 *
 * In the code (in math sense) a row-major order will be used.
 * In the code (in programming sense) a row-major order will be used so that
 * data is stored from left to right and from top to bottom.
 *
 * ┌             ┐
 * │ x₀ y₀ z₀ w₀ │ - base vector of x axis
 * │ x₁ y₁ z₁ w₁ │ - base vector of y axis
 * │ x₂ y₂ z₂ w₂ │ - base vector of z axis
 * │ x₃ y₃ z₃ w₃ │ - base vector of w axis
 * └             ┘
 * Point-matrix multiplication in row-major order:
 *
 *               ┌             ┐
 *               │ x₀ y₀ z₀ w₀ │
 * ┌         ┐   │ x₁ y₁ z₁ w₁ │   ┌             ┐
 * │ x y z w │ * │ x₂ y₂ z₂ w₂ │ = │ x′ y′ z′ w′ │
 * └         ┘   │ x₃ y₃ z₃ w₃ │   └             ┘
 *               └             ┘
 *
 * x′ = x * x₀ + y * x₁ + z * x₂ + w * x₃
 * y′ = x * y₀ + y * y₁ + z * y₂ + w * y₃
 * z′ = x * z₀ + y * z₁ + z * z₂ + w * z₃
 * w′ = x * y₀ + y * w₁ + z * w₂ + w * w₃
 */

/**
 * Represents matrix in row-major order.
 * Data is stored row after row.
 */
export class Mat4 {
    public static identity(): Mat4 {
        return Mat4.new([
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1,
        ]);
    }

    public static translate(v: Vector): Mat4 {
        const { x, y, z } = v.clone();

        return Mat4.new([
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            x, y, z, 1,
        ]);
    }

    public static rotate(v: { x?: number; y?: number; z?: number }): Mat4 {
        let matrices = [];

        if (v.x) {
            const c = Math.cos(v.x);
            const s = Math.sin(v.x);

            matrices.push(Mat4.new([
                1, 0, 0, 0,
                0, c, s, 0,
                0, -s, c, 0,
                0, 0, 0, 1,
            ]));
        }
        
        if (v.y) {
            const c = Math.cos(v.y);
            const s = Math.sin(v.y);

            matrices.push(Mat4.new([
                c, 0, -s, 0,
                0, 1, 0, 0,
                s, 0, c, 0,
                0, 0, 0, 1,
            ]));
        }

        if (v.z) {
            const c = Math.cos(v.z);
            const s = Math.sin(v.z);

            matrices.push(Mat4.new([
                c, s, 0, 0,
                -s, c, 0, 0,
                0, 0, 1, 0,
                0, 0, 0, 1,
            ]));
        }

        if (matrices.length === 0) {
            throw new Error('Expected at least one angle');
        } else if (matrices.length === 1) {
            return matrices[0];
        }

        return matrices.reduce((acc, m) => acc.mul(m));
    }

    public static scale(v: number | Vector): Mat4 {
        let x, y, z;

        if (typeof v === 'number') {
            x = y = z = v;
        } else {
            x = v.x || 1;
            y = v.y || 1;
            z = v.z || 1;
        }

        return Mat4.new([
            x, 0, 0, 0,
            0, y, 0, 0,
            0, 0, z, 0,
            0, 0, 0, 1,
        ]);
    }

    public static new(...args: ConstructorParameters<typeof Mat4>): Mat4 {
        return new Mat4(...args);
    }

    /**
     * [
     *   [x₀ y₀ z₀ w₀]
     *   [x₁ y₁ z₁ w₁]
     *   [x₂ y₂ z₂ w₂]
     *   [x₃ y₃ z₃ w₃]
     * ]
     */
    public v: M4;

    public constructor(v: M4 | number[]) {
        if (v.length !== 4) {
            this.v = (v as number[]).reduce((m, v, i) => {
                const ri = Math.floor(i / 4);
                const ci = i % 4;

                m[ri][ci] = v;

                return m;
            }, EMPTY());
        } else {
            this.v = v as M4;
        }
    }

    public mul(this: Mat4, rhs: Mat4): Mat4 {
        const result = EMPTY();

        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                /**
                 * n-th row of the firs matrix: this.v[0]
                 * n-th col of the second matrix: [this.v[0][n], this.v[1][n], this.v[2][n], this.v[3][n]]
                 */
                result[i][j] = 
                    this.v[i][0] * rhs.v[0][j] +
                    this.v[i][1] * rhs.v[1][j] +
                    this.v[i][2] * rhs.v[2][j] +
                    this.v[i][3] * rhs.v[3][j];
            }
        }

        return Mat4.new(result);
    }

    public transpose(this: Mat4): Mat4 {
        const result = Mat4.new(EMPTY());

        for (let i = 0; i < 4; i++) {
            for (let j = 0; j < 4; j++) {
                result.v[i][j] = this.v[j][i];
            }
        }

        return result;
    }

    toString(this: Mat4): string {
        return `[[numbers]]`;
    }
}

