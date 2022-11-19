import { Color } from '../color.js';
import { Segment2 } from './segment.js';
import type { Img } from '../img.js';
import type { Draw } from '../types.js';
import type { Point2 } from './point.js';

export namespace Bspline2 {
    export type Options = {
        points: Point2[];
        clamp?: { start: boolean; end: boolean };
        degree?: number;
        color?: Color;
    };
}

/**
 * 6.2.1. [resources/grafika_komputerowa.pdf]
 *
 * https://en.wikipedia.org/wiki/De_Boor%27s_algorithm
 * http://www.idav.ucdavis.edu/education/CAGDNotes/Deboor-Cox-Calculation/Deboor-Cox-Calculation.html
 * https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/spline/B-spline/bspline-basis.html
 */
export class Bspline2 implements Draw {
    public static new(...params: ConstructorParameters<typeof Bspline2>): Bspline2 {
        return new Bspline2(...params);
    }

    public points: Point2[];
    public color: Color;
    public degree: number;
    public knots: number[];
    private clamp: { start: boolean; end: boolean };

    public constructor({ points, color, degree, clamp }: Bspline2.Options) {
        this.points = points;
        this.color = color || Color.Red;
        this.degree = degree || points.length - 1;
        this.clamp = {
            start: clamp?.start ?? true,
            end: clamp?.end ?? true,
        };
        this.knots = this.createKnotVector();
    }

    public update(this: Bspline2, options: Partial<Bspline2.Options>): Bspline2 {
        for (const key of Object.keys(options) as (keyof Bspline2.Options)[]) {
            // @ts-expect-error
            this[key] = options[key];
        }

        this.knots = this.createKnotVector();

        return this;
    }

    public createKnotVector(this: Bspline2): number[] {
        const clampPad = this.degree + 1;
        const knotCount = this.points.length + clampPad;
        const knotVector: number[] = new Array(knotCount).fill(0).map((_, i) => {
            return i * 2 >= (knotCount - 1) ? 1 : 0;
        });


        if (this.clamp.start) {
            for (let i = 0; i < this.degree + 1; i++) {
                knotVector[i] = 0;
            }
        }
        if (this.clamp.end) {
            for (let i = 0; i < this.degree + 1; i++) {
                knotVector[knotVector.length - 1 - i] = 1;
            }
        }

        const mid = knotCount - (this.degree + 1) * 2;

        if (mid) {
            const max = knotCount - clampPad;
            for (let i = clampPad; i < max; i++) {
                const val = (i - clampPad + 1) / (mid + 1);

                knotVector[i] = val;
            }
        }

        return knotVector;
    }

    public draw(image: Img): this {
        const step = 0.0002;
        const { knots, points } = this;

        // Draw connected control points.
        Segment2.pipeDraw(image, points, Color.Green);

        for (let x = 0; x <= 1; x += step) {
            const k = knots.findIndex((e, i, arr) => {
                return x >= e && arr[i + 1] > x;
            });

            this.calculate(k, x).round().draw(image);
        }

        return this;
    }

    // https://en.wikipedia.org/wiki/De_Boor%27s_algorithm
    public calculate(k: number, x: number): Point2 {
        const { knots: t, points: c, degree: p } = this;
        const d: Point2[] = [];

        for (let j = 0; j < p + 1; j++) {
            d.push(c[j + k - p].clone());
        }

        for (let r = 1; r < p + 1; r++) {
            for (let j = p; j > r - 1; j--) {
                const a = t[j + k - p];
                const b = t[j + 1 + k - r];
                const c = t[j + k - p];

                const alpha = (x - a) / (b - c);

                if (Number.isNaN(alpha)) {
                    // debugger;
                }

                const px = (1 - alpha) * d[j - 1].x + alpha * d[j].x;
                const py = (1 - alpha) * d[j - 1].y + alpha * d[j].y;

                d[j].x = px;
                d[j].y = py;
            }
        }

        d[p].color = this.color;

        return d[p];
    }
}
