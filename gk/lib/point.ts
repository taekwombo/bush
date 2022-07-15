import type { Color } from './color.js';
import type { Debug, Draw, ImageDataExt } from './types.js';

export class Point2 implements Draw, Debug {
    public x: number;
    public y: number;
    public color?: Color;

    public constructor(x: number, y: number, color?: Color) {
        if (Number.isNaN(x) || Number.isNaN(y) || Math.abs(x) === Infinity || Math.abs(y) === Infinity) {
            throw new Error(`Invalid arguments x=${x} y=${y}`);
        }

        this.x = Math.round(x);
        this.y = Math.round(y);
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        image.drawPoint(this.x, this.y, this.color);

        return this;
    }

    public debug(): string {
        return `(${this.x}, ${this.y})`;
    }
}
export function p2 (...args: ConstructorParameters<typeof Point2>): Point2 {
    return new Point2(...args);
}

