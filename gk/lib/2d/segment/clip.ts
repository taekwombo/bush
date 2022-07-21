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
                a.set(nn(vmin.intersection(s2(a, b))));

                break;
            }

            case 0b1010:
            case 0b0010:
            case 0b0110: {
                a.set(nn(vmax.intersection(s2(a, b))));

                break;
            }

            case 0b0100: {
                a.set(nn(hmax.intersection(s2(a, b))));

                break;
            }

            case 0b1000: {
                a.set(nn(hmin.intersection(s2(a, b))));

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
export function clipLB(segment: Segment2, options: Range2): null | Segment2 {
    const { x: [xMin, xMax], y: [yMin, yMax] } = options;
    const { start: a, end: b } = segment;

    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const pi = [-dx, dx, -dy, dy];
    const qi = [a.x - xMin, xMax - a.x, a.y - yMin, yMax - a.y];

    const rs: number[] = [];

    for (let i = 0; i < pi.length; i++) {
        const p = pi[i];
        const q = qi[i];

        // Parallel to the edge and outside of the clip window.
        if (p === 0 && q < 0) {
            return null;
        }

        if (p !== 0) {
            rs.push(q / p);
        }
    }

    const t1 = Math.max(0, ...rs);
    const t2 = Math.min(1, ...rs);

    console.log({ t1, t2, pi, qi, a: a.debug(), b: b.debug() });

    if (t2 !== 1) {
        b.x += t2 * dx;
        b.y += t2 * dy;
    }
    if (t1 !== 0) {
        a.x += t1 * dx;
        a.y += t1 * dy;
    }
    console.log({ t1, t2, pi, qi, a: a.debug(), b: b.debug() });

    return segment;
}
