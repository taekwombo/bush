import { Color } from './color.js';
import { drawLine } from './line/draw.js';
import { clip } from './line/clip.js';
import { extend } from './line/extend.js';
import { equation } from './line/equation.js';
import * as int from './line/intersection.js';
import type { Point2 } from './point.js';
import type { Debug, Draw, ImageDataExt } from './types.js';

export class Line2 implements Draw, Debug {
    public static clip = clip;
    public static extend = extend;
    public static drawLine = drawLine;
    public static equation = equation;
    public static int = int;

    public points: Point2[];
    public color?: Color;

    public constructor(points: Point2[], color?: Color) {
        this.points = points;
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        const { color } = this;

        for (let i = 1; i < this.points.length; i++) {
            const s = this.points[i - 1];
            const e = this.points[i];

            drawLine(image, s, e, color);
        }

        return this;
    }

    public debug(): string {
        return this.points.map((p) => p.debug()).join(' - ');
    }

    public invert(this: Line2): Line2 {
        this.points.reverse();

        return this;
    }

    public assert2Point(this: Line2) {
        if (this.points.length !== 2) {
            throw new Error(`Expected line with 2 points, got ${this.points.length}`);
        }
    }
}
