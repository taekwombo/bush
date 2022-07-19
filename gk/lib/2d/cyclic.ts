import type { Color } from '../color.js';
import type { Point2 } from './point.js';
import type { Draw, ImageDataExt } from '../types.js';

export class Cyclic implements Draw {
    public center: Point2;
    public trackRadius: number;
    public radius: number;
    public height: number;
    public inside: boolean;
    public color?: Color;
    protected maxTick: number;
    protected step: number;

    public constructor(c: Point2, tr: number, r: number, h: number, inside: boolean = false, color?: Color) {
        this.center = c;
        this.trackRadius = tr;
        this.radius = r;
        this.height = h;
        this.inside = inside;
        this.color = color;
        this.maxTick = 400;
        this.step = 0.05;
    }

    public draw(image: ImageDataExt): this {
        // http://www.algorytm.org/podstawy-grafiki/krzywe-cykliczne.html
        const { maxTick, step, center, inside, trackRadius, radius, height, color } = this;
        let tick = 0;

        const r = inside ? trackRadius + radius : trackRadius - radius;
        const sign = inside ? -1 : 1;

        while (tick <= maxTick) {
            const rt = r * tick;
            const x = r * Math.cos(tick) + (sign * height * Math.cos(rt / radius));
            const y = r * Math.sin(tick) - height * Math.sin(rt / radius);

            image.drawPoint(center.x + Math.floor(x), center.y + Math.floor(y), color);

            tick += step;
        }

        return this;
    }
}
