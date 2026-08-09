export type Result<T> = [false, string] | [true, T];
export type ParseCallback<V> = (key: string, value: string | null) => Result<V>;

export class Parse {
    static bool(off: string, long: string, short?: string): ParseCallback<boolean> {
        return (key: string): Result<boolean> => {
            if (key === off)
                return [true, false];
            if (key === long)
                return [true, true];
            if (short !== undefined && key === short) {
                return [true, true];
            }

            return [false, 'Unreachable'];
        };
    }

    static number(_: string, value: string | null): Result<number> {
        if (value === null) {
            return [false, 'Missing value, expected integer'];
        }

        const result = Number(value);

        if (Number.isNaN(result)) {
            return [false, 'Invalid value provided ' + value];
        }

        if (!Number.isFinite(result)) {
            return [false, 'Got infinity'];
        }

        return [true, result];
    }

    static integer(_: string, value: string | null): Result<number> {
        const [ok, res] = Parse.number(_, value);

        if (!ok) {
            return [false, res];
        }

        if (res % 1 !== 0) {
            return [false, 'Expected integer, got ' + res];
        }

        return [true, res];
    }

    static str(_: string, value: string | null): Result<string> {
        if (value === null)
            return [false, 'Missing value'];

        return [true, value];
    }

    static strEnum<E>(variants: string[]): (_: string, value: string | null) => Result<E> {
        return (k, v) => {
            const [res, out] = Parse.str(k, v);

            if (!res) {
                return [res, out];
            }

            if (!variants.includes(out)) {
                return [false, `Invalid value "${out}" provided, expected one of: ${variants.join(', ')}`];
            }

            return [true, out] as Result<E>;
        };
    }

    static range(_: string, value: string | null): Result<[number, number]> {
        if (value === null)
            return [false, 'Missing value'];

        if (!/^-?\d+\.\.-?\d+$/.test(value))
            return [false, 'Invalid value, expecing <int>..<int>'];

        const [left, right] = value.split('..');
        const [rs, start] = Parse.integer(_, left);
        const [re, end] = Parse.integer(_, right);

        if (!rs || !re)
            return [false, 'Invalid value, expecing <int>..<int>'];

        return [true, [start, end]];
    }
}
