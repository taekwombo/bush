export function nn<T>(v: T): Exclude<T, null> {
    if (v === null) {
        throw new Error('Expected value to be non null');
    }

    return v as Exclude<T, null>;
}

/**
 * Ensure that all arguments are not NaN nor Infinity.
 */
export function num(...vals: number[]): void {
    for (const v of vals) {
        if (Math.abs(v) === Infinity || Number.isNaN(v)) {
            throw new Error(`arguments can not be NaN nor Infinity`);
        }
    }
}

