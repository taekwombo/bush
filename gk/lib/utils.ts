export function nn<T>(v: T): Exclude<T, null> {
    if (v === null) {
        throw new Error('Expected value to be non null');
    }

    return v as Exclude<T, null>;
}

/** Add overline */
export function ovl(v: string): string {
    return v
        .split('')
        .map((c) => `${c}\u0305`)
        .join('');
}
