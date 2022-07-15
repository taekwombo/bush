import type { Line2 } from '../line.js';
import type { Point2 } from '../point.js';
import type { Range2 } from '../types.js';

export function clip(line: Line2, options: Range2, eq: { a: number; b: number }): Line2 {
    line.assert2Point();

    const { yMin, yMax, xMin, xMax } = options;
    const { a, b } = eq;
    const [start, end] = line.points;

    function cp(point: Point2): void {
        if (point.y < yMin) {
            point.y = yMin;
            point.x = (yMin - b) / a;
        } else if (point.y > yMax) {
            point.y = yMax;
            point.x = (yMax - b) / a;
        }

        if (point.x < xMin) {
            point.x = xMin;
            point.y = a * xMin + b;
        } else if (point.x > xMax) {
            point.x = xMax;
            point.y = a * xMax + b;
        }

        point.x = Math.round(point.x);
        point.y = Math.round(point.y);
    }

    cp(start);
    cp(end);

    return line;
}
