import { rad } from '../../utils.js';
import type { Vector } from '../vector/mod.js';

export type Row = [x: number, y: number, z: number, w: number];
type M4 = [Row, Row, Row, Row];

type VLikeParam = Vector | Pick<Vector, 'x' | 'y' | 'z'> | [
    x: number,
    y: number,
    z: number,
];

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
    /** Scale, translate, rotate. */
    public static mul(input: Partial<Record<'translate' | 'scale' | 'rotate', Mat4>>): Mat4 {
        if (!input.translate && !input.rotate && !input.scale) {
            throw new Error('At least one matrix required');
        }

        let m = Mat4.identity();

        if (input.scale) {
            m = m.mul(input.scale);
        }

        if (input.translate) {
            m = m.mul(input.translate);
        }

        if (input.rotate) {
            m = m.mul(input.rotate);
        }

        return m;
    }

    public static identity(): Mat4 {
        return Mat4.new([
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1,
        ]);
    }

    /**
     * https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/opengl-perspective-projection-matrix
     * http://www.songho.ca/opengl/gl_projectionmatrix.html
     * https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_model_view_projection#model_transform
     * https://webglfundamentals.org/webgl/lessons/webgl-3d-perspective.html
     */
    public static perspective(aspect: number, near: number, far: number, fov: number): Mat4 {
        // Size of the visible near plane.
        const top = Math.tan(rad(fov / 2)) * near;
        const bottom = -top;
        const right = top * aspect;
        const left = -right;

        const x = (2 * near) / (right - left);
        const y = (2 * near) / (top - bottom);
        const zx = (right + left) / (right - left);
        const zy = (top + bottom) / (top - bottom);
        const zz = -((far + near) / (far - near));
        const wz = -((2 * far * near) / (far - near));

        return Mat4.new([
            x,  0,   0,  0,
            0,  y,   0,  0,
            zx, zy, zz, -1,
            0,  0,  wz,  0,
        ]);
    }

    public static translate(v: VLikeParam): Mat4 {
        let x, y, z;

        if (Array.isArray(v)) {
            x = v[0], y = v[1], z = v[2];
        } else {
            x = v.x, y = v.y, z = v.z;
        }

        return Mat4.new([
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            x, y, z, 1,
        ]);
    }

    public static rotate(v: VLikeParam): Mat4 {
        let matrices = [];

        let x, y, z;

        if (Array.isArray(v)) {
            x = v[0], y = v[1], z = v[2];
        } else {
            x = v.x, y = v.y, z = v.z;
        }

        if (x) {
            const c = Math.cos(x);
            const s = Math.sin(x);

            matrices.push(Mat4.new([
                1, 0, 0, 0,
                0, c, s, 0,
                0, -s, c, 0,
                0, 0, 0, 1,
            ]));
        }
        
        if (y) {
            const c = Math.cos(y);
            const s = Math.sin(y);

            matrices.push(Mat4.new([
                c, 0, -s, 0,
                0, 1, 0, 0,
                s, 0, c, 0,
                0, 0, 0, 1,
            ]));
        }

        if (z) {
            const c = Math.cos(z);
            const s = Math.sin(z);

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

    public static scale(v: number | VLikeParam): Mat4 {
        let x, y, z;

        if (Array.isArray(v)) {
            x = v[0], y = v[1], z = v[2];
        } else if (typeof v === 'number') {
            x = y = z = v;
        } else {
            x = v.x, y = v.y, z = v.z;
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

    public inverse(this: Mat4): Mat4 {
        // Shamelessly copied from: https://github.com/toji/gl-matrix/blob/master/src/mat4.js
        const [r0, r1, r2, r3] = this.v;
        
        let a00 = r0[0],
          a01 = r0[1],
          a02 = r0[2],
          a03 = r0[3];
        let a10 = r1[0],
          a11 = r1[1],
          a12 = r1[2],
          a13 = r1[3];
        let a20 = r2[0],
          a21 = r2[1],
          a22 = r2[2],
          a23 = r2[3];
        let a30 = r3[0],
          a31 = r3[1],
          a32 = r3[2],
          a33 = r3[3];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the determinant
        let det =
          b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if (!det) {
          throw new Error('no inverse');
        }

        det = 1.0 / det;

        const out = [];

        out[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        out[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        out[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        out[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
        out[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        out[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        out[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        out[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
        out[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
        out[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
        out[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
        out[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
        out[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
        out[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
        out[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
        out[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;

        return Mat4.new(out);
    }

    toString(this: Mat4): string {
        return `[[numbers]]`;
    }
}

