import type { Line2 } from '../line.js';

export enum LineType {
    Vertical = 'v',
    Horizontal = 'h',
    Regular = 'r',
};

/**
 * Standard line equation
 * Ax + By + C = 0
 */
type Standard = {
    a: number;
    b: number;
    c: number;
};

/**
 * General line equation
 * y = ax + b
 */
export type General = {
    a: number;
    b: number;
};

type Equation = { standard: Standard } & (
    | { type: LineType.Vertical; x: number }
    | { type: LineType.Horizontal; y: number }
    | { type: LineType.Regular; general: General }
);

export function equation(line: Line2): Equation {
    line.assert2Point();    

    const [start, end] = line.points;

    const dx = end.x - start.x;
    const dy = end.y - start.y;

    // Vertical line
    if (dx === 0) {
        return {
            type: LineType.Vertical,
            x: start.x,
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
            type: LineType.Horizontal,
            y: start.y,
            standard: {
                a: 0,
                b: 1,
                c: -start.y,
            },
        };
    }

    const a = dy / dx;
    const b = start.y - a * start.x;

    // 1y = ax + c
    // y - ax - c = 0
    // => ax + b - y = 0
    // ax + by = c
    return {
        type: LineType.Regular,
        general: { a, b },
        // TODO: should be an INTEGER
        standard: { a: -a, b: 1, c: -b },
    };
}
