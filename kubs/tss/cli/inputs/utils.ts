import type { Cli } from '../types.ts';
import type { Flag } from '../flag.ts';
import type { Base, Names } from './options.ts';

type FlagNames = (string | null | undefined)[];

export function names(long: FlagNames, short: FlagNames = []): Flag.Descriptor[] {
    return long
        .filter((v) => v != null)
        .map((name) => ({ name }))
        .concat(
            short.filter((v) => v != null).map((name) => ({ name, short: true }))
        );
}

type MakeParserOptions<T> = {
    flag: Flag,
    parseOptions: Flag.ParseOptions,
    valueParser: (k: string, v: string | null) => [false, error: string] | [true, T],
    options: Base<T> & Names,
};

export function makeParser<K, V>({ flag, parseOptions, valueParser, options }: MakeParserOptions<V>): (args: string[]) => [K, V] {
    return (args: string[]): [K, V] => {
        const result = flag.parse(args, parseOptions);
        const { defaultValue, optional, name } = options;

        // null | [key: string, null] trigger optional + default branch.
        if (result === null || result[1] === null) {
            if (defaultValue !== undefined) {
                return [name as K, defaultValue];
            }
            if (optional) {
                return [name as K, null as V];
            }

            if (result === null) {
                throw new Error(`Failed to find input for flag: ${name}`);
            }
        }

        const [err, value] = valueParser(result[0], result[1]);

        if (!err) {
            throw new Error(`Failed to parse value for flag ${name}: ${value}`);
        }

        return [name as K, value as V];
    };
}

export function makeInput<K, V>(parser: (args: string[]) => [K, V], help: Cli.Help, names: Flag.Descriptor[]) {
    return {
        onAdd: (reg: Cli.RegisteredFlags) => {
            names.forEach(({ name }) => reg.add(name));
        },
        parse: parser,
        help: () => help,
    }
}
