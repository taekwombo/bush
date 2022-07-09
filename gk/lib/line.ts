import type { Color } from './color.js';
import type { Point2 } from './point.js';
import type { Debug, Draw, ImageDataExt } from './types.js';

export class Line2 implements Draw, Debug {
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
}

export function drawLine(image: ImageDataExt, start: Point2, end: Point2, color?: Color): void {
    // http://www.algorytm.org/podstawy-grafiki/algorytm-bresenhama.html
    let deltaX = end.x - start.x;
    let deltaY = end.y - start.y;
    const stepX = deltaX < 0 ? -1 : 1; // Line is drawn from right to left then -1.
    const stepY = deltaY < 0 ? -1 : 1; // Line is drawn from top to bottom then -1.
    let { x: x0, y: y0 } = start;
    let { x: x1, y: y1 } = end;

    deltaX = Math.abs(Math.round(deltaX));
    deltaY = Math.abs(Math.round(deltaY));

    let decision: number = 0;

    // Drawing along X asis on each step.
    if (deltaX > deltaY) {
        decision = -deltaX;

        while (x0 !== x1) {
            image.drawPoint(x0, y0, color);
            decision += 2 * deltaY;

            if (decision > 0) {
                y0 += stepY;
                decision -= 2 * deltaX;
            }

            x0 += stepX;
        }
    } else {
        decision = -deltaY;

        while (y0 !== y1) {
            image.drawPoint(x0, y0, color);
            decision += 2 * deltaX;

            if (decision > 0) {
                x0 += stepX;
                decision -= 2 * deltaY;
            }

            y0 += stepY;
        }
    }
}
