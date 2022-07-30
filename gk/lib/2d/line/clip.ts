import { nn } from '../../utils.js';
import { p2 } from '../point.js';
import { Segment2 } from '../segment.js';
import type { Line2 } from '../line.js';
import type { Point2 } from '../point.js';
import type { Range2 } from '../range.js';

/**
 * Mutates the "segment" property of line reducing it size to desired range.
 * This works for non-vertical and non-horizontal lines.
 */
export function clipSlope(line: Line2, options: Range2): Line2 {
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

/**
 * 3.1.4. [resources/grafika_komputerowa.pdf]
 * [resources/v_skala_line_and_line_segment_clipping.pdf]
 */
const TAB1 = [null, 0, 0, 1, 1, null, 0, 2, 2, 0, null, 1, 1, 0, 0, null];
const TAB2 = [null, 3, 1, 3, 2, null, 2, 3, 3, 2, null, 2, 3, 1, 3, null];
export function SKALA(line: Line2, options: Range2): Segment2 | null {
    const cross = (
        ax: number, ay: number, aw: number,
        bx: number, by: number, bw: number,
    ): [x: number, y: number, w: number] => [
        ay * bw - by * aw,
        aw * bx - bw * ax,
        ax * by - ay * bx,
    ];

    const dot = (
        ax: number, ay: number, aw: number,
        bx: number, by: number, bw: number,
    ): number => ax * bx + ay * by + aw * bw;

    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { a, b, c } = line.standard;

    const windowPoints = [
        p2(xMax, yMin), // X0
        p2(xMax, yMax), // X1
        p2(xMin, yMax), // X2
        p2(xMin, yMin), // X3
    ];
    // Vectors of edges (x, y, w)
    const edge: [number, number, number][] = [
        // e0 X0-X1
        [1, 0, -xMax],
        // e1 X1-X2
        [0, 1, -yMax],
        // e2 X2-X3
        [1, 0, -xMin],
        // e3 X3-X0
        [0, 1, -yMin],
    ];

    const p: [number, number, number] = [a, b, c];
    let ct: number = 0;

    for (let i = 0; i < 4; i++) {
        const point = windowPoints[i];
        if (dot(...p, point.x, point.y, 1) >= 0) {
            ct = (1 << i) | ct;
        }
    }

    const i = TAB1[ct];
    const j = TAB2[ct];

    if (i === null || j === null) {
        return null;
    }

    const pa = cross(...p, ...edge[i]);
    const pb = cross(...p, ...edge[j]);

    return new Segment2(
        p2(
            pa[0] / pa[2],
            pa[1] / pa[2],
        ),
        p2(
            pb[0] / pb[2],
            pb[1] / pb[2],
        ),
        line.color,
    );
}
