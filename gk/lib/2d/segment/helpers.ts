import { p2 } from '../point.js';
import type { Segment2 } from '../segment.js';
import type { Point2 } from '../point.js';

/**
 * Point-Segment Position
 * Point position in relation to the line segment.
 *  0 - colinear.
 * -1 - left side of the segment
 *  1 - right side of the segment
 */
export enum PSPos {
    Left = -1,
    Colinear = 0,
    Right = 1,
};

/**
 * Returns a position of the vector represented by points (A, C)
 * in relation to vector represented by points (A, B).
 * u⃗ - A̅B̅ segment   
 * v⃗ - A̅C̅ segment
 */
export function pointSide(a: Point2, b: Point2, c: Point2): PSPos {
    const ux = b.x - a.x;
    const uy = b.y - a.y;
    const vx = c.x - a.x;
    const vy = c.y - a.y;
    const l = ux * vy;
    const r = uy * vx;

    if (l > r) {
        return PSPos.Left;
    } else if (l < r) {
        return PSPos.Right;
    }

    return PSPos.Colinear;
}

/**
 * Returns normal vector to the segment represented by points (A, B).
 * Depending on the value of the argument onLeft.
 */
export function normal(a: Point2, b: Point2, onLeft: boolean = false): Point2 {
    const dx = b.x - a.x;
    const dy = b.y - a.y;

    return onLeft ? p2(-dy, dx): p2(dy, -dx);
}

/**
 * Returns true if point C is on the same line as segment A̅B̅.
 */
export function colinear(segment: Segment2, c: Point2): boolean {
    const { start: a, end: b } = segment;

    return Object.is((b.x - a.x) * c.y, (b.y - a.y) * c.x);
}

/**
 * Returns true if the point C is on the segment A̅B̅.
 */
export function contains(segment: Segment2, c: Point2): boolean {
    if (c.eq(segment.start) || c.eq(segment.end)) {
        return true;
    }

    if (!colinear(segment, c)) {
        return false;
    }

    // Compare AB∙AB and AC∙AB
    const { start: a, end: b } = segment;
    const ab = p2(b.x - a.x, b.y - a.y);
    const ac = p2(c.x - a.x, c.y - a.y);

    const aTc = ac.x * ab.x + ac.y * ab.y;

    if (aTc < 0) {
        return false;
    }

    const aTb = ab.x * ab.x + ab.y * ab.y;
    return aTc <= aTb;
}
