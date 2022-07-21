import { drawSegment } from './segment/draw.js';
import { ovl } from '../utils.js';
import { extend } from './segment/extend.js';
import { intersection } from './segment/intersection.js';
import type { Color } from '../color.js';
import type { Point2 } from './point.js';
import type { Range2, Draw, Debug, Eq, ImageDataExt } from '../types.js';

export { clipSC, clipLB } from './segment/clip.js';

export class Segment2 implements Eq, Draw, Debug {
    public static extend = extend;
    public static intersection = intersection;
    public static drawSegment = drawSegment;

    public static pipeDraw(image: ImageDataExt, points: Point2[], color?: Color) {
        for (let i = 1; i < points.length; i++) {
            drawSegment(image, points[i - 1], points[i], color);
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

    public draw(img: ImageDataExt): this {
        drawSegment(img, this.start, this.end, this.color);
        return this;
    }

    public eq(this: Segment2, rhs: Segment2): boolean {
        return this.start.eq(rhs.start) && this.end.eq(rhs.end);
    }

    public debug(this: Segment2): string {
        return `${ovl('AB')} A=${this.start.debug()} B=${this.end.debug()}`;
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
        return intersection(this, other);
    }
}

/**
 * An alias to Segment2 class.
 */
export function s2(...params: ConstructorParameters<typeof Segment2>): Segment2 {
    return new Segment2(...params);
}

export { drawSegment };
