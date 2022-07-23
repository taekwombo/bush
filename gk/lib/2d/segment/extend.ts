import { Line2 } from '../line.js';
import { p2 } from '../point.js';
import { nn } from '../../utils.js';
import { clipSlope } from '../line/clip.js';
import { LineType } from '../../types.js';
import type { Range2 } from '../range.js';
import type { Segment2 } from '../segment.js';

/** Mutates segment argument extending it to desired size. */
export function extend(segment: Segment2, options: Range2): Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const line = Line2.fromSegment(segment);

    const { start, end } = segment;

    // Vertical line can be extended to yMin and yMax
    // at the current x position.
    if (line.type === LineType.Vertical) {
        const topDown = start.y < end.y;

        start.y = topDown ? yMin : yMax;
        end.y = topDown ? yMax : yMin;

        return segment;
    }

    // Horizontal line can be extended to xMin and xMax
    // at the current y position.
    if (line.type === LineType.Horizontal) {
        const leftRight = start.x < end.x;

        start.x = leftRight ? xMin : xMax;
        end.x = leftRight ? xMax : xMin;

        return segment;
    }

    const { a, b } = nn(line.slope);

    const point1 = p2(0, b);
    const point2 = p2(xMax, xMax * a + b);

    if (point1.y < point2.y && start.y < end.y) {
        start.set(point1);
        end.set(point2);
    } else {
        start.set(point2);
        end.set(point1);
    }

    clipSlope(line, options);

    return segment;
}
