import { p2 } from '../point.js';
import type { Segment2 } from '../segment.js';
import type { Point2 } from '../point.js';

/**
 * Finds an intersection of a two segments seg1 and seg2.
 *
 * https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
 */
export function def(seg1: Segment2, seg2: Segment2): null | Point2 {
    const { start: a, end: b } = seg1;
    const { start: c, end: d } = seg2;

    const num = (a.x - c.x) * (c.y - d.y) - (a.y - c.y) * (c.x - d.x);
    const den = (a.x - b.x) * (c.y - d.y) - (a.y - b.y) * (c.x - d.x);

    if (den === 0) {
        return null;
    }

    const t = num / den;

    if (t > 1 || t < 0) {
        return null;
    }

    function minMax(p1: Point2, p2: Point2, axis: 'x' | 'y'): [min: number, max: number] {
        const v1 = p1[axis], v2 = p2[axis];

        return v1 > v2 ? [v2, v1] : [v1, v2];
    }

    const [xMin1, xMax1] = minMax(a, b, 'x');
    const [yMin1, yMax1] = minMax(a, b, 'y');
    const [xMin2, xMax2] = minMax(c, d, 'x');
    const [yMin2, yMax2] = minMax(c, d, 'y');

    let intersects: boolean = false;

    // If bounding box of the first segments includes the bounding box of the second segment
    // or if the second one includes the first one then there is an intersection on the segments.
    if (xMin1 <= xMin2 && xMax1 >= xMax2 && yMin1 <= yMin2 && yMax1 >= yMax2) {
        intersects = true;
    } else if (xMin2 <= xMin1 && xMax2 >= xMax1 && yMin2 <= yMin1 && yMax2 >= yMax1) {
        intersects = true;
    }

    if (!intersects) {
        const u = (
            (a.x - c.x) * (a.y - b.y) - (a.y - c.y) * (a.x - b.x)
        ) / (
            (a.x - b.x) * (c.y - d.y) - (a.y - b.y) * (c.x - d.x)
        );

        if (u <= 1 && u >= 0) {
            intersects = true;
        }
    }

    if (!intersects) {
        return null;
    }

    const x = a.x + (b.x - a.x) * t;
    const y = a.y + (b.y - a.y) * t;

    return p2(x, y);
}

/** https://stackoverflow.com/a/565282 */
export function cross(s1: Segment2, s2: Segment2): Point2 | null {
    const { start: a, end: b } = s1;
    const { start: c, end: d } = s2;

    const dx0 = b.x - a.x;
    const dy0 = b.y - a.y;
    const dx1 = d.x - c.x;
    const dy1 = d.y - c.y;

    // s = (-dy0 * (a.x - c.x) + dx0 * (a.y - c.y)) / (-dx1 * dy0 + dx0 * dy1);
    // t = ( dx1 * (a.y - c.y) - dy1 * (a.x - c.x)) / (-dx1 * dy0 + dx0 * dy1);

    const dn = (-dx1 * dy0 + dx0 * dy1);
    const s = (-dy0 * (a.x - c.x) + dx0 * (a.y - c.y)) / dn;
    const t = (dx1 * (a.y - c.y) - dy1 * (a.x - c.x)) / dn;

    if (s >= 0 && s <= 1 && t >= 0 && t <= 1) {
        return p2(
            a.x + t * dx0,
            a.y + t * dy0,
        );
    }

    return null;
}
