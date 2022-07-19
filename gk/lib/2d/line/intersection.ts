import { p2 } from '../point.js';
import { nn } from '../../utils.js';
import { Line2 } from '../line.js';
import { LineType } from '../../types.js';
import type { Segment2 } from '../segment.js';
import type { Point2 } from '../point.js';

/**
 * Finds an intersection of a line "line" and a segment "seg".
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
export function segment(line: Line2, segment: Segment2): null | Point2 {
    const { standard: { a, b, c } } = line;
    const { start: s, end: e } = segment;

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
 * Finds an intersection of two lines.
 *
 * https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
 */
export function lines(l1: Line2, l2: Line2): null | Point2 {
    if (l1.type !== LineType.Slope && l1.type === l2.type) {
        // May be 0 or Infinity intersection points.
        return null;
    }

    let x: number | undefined;
    let y: number | undefined;

    if (l1.type === LineType.Vertical) {
        x = l1.segment.start.x;
    } else if (l1.type === LineType.Horizontal) {
        y = l1.segment.start.y;
    }

    if (l2.type === LineType.Vertical) {
        x = l2.segment.start.x;
    } else if (l2.type === LineType.Horizontal) {
        y = l2.segment.start.y;
    }

    if (x !== undefined && y !== undefined) {
        return p2(x, y);
    }

    if (x === undefined && y === undefined) {
        const { a, b: c } = nn(l1.slope);
        const { a: b, b: d } = nn(l2.slope);

        x = (d - c) / (a - b);
        y = a * ((d - c) / (a - b)) + c;
    } else {
        const { a, b } = nn(l1.type === LineType.Slope ? l1.slope : l2.slope);

        if (y === undefined) {
            y = a * (x as number) + b;
        } else {
            x = (y - b) / a;
        }
    }

    return p2(x as number, y);
}

