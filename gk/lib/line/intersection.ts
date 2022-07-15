import { p2 } from '../point.js';
import { equation } from './equation.js';
import type { Line2 } from '../line.js';
import type { Point2 } from '../point.js';

/**
 * Finds an intersection of a segments "l1" and "l2".
 */
export function standard(l1: Line2, l2: Line2): null | Point2 {
    // 3.1.1. [pdfs/grafika_komputerowa.pdf]
    // 
    // Intersection between Segment (P₁, P₂) and a line (ax + by = c).
    // P₁ = (x₁, y₁)
    // P₂ = (x₂, y₂)
    // 
    // |x|   |x₂|     |x₁ - x₁|
    // | | = |  | + t |       |  for t ∈ [0, 1]
    // |y|   |y₁|     |y₂ - y₂|
    //
    //         c - ax₁ - by₁
    // t = ───────────────────────
    //     a(x₂ - x₁) + b(y₂ - y₁)

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
 */
export function segments(l1: Line2, l2: Line2): null | Point2 {
}

/**
 * Finds an intersection of two lines passing through "l1" and "b" segments.
 */
export function lines(l1: Line2, l2: Line2): null | Point2 {
    l2.assert2Point();

    const eq = equation(l1);
    const { standard: { a, b, c } } = eq;
    console.log(eq);
    const [s, e] = l2.points;
    console.log(l1.points.map((p) => p.debug()));
    console.log(l2.points.map((p) => p.debug()));

    const v1 = a * s.x;
    const v2 = b * s.y;
    const v3 = -c - v1 - v2;
    const v4 = a * (e.x - s.x);
    const v5 = b * (e.y - s.y);
    const v6 = v4 + v5;
    const v7 = v3 / v6;
    console.log({ v1, v2, v3, v4, v5, v6, v7 });

    const t = v7;

    console.log({ t, a, b, c }, s, e);
    if (t <= 1 && t >= 0) {
        return p2(
            s.x + t * (e.x - s.x),
            s.y + t * (e.y - s.y),
        );
    }

    return null;
}
