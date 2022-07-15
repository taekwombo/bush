import type { Line2 } from '../line.js';

export enum LineType {
    Vertical = 'v',
    Horizontal = 'h',
    Regular = 'r',
};

type Equation = { type: LineType.Vertical; x: number; }
    | { type: LineType.Horizontal, y: number }
    | {
        type: LineType.Regular,
        general: { a: number; b: number };
        standard: { a: number; b: 1; c: number };
    };

export function equation(line: Line2): Equation {
    line.assert2Point();    

    const [start, end] = line.points;

    const dx = end.x - start.x;
    const dy = end.y - start.y;

    // Vertical line
    if (dx === 0) {
        return { type: LineType.Vertical, x: start.x };
    }

    // Horizontal line
    if (dy === 0) {
        return { type: LineType.Horizontal, y: start.y };
    }

    const a = dy / dx;
    const b = start.y - a * start.x;

    // y = ax + b
    // ax + by = c
    return {
        type: LineType.Regular,
        general: { a, b },
        standard: { a, b: 1, c: b },
    };
}
