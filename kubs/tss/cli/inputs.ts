import type { CliInput } from './types.ts';
import type { InferK, InferV, Options as GenericOptions } from './generic.ts';

import { GenericInput, Result, ParseCallback } from './generic.ts';

export type Options<T> = Omit<GenericOptions.Generic<T>, 'typeName'>;

type Arg<T, O extends Options<T>> = CliInput<InferK<O>, InferV<T, O>>;

export function bool<O extends Options<boolean>>(opt: O): Arg<boolean, O> {
    type K = InferK<O>;
    type V = InferV<boolean, O>;

    const offName = `no-${opt.name}`;

    return new GenericInput<K, V>(Parse.bool(offName, opt.name, opt.shortName), { ...opt, typeName: 'boolean' }, [offName], false);
}

export function num<O extends Options<number>>(opt: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    return new GenericInput<K, V>(Parse.number, { ...opt, typeName: 'number' }, []);
}

export function int<O extends Options<number>>(opt: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    return new GenericInput<K, V>(Parse.integer, { ...opt, typeName: 'integer' }, []);
}

export function str<O extends Options<string>>(opt: O): Arg<string, O> {
    type K = InferK<O>;
    type V = InferV<string, O>;

    return new GenericInput<K, V>(Parse.str, { ...opt, typeName: 'string' }, []);
}

export function strEnum<O extends (Options<E> & { variants: E[] }), E extends string>(opt: O): CliInput<InferK<O>, InferV<E, O>> {
    if (opt.variants.length === 0) {
        throw new Error('strEnum input must have non-empty variants');
    }

    if (opt.variants.some((v) => v.length === 0)) {
        throw new Error('stdEnum variants must be non-empty');
    }

    type K = InferK<O>;
    type V = InferV<E, O>;

    const input = new GenericInput<K, V>(Parse.strEnum(opt.variants), { ...opt, typeName: 'string' }, []);

    input.help().addInfo(`variants=${opt.variants.join(', ')}`);

    return input;
}

export function range<O extends Options<[number, number]>>(opt: O): Arg<[number, number], O> {
    type K = InferK<O>;
    type V = InferV<[number, number], O>;

    return new GenericInput<K, V>(Parse.range, { ...opt, typeName: 'range' }, []);
}

class Parse {
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
