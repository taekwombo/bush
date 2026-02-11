import { assert } from './assert.ts';

export function character(): string {
    const rnd = Math.random() * 24;

    return String.fromCharCode(Math.floor(rnd) + 97)
}

export function alphabetic(length: number): string {
    assert(length >= 0, 'alphabetic length >= 0', { length });

    return new Array(length).fill(0).map(character).join('');
}
