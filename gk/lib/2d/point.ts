import type { Color } from '../color.js';
import { Clone, Debug, Draw, Eq, ImageDataExt } from '../types.js';

export class Point2 implements Clone<Point2>, Draw, Debug, Eq {
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

    public eq(this: Point2, rhs: Point2): boolean {
        return this.x === rhs.x && this.y === rhs.y;
    }

    public set(this: Point2, { x, y, color }: Point2): Point2 {
        this.x = x;
        this.y = y;
        this.color = color;

        return this;
    }

    public clone(this: Point2): Point2 {
        return new Point2(this.x, this.y, this.color);
    }
}

/**
 * An alias to Point2 class.
 */
export function p2 (...args: ConstructorParameters<typeof Point2>): Point2 {
    return new Point2(...args);
}

