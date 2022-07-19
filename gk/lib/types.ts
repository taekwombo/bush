import type { Color } from './color.js';

export const enum LineType {
    Vertical = 'v',
    Horizontal = 'h',
    Slope = 's', 
}

export type Range2 = {
    x: [min: number, max: number];
    y: [min: number, max: number];
}

export type ImageDataExt = ImageData & {
    validate(x: number, y: number): boolean;
    index(x: number, y: number): number;
    drawPoint(this: ImageDataExt, x: number, y: number, color?: Color): void; 
    floodFill(x: number, y: number, color: Color, mod?: 4 | 8): void;
}

export interface Draw {
    draw: (image: ImageDataExt) => this;
}

export interface Fill {
    fill: (image: ImageDataExt) => this;
}

export interface Debug {
    debug: () => string;
}

export interface Eq {
    eq: (lhs: this, rhs: this) => boolean;
}
