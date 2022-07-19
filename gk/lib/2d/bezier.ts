import { Color } from '../color.js';
import type { Draw, ImageDataExt } from '../types.js';
import type { Point2 } from './point.js';

const step = 0.0025;
const fCache: Map<number, number> = new Map();
const ntCache: Map<number, Map<number, number>> = new Map();

function factorial(n: number): number {
    if (n === 0 || n === 1) {
        return 1;
    }

    if (fCache.has(n)) {
        return fCache.get(n)!;
    }

    const result = factorial(n - 1) * n;

    fCache.set(n, result);

    return result;
}

function nt(n: number, k: number): number {
    if (!(k >= 0 && n >= k)) {
        throw new Error(`Condition: ${n} >= ${k} >= 0`);
    }

    if (k === 0 || n === k) {
        return 1;
    }

    const result = factorial(n) / (factorial(k) * factorial(n - k));

    if (!ntCache.has(n)) {
        ntCache.set(n, new Map().set(k, result));
    } else {
        ntCache.get(n)!.set(k, result);
    }

    return result;
}

function bernstein(n: number, i: number, t: number): number {
    // https://pl.wikipedia.org/wiki/Wielomiany_Bernsteina
    return nt(n, i) * (t ** i) * ((1 - t) ** (n - i));
}

export function b4(p1: Point2, p2: Point2, p3: Point2, p4: Point2, image: ImageDataExt): ImageData {
    // http://www.algorytm.org/podstawy-grafiki/krzywa-beziera.html
    // 0 <= t <= 1
    let t = 0.0;

    while (t <= 1) {
        const d = (1 - t)
        const x = p1.x * (d ** 3)
            + 3 * p2.x * t * (d ** 2)
            + 3 * p3.x * (t ** 2) * d
            + p4.x * t ** 3;
        const y = p1.y * (d ** 3)
            + 3 * p2.y * t * (d ** 2)
            + 3 * p3.y * (t ** 2) * d
            + p4.y * t ** 3;

        image.drawPoint(Math.floor(x), Math.floor(y), new Color(0, 180, 55, 255));

        t += step;
    }

    return image;
}

export class Bezier2 implements Draw {
    public points: Point2[];
    public color: Color;

    public constructor(points: Point2[], color?: Color) {
        this.points = points;
        this.color = color || new Color(200, 200, 50, 255);
    }

    public draw(image: ImageDataExt): this {
        // https://byc-matematykiem.pl/krzywe-beziera/

        const { points, color } = this;

        if (points.length < 4) {
            throw new Error('To few points to draw the curve');
        }

        const n = points.length - 1;

        let t = 0.0;

        while (t <= 1.0) {
            let x = 0;
            let y = 0;

            for (let i = 0; i <= n; i++) {
                const p = points[i];

                x += bernstein(n, i, t) * p.x;
                y += bernstein(n, i, t) * p.y;
            }

            image.drawPoint(Math.floor(x), Math.floor(y), color);

            t += step;
        }

        return this;
    }
}

