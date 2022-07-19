import { p2 } from '../point.js';
import { nn } from '../../utils.js';
import { s2 } from '../segment.js';
import type { Segment2 } from '../segment.js';
import type { Range2 } from '../../types.js';

/**
 * Sutherland-Cohen algorithm.
 *
 * 3.1.2. [pdfs/grafika_komputerowa.pdf]
 *
 * 1001│1000│1010
 * ────│────│────
 * 0001│0000│0010
 * ────│────│────
 * 0101│0100│0110
 */
export function clipSC(segment: Segment2, options: Range2): null | Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;

    const pointCode = (x: number, y: number): number => {
        if (xMin > y) {
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
        }

        return 0b0;
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
                a.x = nn(vmin.intersection(s2(a, b))).x;

                break;
            }

            case 0b1010:
            case 0b0010:
            case 0b0110: {
                a.x = nn(vmax.intersection(s2(a, b))).x;

                break;
            }

            case 0b0100: {
                a.y = nn(hmax.intersection(s2(a, b))).y;

                break;
            }

            case 0b1000: {
                a.y = nn(hmin.intersection(s2(a, b))).y;

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
 * 3.1.3. [pdfs/grafika_komputerowa.pdf]
 */
export function clipLB(line: Segment2, options: Range2): null | Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { start: a, end: b } = line;
    let t1 = 0;
    let t2 = 1;
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    function test(p: number, q: number): number {
        if (p < 0) {
            const r = q / p;

            if (r > t2) {
                return 0;
            } else if (r > t1) {
                t1 = r;
            }
        } else if (p > 0) {
            const r = q / p;

            if (r < t1) {
                return 0;
            } else if (r < t2) {
                t2 = r;
            }
        } else if (q < 0) {
            return 0;
        }

        return 1;
    }


    if (test(-dx, a.x - xMin)) {
        if (test(dx, xMax - a.x)) {
            if (test(-dy, a.y - yMin)) {
                if (test(dy, yMax - a.y)) {
                    if (t2 !== 0) {
                        b.x = t2 * dx;
                        b.y = t2 * dy;
                    }
                    if (t1 !== 0) {
                        a.x = t1 * dx;
                        a.y = t1 * dx;
                    }

                    return line;
                }
            }
        }
    }

    console.log('>> null');

    return null;
}
