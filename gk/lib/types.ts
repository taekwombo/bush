import type { Color } from './color.js';

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
