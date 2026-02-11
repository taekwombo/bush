import { assert } from './assert.ts';

type InputType = 'string' | 'int' | 'bigint' | 'boolean';

type Arg = {
    name: string;
    short?: string;
    optional: boolean;
    type: InputType;
    defaultValue?: any;
    description?: string;
    variants?: string[];
};

type Input<Key, Default, Optional> = {
    name: Key;
    defaultValue?: Default;
    optional?: Optional;
    description?: string;
    variants?: string[];
};

type Output<Type, Default extends Type | undefined, Optional extends boolean | undefined> = 
    Optional extends true
        ? Default extends Type
            ? Type
            : Type | null
        : Type;

const STYLE = {
    bold: 'font-weight: bold',
    opt: 'font-weight: bold;text-decoration: underline',
    accent: 'font-weight: bold;color: orange',
    normal: 'font-weight: normal;color: normal',
    blue: 'color: blue',
};

export class Args<T extends {}> {
    private args: string[];
    private expects: Arg[];
    private output: T;

    public constructor() {
        this.args = Deno.args.slice(0);
        this.expects = [];
        this.output = {} as T;
    }

    public str<
        K extends string,
        D extends string  | undefined = undefined,
        O extends boolean | undefined = false,
    >(input: Input<K, D, O>) {
        this.validateName(input.name);
        this.expects.push({
            ...input,
            type: 'string',
            optional: input.optional ?? false,
        });
        return this as unknown as Args<T & Record<K, Output<string, D, O>>>;
    }

    public int<
        K extends string,
        D extends number  | undefined = undefined,
        O extends boolean | undefined = false,
    >(input: Input<K, D, O>) {
        this.validateName(input.name);
        this.expects.push({
            ...input,
            type: 'int',
            optional: input.optional ?? false,
        });
        return this as unknown as Args<T & Record<K, Output<number, D, O>>>;
    }

    // defaultValue is always false for boolean CLI argument
    // optional is always true for boolean CLI argument
    public bool<
        K extends string,
        D extends boolean | undefined = undefined,
        O extends boolean | undefined = false,
    >(input: Omit<Input<K, D, O>, 'defaultValue' | 'optional'>) {
        this.validateName(input.name);
        this.expects.push({
            ...input,
            defaultValue: false,
            optional: true,
            type: 'boolean',
        });
        return this as unknown as Args<T & Record<K, Output<boolean, D, O>>>;
    }

    public bigInt<
        K extends string,
        D extends bigint  | undefined = undefined,
        O extends boolean | undefined = false,
    >(input: Input<K, D, O>) {
        this.validateName(input.name);
        this.expects.push({
            ...input,
            type: 'bigint',
            optional: input.optional ?? false,
        });
        return this as unknown as Args<T & Record<K, Output<bigint, D, O>>>;
    }

    private validateName(name: string) {
        assert(name !== 'help', 'Invalid option name "help", reserved.');
        assert(/^[a-zA-Z-]+$/.test(name), `Invalid option name "${name}", valid characters a-z, A-Z, -.`);
        assert(this.expects.find((i) => i.name === name) === undefined, `Invalid option name "${name}", already exists.`);
    }

    private assignShortNames() {
        const shorts: Set<string> = new Set();

        shorts.add('h');

        for (const arg of this.expects) {
            for (let i = 0; i < arg.name.length; i++) {
                const char = arg.name.slice(i, i + 1);

                if (shorts.has(char)) {
                    continue;
                }

                shorts.add(char);
                arg.short = char;
                break;
            }
        }
    }

    public parse(): T {
        this.assignShortNames();
        const output: T = {} as any;

        if (Deno.args.findIndex((arg) => arg === '-h' || arg === '--help') > -1) {
            this.printUsage();
            Deno.exit(0);
        }

        for (const arg of this.expects) {
            const val = this.getArg(arg);

            if (arg.variants && val && !arg.variants.includes(val)) {
                this.exit(arg, val);
            }

            switch (arg.type) {
                case 'boolean':
                    (output as any)[arg.name] = this.asBoolean(arg, val);
                    break;
                case 'string':
                    (output as any)[arg.name] = this.asString(arg, val);
                    break;
                case 'int':
                    (output as any)[arg.name] = this.asInt(arg, val);
                    break;
                case 'bigint':
                    (output as any)[arg.name] = this.asBigInt(arg, val);
                    break;
            }
        }

        return output;
    }

    private getArg(arg: Arg): string | null {
        const prefix = `--${arg.name}`;
        const shortP = `-${arg.short}`;
        for (const input of this.args) {
            const match = arg.short !== undefined
                ? input.startsWith(shortP) || input.startsWith(prefix)
                : input.startsWith(prefix);

            const val = input.split('=', 2)[1] ?? null;

            if (match && arg.type === 'boolean') {
                // By default when just --<name> is provided we treat is as true.
                return val ?? 'true';
            }

            if (!match || val === null) {
                continue;
            }

            return val;
        }

        if (!arg.optional) {
            this.exit(arg, null);
        }

        return null;
    }

    private asBoolean(arg: Arg, value: string | null): boolean | null {
        if (value === null) {
            return arg.defaultValue;
        }
        switch (value.toLowerCase()) {
            case 'no': return false;
            case 'false': return false;
            case 'true': return true;
            case 'yes': return true;
            default:
                this.exit(arg, value);
        }
    }

    private asString(arg: Arg, value: string | null): string | null {
        if (value === null) {
            if (arg.defaultValue !== undefined) {
                return arg.defaultValue;
            }
            return null;
        }

        return value;
    }

    private asInt(arg: Arg, value: string | null): number | null {
        if (value === null) {
            if (arg.defaultValue !== undefined) {
                return arg.defaultValue;
            }
            // getArg checks "optional" constraint
            return null;
        }

        const number = parseInt(value);

        if (Number.isNaN(number)) {
            this.exit(arg, { in: value, parsed: number });
        }

        return number;
    }

    private asBigInt(arg: Arg, value: string | null): bigint | null {
        if (value === null) {
            if (arg.defaultValue !== undefined) {
                return arg.defaultValue;
            }
            return null;
        }

        try {
            return BigInt(value);
        } catch (_) {
            this.exit(arg, value);
        }
    }

    private printUsage(failed?: Arg): void {
        console.error('%cUSAGE:', STYLE.bold);

        const pad = '  ';
        const pad2 = pad + pad;

        for (const { name, short, type, optional, description, variants, defaultValue } of this.expects) {
            const names = short ? `-${short} --${name}` : `--${name}`;

            const prefix = name === failed?.name ? `%c${names}` : names;
            const extraArgs = name === failed?.name ? [STYLE.bold, STYLE.accent, STYLE.normal] : [STYLE.bold, STYLE.normal];
            const argType = `${type}${optional ? '?' : ''}${defaultValue ? ' || ' + defaultValue : ''}`;

            console.error(`%c${prefix}=<${argType}>%c`, ...extraArgs);

            if (variants) {
                console.error(pad + '%cVariants:', STYLE.bold)
                console.error(pad2 + '%c' + variants.join(', '), STYLE.blue);
            }

            if (description) {
                console.error(pad + '%cDescription:', STYLE.bold)
                for (const l of description.split('\n')) {
                    console.error(pad2 + l);
                }
            }

            console.error('');
        }
    }

    private exit(failed: Arg, value: any): never {
        console.error(
           `%cMissing: %c--${failed.name}%c of type ${failed.type} - got "${value}"\n`,
           STYLE.bold,
           STYLE.accent,
           STYLE.normal,
         );

        this.printUsage(failed);

        Deno.exit(1);
    }
}
