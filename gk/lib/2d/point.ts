import { rad } from '../utils.js';
import type { Color } from '../color.js';
import type { Img } from '../img.js';
import type { Clone, Draw, Eq } from '../types.js';

export class Point2 implements Clone<Point2>, Draw, Eq {
    public static fromNDC(image: Img, { x, y }: Record<'x' | 'y', number>): Point2 {
        return new Point2(
            (x + 1) / 2 * image.image.width,
            (y + 1) / 2 * image.image.height,
        );
    }

    public static from({ x, y }: Record<'x' | 'y', number>): Point2 {
        return new Point2(x, y);
    }

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

    public draw(image: Img): this {
        image.drawPoint(this.x, this.y, this.color);

        return this;
    }

    public toString(): string {
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

    public round(this: Point2): Point2 { 
        this.x = Math.round(this.x);
        this.y = Math.round(this.y);

        return this;
    }

    public distance(this: Point2, other: Point2): number {
        return Math.sqrt(
            (other.x - this.x) ** 2 + (other.y - this.y) ** 2
        );
    }
    
    public rotate(this: Point2, angle: number, c?: Point2): Point2 {
        let { x, y } = this;
        if (c) {
            x -= c.x;
            y -= c.y;
        }

        const sin = Math.sin(rad(angle));
        const cos = Math.cos(rad(angle));

        const xp = Math.floor(x * cos - y * sin);
        const yp = Math.floor(x * sin + y * cos);

        this.x = xp;
        this.y = yp;

        if (c) {
            this.x += c.x;
            this.y += c.y;
        }

        return this;
    }
}

/**
 * An alias to Point2 class.
 */
export function p2 (...args: ConstructorParameters<typeof Point2>): Point2 {
    return new Point2(...args);
}

p2.from = Point2.from;

