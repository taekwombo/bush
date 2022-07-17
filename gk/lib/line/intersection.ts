import { p2 } from '../point.js';
import { equation, LineType } from './equation.js';
import { Line2 } from '../line.js';
import type { General } from './equation.js';
import type { Point2 } from '../point.js';

/**
 * Finds an intersection of a line "l1" and a segment "l2".
 *
 * 3.1.1. [pdfs/grafika_komputerowa.pdf]
 * 
 * Intersection between Segment (P₁, P₂) and a line (ax + by = c).
 * P₁ = (x₁, y₁)
 * P₂ = (x₂, y₂)
 * 
 * |x|   |x₂|     |x₁ - x₁|
 * | | = |  | + t |       |  for t ∈ [0, 1]
 * |y|   |y₁|     |y₂ - y₂|
 *
 *         c - ax₁ - by₁
 * t = ───────────────────────
 *     a(x₂ - x₁) + b(y₂ - y₁)
 */
export function standard(l1: Line2, l2: Line2): null | Point2 {

    const { standard: { a, b, c } } = equation(l1);
    const [s, e] = l2.points;

    const t = (-c - a * s.x - b * s.y) / (a * (e.x - s.x) + b * (e.y - s.y));

    if (t <= 1 && t >= 0) {
        return p2(
            s.x + t * (e.x - s.x),
            s.y + t * (e.y - s.y),
        );
    }

    return null;
}

/**
 * Finds an intersection of a segments "l1" and "l2".
 *
 * https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
 */
export function segments(l1: Line2, l2: Line2): null | Point2 {
    const [a, b] = l1.points;
    const [c, d] = l2.points;

    const t = (
        (a.x - c.x) * (c.y - d.y) - (a.y - c.y) * (c.x - d.x)
    ) / (
        (a.x - b.x) * (c.y - d.y) - (a.y - b.y) * (c.x - d.x)
    );

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

/**
 * Finds an intersection of two lines passing through "l1" and "b" segments.
 *
 * https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
 */
export function lines(l1: Line2, l2: Line2): null | Point2 {
    l2.assert2Point();

    const eq1 = equation(l1);
    const eq2 = equation(l2);

    if (eq1.type !== LineType.Regular && eq1.type === eq2.type) {
        // May be 0 or Infinity intersection points.
        return null;
    }

    let x: number | undefined;
    let y: number | undefined;

    if (eq1.type === LineType.Vertical) {
        x = l1.points[0].x;
    } else if (eq1.type === LineType.Horizontal) {
        y = l1.points[0].y;
    }

    if (eq2.type === LineType.Vertical) {
        x = l2.points[0].x;
    } else if (eq2.type === LineType.Horizontal) {
        y = l2.points[0].y;
    }

    if (x !== undefined && y !== undefined) {
        return p2(x, y);
    }

    type T = { general: General };

    if (x === undefined && y === undefined) {
        const { a, b: c } = (eq1 as T).general;
        const { a: b, b: d } = (eq2 as T).general;

        x = (d - c) / (a - b);
        y = a * ((d - c) / (a - b)) + c;
    } else {
        const { a, b } = eq1.type === LineType.Regular ? eq1.general : (eq2 as T).general;

        if (y === undefined) {
            y = a * (x as number) + b;
        } else {
            x = (y - b) / a;
        }
    }

    return p2(x as number, y);
}

