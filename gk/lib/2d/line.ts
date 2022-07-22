import { Color } from '../color.js';
import { equation } from './line/equation.js';
import * as clip from './line/clip.js';
import * as intersection from './line/intersection.js';
import { LineType } from '../types.js';
import type { Slope, Standard } from './line/equation.js';
import type { Segment2 } from './segment.js';
import type { Debug, Draw, ImageDataExt } from '../types.js';

export class Line2 implements Draw, Debug {
    public static clip = clip;
    public static intersection = intersection;

    public static fromSegment(seg: Segment2, color?: Color): Line2 {
        const eq = equation(seg);

        return new Line2(
            eq.type,
            eq.standard,
            eq.slope,
            eq.segment,
            color,
        );
    }

    public type: LineType;
    public standard: Standard;
    public slope: null | Slope;
    public color?: Color;
    public segment: Segment2;

    public constructor(t: LineType, st: Standard, sl: null | Slope, seg: Segment2, color?: Color) {
        this.type = t;
        this.standard = st;
        this.slope = sl;
        this.color = color;
        this.segment = seg;
    }

    public draw(image: ImageDataExt): this {
        const { width, height } = image;

        this.segment.extend({
            x: [0, width],
            y: [0, height],
        }).draw(image);

        return this;
    }

    public debug(): string {
        return 'todo';
    }
}
