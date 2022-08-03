import { drawSegment } from './segment/draw.js';
import { extend } from './segment/extend.js';
import { r2 } from './range.js';
import * as intersection from './segment/intersection.js';
import * as clip from './segment/clip.js';
import * as helpers from './segment/helpers.js';
import type { Color } from '../color.js';
import type { Img } from '../img.js';
import type { Point2 } from './point.js';
import type { Range2 } from './range.js';
import type { Draw, Eq } from '../types.js';

export class Segment2 implements Eq, Draw {
    public static helpers = helpers;
    public static clip = clip;
    public static extend = extend;
    public static intersection = intersection;
    public static drawSegment = drawSegment;

    public static pipeDraw(image: Img, points: (Point2 | Point2[])[], color?: Color) {
        const options = r2([0, image.image.width], [0, image.image.height]);
        const ps = points.filter((p): p is Point2 => !Array.isArray(p));
        const arr = points.filter((p): p is Point2[] => Array.isArray(p));

        for (let i = 1; i < ps.length; i++) {
            const s = clip.DMVD(s2(ps[i - 1], ps[i], color), options);

            if (s) {
                drawSegment(image, s.start, s.end, color);
            }
        }

        for (const ps of arr) {
            for (let i = 1; i < ps.length; i++) {
                const s = clip.DMVD(s2(ps[i - 1], ps[i], color), options);

                if (s) {
                    drawSegment(image, s.start, s.end, color);
                }
            }
        }
    }

    public start: Point2;
    public end: Point2;
    /**
     * Color for the whole line segment.
     * In order to create different colors on each end of the
     * segment assign colors to the start and end points.
     */
    public color?: Color;

    public constructor(s: Point2, e: Point2, color?: Color) {
        if (s.eq(e)) {
            throw new Error('Segment expects two different points');
        }

        this.start = s;
        this.end = e;
        this.color = color;
    }

    public draw(img: Img): this {
        drawSegment(img, this.start, this.end, this.color);
        return this;
    }

    public eq(this: Segment2, rhs: Segment2): boolean {
        return this.start.eq(rhs.start) && this.end.eq(rhs.end);
    }

    public toString(this: Segment2): string {
        return `A̅B̅=(${this.start}, ${this.end})`;
    }

    public invert(this: Segment2): Segment2 {
        const t = this.start;
        this.start = this.end;
        this.end = t;

        return this;
    }

    public extend(this: Segment2, options: Range2): Segment2 {
        extend(this, options); 

        return this;
    }

    public intersection(this: Segment2, other: Segment2): Point2 | null {
        return intersection.def(this, other);
    }
}

/**
 * An alias to Segment2 class.
 */
export function s2(...params: ConstructorParameters<typeof Segment2>): Segment2 {
    return new Segment2(...params);
}

export { drawSegment };
