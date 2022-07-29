type R2 = [min: number, max: number];

export class Range2 {
    public x: R2;
    public y: R2;

    public constructor(x: R2, y: R2) {
        this.x = x;
        this.y = y;
    }
}

export function r2(...args: ConstructorParameters<typeof Range2>): Range2 {
    return new Range2(...args);
}
