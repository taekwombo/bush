import { nn } from '../../utils.js';
import { Line2 } from '../line.js';
import type { Point2 } from '../point.js';
import type { Range2 } from '../../types.js';

/**
 * Mutates the "segment" property of line reducing it size to desired range.
 * This works for non-vertical and non-horizontal lines.
 */
export function clip(line: Line2, options: Range2): Line2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { a, b } = nn(line.slope);
    const { start, end } = line.segment;

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

