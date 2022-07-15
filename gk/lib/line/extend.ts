import { equation, LineType } from './equation.js';
import { p2 } from '../point.js';
import { clip } from './clip.js';
import type { Line2 } from '../line.js';
import type { Range2 } from '../types.js';

export function extend(line: Line2, options: Range2): Line2 {
    line.assert2Point();

    const { yMin, yMax, xMax, xMin } = options;

    const e = equation(line);
    const [start, end] = line.points;

    // Vertical line can be extended to yMin and yMax
    // at the current x position.
    if (e.type === LineType.Vertical) {
        const topDown = start.y < end.y;

        start.y = topDown ? yMin : yMax;
        end.y = topDown ? yMax : yMin;

        return line;
    }

    // Horizontal line can be extended to xMin and xMax
    // at the current y position.
    if (e.type === LineType.Horizontal) {
        const leftRight = start.x < end.x;

        start.x = leftRight ? xMin : xMax;
        end.x = leftRight ? xMax : xMin;

        return line;
    }

    const { a, b } = e.general;
    const point1 = p2(0, b);
    const point2 = p2(xMax, xMax * a + b);

    if (point1.y < point2.y && start.y < end.y) {
        line.points[0] = point1;
        line.points[1] = point2;
    } else {
        line.points[0] = point2;
        line.points[1] = point1;
    }

    return clip(line, options, e.general);
}
