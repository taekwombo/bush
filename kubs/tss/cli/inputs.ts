import type { InferK, InferV, CliOptions, CliInput } from './types.ts';
import { GenericInput, Result, ParseCallback } from './generic.ts';

type Arg<T, O extends CliOptions<T>> = CliInput<InferK<O>, InferV<T, O>>;

export function bool<O extends CliOptions<boolean>>(opt: O): Arg<boolean, O> {
    type K = InferK<O>;
    type V = InferV<boolean, O>;

    const offName = `no-${opt.name}`;

    return new GenericInput<K, V>(Parse.bool(offName, name), opt, [offName], false);
}

export function num<O extends CliOptions<number>>(opt: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    return new GenericInput<K, V>(Parse.number, opt, []);
}

export function int<O extends CliOptions<number>>(opt: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    return new GenericInput<K, V>(Parse.integer, opt, []);
}

export function str<O extends CliOptions<string>>(opt: O): Arg<string, O> {
    type K = InferK<O>;
    type V = InferV<string, O>;

    return new GenericInput<K, V>(Parse.str, opt, []);
}

class Parse {
    static bool(offName: string, name: string): ParseCallback<boolean> {
        return (key: string): Result<boolean> => {
            if (key === offName)
                return [true, false];
            if (key === name)
                return [true, true];

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
        const [ok, res] = this.number(_, value);

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
}
