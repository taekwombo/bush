import type { Color } from '../color.js';
import type { Point2 } from './point.js';
import type { Debug, Draw, ImageDataExt } from '../types.js';

export class Circle2 implements Draw, Debug {
    public center: Point2;
    public radius: number;
    public color?: Color;

    public constructor(center: Point2, radius: number, color?: Color) {
        this.center = center;
        this.radius = radius;
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        drawCircle(image, this.center.x, this.center.y, this.radius, this.color);

        return this;
    }

    public debug(): string {
        return `O(${this.center.debug()}, r=${this.radius})`;
    }
}

export class Ellipsis2 implements Draw, Debug {
    public center: Point2;
    public a: number;
    public b: number;
    public color?: Color;

    public constructor(center: Point2, a: number, b: number, color?: Color) {
        this.center = center;
        this.a = a;
        this.b = b;
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        drawEllipsis(image, this.center.x, this.center.y, this.a, this.b, this.color);

        return this;
    }

    public debug(): string {
        return `E(${this.center.debug()}, a=${this.a}, b=${this.b})`;
    }
}

export function drawCircle(image: ImageDataExt, cx: number, cy: number, radius: number, color?: Color): ImageDataExt {
    // http://www.algorytm.org/podstawy-grafiki/kreslenie-okregow.html
    let x = 0;
    let y = radius;
    let decision = 5 - 4 * radius;

    while (x < y) {
        // Draw 8 points.
        image.drawPoint(cx + x,  cy + y, color);
        image.drawPoint(cx + -x, cy + y, color);
        image.drawPoint(cx + x,  cy + -y, color);
        image.drawPoint(cx + -x, cy + -y, color);
        image.drawPoint(cx + y,  cy + x, color);
        image.drawPoint(cx + -y, cy + x, color);
        image.drawPoint(cx + y,  cy + -x, color);
        image.drawPoint(cx + -y, cy + -x, color);

        if (decision >= 0) {
            decision += 8 * (x - y) + 20;
            y--;
            x++;
        } else {
            decision += 8 * x + 12;
            x++;
        }
    }

    return image;
}

export function drawEllipsis(
    image: ImageDataExt,
    cx: number,
    cy: number,
    radiusA: number,
    radiusB: number,
    color?: Color,
): ImageDataExt {
    // http://www.algorytm.org/podstawy-grafiki/kreslenie-elipsy.html

    let a = radiusA;
    let b = radiusB;
    let x = 0;
    let y = b;
    let decision = (4 * a**2) - (4 * a**2 * b) + a**2;
    let breakpoint = a**4 / (a**2 + b**2);

    while (x**2 < breakpoint) {
        image.drawPoint(cx + x,  cy + y, color);
        image.drawPoint(cx + x,  cy + -y, color);
        image.drawPoint(cx + -x, cy + y, color);
        image.drawPoint(cx + -x, cy + -y, color);

        if (decision >= 0) {
            decision += 8 * (b**2 * x) + 12 * b**2 - 8 * (a**2 * y) + 8 * a**2;
            y--;
            x++;
        } else {
            decision += 8 * (b**2 * x) + 12 * b**2;
            x++;
        }
    }

    a = radiusB;
    b = radiusA;
    x = 0;
    y = b;
    decision = (4 * a**2) - (4 * a**2 * b) + a**2;
    breakpoint = a**4 / (a**2 + b**2);

    while (x**2 < breakpoint) {
        image.drawPoint(cx + y,  cy + x, color);
        image.drawPoint(cx + -y, cy + x, color);
        image.drawPoint(cx + y,  cy + -x, color);
        image.drawPoint(cx + -y, cy + -x, color);

        if (decision >= 0) {
            decision += 8 * (b**2 * x) + 12 * b**2 - 8 * (a**2 * y) + 8 * a**2;
            y--;
            x++;
        } else {
            decision += 8 * (b**2 * x) + 12 * b**2;
            x++;
        }
    }

    return image;
}

