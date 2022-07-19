import { LineType } from '../../types.js';
import type { Segment2 } from '../segment.js';

/** y = ax + b */
export type Slope = {
    a: number;
    b: number;
};

/** ax + by + c = 0 */
export type Standard = {
    a: number;
    b: number;
    c: number;
};

type Equation = {
    type: LineType;
    standard: Standard;
    slope: null | Slope;
    segment: Segment2;
};

export function equation(segment: Segment2): Equation {
    const { start, end } = segment;

    const dx = end.x - start.x;
    const dy = end.y - start.y;

    // Vertical line
    if (dx === 0) {
        return {
            segment,
            type: LineType.Vertical,
            slope: null,
            standard: {
                a: 1,
                b: 0,
                c: -start.x,
            },
        };
    }

    // Horizontal line
    if (dy === 0) {
        return {
            segment,
            type: LineType.Horizontal,
            slope: {
                a: 0,
                b: start.y,
            },
            standard: {
                a: 0,
                b: 1,
                c: -start.y,
            },
        };
    }

    const a = dy / dx;
    const b = start.y - a * start.x;

    return {
        segment,
        type: LineType.Slope,
        slope: { a, b },
        standard: { a: -a, b: 1, c: -b },
    };
}
