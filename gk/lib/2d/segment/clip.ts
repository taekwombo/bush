import { p2 } from '../point.js';
import { nn } from '../../utils.js';
import { s2 } from '../segment.js';
import type { Segment2 } from '../segment.js';
import type { Range2 } from '../range.js';

/**
 * Sutherland-Cohen algorithm.
 *
 * 3.1.2. [resources/grafika_komputerowa.pdf]
 *
 * 1001│1000│1010
 * ────│────│────
 * 0001│0000│0010
 * ────│────│────
 * 0101│0100│0110
 */
export function SC(segment: Segment2, options: Range2): null | Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;

    const pointCode = (x: number, y: number): number => {
        if (yMin > y) {
            if (xMin > x) {
                return 0b1001;
            } else if (xMax < x) {
                return 0b1010;
            } else {
                return 0b1000;
            }
        } else if (yMax < y) {
            if (xMin > x) {
                return 0b0101;
            } else if (xMax < x) {
                return 0b0110;
            } else {
                return 0b0100;
            }
        } else {
            if (xMin > x) {
                return 0b0001;
            } else if (xMax < x) {
                return 0b0010;
            } else {
                return 0b0;
            }
        }
    };

    let { start: a, end: b } = segment;

    let c1 = pointCode(a.x, a.y);
    let c2 = pointCode(b.x, b.y);

    const hmin = s2(p2(xMin, yMin), p2(xMax, yMin));
    const hmax = s2(p2(xMin, yMax), p2(xMax, yMax));
    const vmin = s2(p2(xMin, yMin), p2(xMin, yMax));
    const vmax = s2(p2(xMax, yMin), p2(xMax, yMax));

    while (c1 | c2) {
        if (c1 & c2) {
            return null;
        }

        if (c1 === 0) {
            let t: any = c1;
            c1 = c2;
            c2 = t;
            t = a;
            a = b;
            b = t;
        }

        switch (c1) {
            case 0b0001:
            case 0b0101:
            case 0b1001: {
                a.set(nn(s2(a, b).intersection(vmin)));

                break;
            }

            case 0b1010:
            case 0b0010:
            case 0b0110: {
                a.set(nn(s2(a, b).intersection(vmax)));

                break;
            }

            case 0b0100: {
                a.set(nn(s2(a, b).intersection(hmax)));

                break;
            }

            case 0b1000: {
                a.set(nn(s2(a, b).intersection(hmin)));

                break;
            }

            default:
                throw new Error('Unreachable');
        }

        c1 = pointCode(a.x, a.y);
    }

    return segment;
}

/**
 * Liang-Barsky algorithm.
 *
 * 3.1.3. [resources/grafika_komputerowa.pdf]
 */
export function LB(segment: Segment2, options: Range2): null | Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { start: a, end: b } = segment;
    const dx = b.x - a.x;
    let t1 = 0;
    let t2 = 1;

    function test(p: number, q: number): boolean {
        let r: number;
        if (p < 0) {
            r = q / p;

            if (r > t2) {
                return false;
            } else if (r > t1) {
                t1 = r;
            }
        } else if (p > 0) {
            r = q / p;

            if (r < t1) {
                return false;
            } else if (r < t2) {
                t2 = r;
            }
        } else if (q < 0) {
            return false;
        }

        return true;
    }

    if (test(-dx, a.x - xMin)) {
        if (test(dx, xMax - a.x)) {
            const dy = b.y - a.y;
            if (test(-dy, a.y - yMin)) {
                if (test(dy, yMax - a.y)) {
                    if (t2 < 1) {
                        b.x = a.x + t2 * dx;
                        b.y = a.y + t2 * dy;
                    }
                    if (t1 > 0) {
                        a.x = a.x + t1 * dx;
                        a.y = a.y + t1 * dy;
                    }

                    return segment;
                }
            }
        }
    }

    return null;
}

/**
 * [resources/another_2d_line_clipping_method.pdf]
 *
 * Another Simple but Faster Method for 2D Line Clipping
 *
 * by Dimitrios Matthes and Vasileios Drakopoulos
 */
export function DMVD(segment: Segment2, options: Range2): Segment2 | null {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { start: a, end: b } = segment;

    // Step 1. Check for out of window segment.
    if (
        (a.x < xMin && b.x < xMin)      // line is left to the clipping window
        || (a.x > xMax && b.x > xMax)   // line is right to the clipping window
        || (a.y < yMin && b.y < yMin)   // line is over the clipping window
        || (a.y > yMax && b.y > yMax)   // line is under the clipping window
    ) {
        return null;
    }

    // Step 2. Correct point positions if needed.
    const x: [s: number, e: number] = [a.x, b.x];
    const y: [s: number, e: number] = [a.y, b.y];

    let i = 0;
    while (i < 2) {
        if (x[i] < xMin) {
            x[i] = xMin;
            y[i] = a.y + ((b.y - a.y) / (b.x - a.x)) * (xMin - a.x);
        } else if (x[i] > xMax) {
            x[i] = xMax;
            y[i] = a.y + ((b.y - a.y) / (b.x - a.x)) * (xMax - a.x);
        }

        if (y[i] < yMin) {
            y[i] = yMin;
            x[i] = a.x + ((b.x - a.x) / (b.y - a.y)) * (yMin - a.x);
        } else if (y[i] > yMax) {
            y[i] = yMax;
            x[i] = a.x + ((b.x - a.x) / (b.y - a.y)) * (yMax - a.x);
        }

        i++;
    }

    // Step 3. Check if new points are inside clip window.
    // If the points are outside return null.
    if ((x[0] < xMin && x[1] < xMin) || (x[0] > xMax && x[1] > xMax)) {
        return null;
    }

    a.set(p2(x[0], y[0]));
    b.set(p2(x[1], y[1]));

    return segment;
}
