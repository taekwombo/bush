import type { Img } from './img.js';

export const enum LineType {
    Vertical = 'v',
    Horizontal = 'h',
    Slope = 's', 
}

export interface Draw {
    draw: (image: Img) => this;
}

export interface Fill {
    fill: (image: Img) => this;
}

export interface Debug {
    debug: () => string;
}

export interface Eq {
    eq: (lhs: this, rhs: this) => boolean;
}

export interface Clone<T> {
    clone: (this: T) => T;
}
