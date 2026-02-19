import { assert } from './assert.ts';

const STYLE = {
    bold: 'font-weight: bold',
};

export class Flag {
    public names: string[];

    public constructor(names: string[]) {
        assert(names.length > 0, 'new Arg(names) expects names to have at least one element');

        this.names = names;
    }

    public parse(args: string[], expectsValue: boolean = true): null | [flag: string, value: null | string] {
        for (let i = 0; i < args.length; i++) {
            const current = args[i];

            for (const flag of this.names) {
                if (!current.startsWith(flag)) {
                    continue;
                }

                if (!expectsValue) {
                    args.splice(i, 1);
                    return [flag, null];
                }

                let value = current.slice(flag.length);

                if (value[0] === '=') {
                    args.splice(i, 1);
                    return [flag, value.slice(1)];
                }

                value = args[i + 1];

                args.splice(i, 2);
                return [flag, value ?? null];
            }
        }

        return null;
    }
}

export namespace FlagInput {
    export type InputType = 'string' | 'number' | 'bigint' | 'boolean';
    export type Options = {
        optional: boolean;
        inputType: InputType;
        defaultValue?: any;
        description?: string;
        allowedValues?: any[];
    };

    export type NamesRegistry = {
        short: Set<string>;
        long: Set<string>;
    };

    export type Failure = { kind: 'missing_input' } | { kind: 'invalid', value: string } | { kind: 'value_not_allowed', value: any };
}


export class FlagInput {
    protected static validateName(name: string, reg: FlagInput.NamesRegistry) {
        assert(name !== 'help', 'Invalid option name "help", reserved.');
        assert(/^[a-zA-Z-]+$/.test(name), `Invalid option name "${name}", valid characters a-z, A-Z, -.`);
        assert(reg.long.has(name) === false, `Invalid option name "${name}", already exists.`);
    }

    protected static assignShortName(long: string, reg: FlagInput.NamesRegistry): string | null {
        for (const char of long) {
            if (reg.short.has(char)) {
                continue;
            }

            reg.short.add(char);
            return `-${char}`;
        }

        return null;
    }

    protected static assignOffName(long: string, reg: FlagInput.NamesRegistry, inputType: FlagInput.InputType): string | null {
        if (inputType !== 'boolean') {
            return null;
        }

        const off = `no-${long}`;

        this.validateName(off, reg);
        reg.long.add(off);

        return `--${off}`;
    }

    protected static isOffFlag(name: string): boolean {
        return name.startsWith('--no-');
    }

    public name: string;
    public flag: Flag;
    public opts: FlagInput.Options;

    public constructor(name: string, options: FlagInput.Options, reg: FlagInput.NamesRegistry) {
        FlagInput.validateName(name, reg);

        const short = FlagInput.assignShortName(name, reg);
        const off = FlagInput.assignOffName(name, reg, options.inputType);
        const names = [`--${name}`, short, off].filter((v) => v !== null);

        this.name = name;
        this.flag = new Flag(names);
        this.opts = options;
    }

    private fail(error: FlagInput.Failure): never {
        let message: string;

        switch (error.kind) {
            case 'missing_input':
                message = `No value was found for flag "${this.name}"`;
                break;
            case 'invalid':
                message = `Invalid value "${error.value}" found for flag "${this.name}" of type "${this.opts.inputType}"`;
                break;
            case 'value_not_allowed':
                message = `Invalid value "${error.value}" found for flag "${this.name}", must be one of [${this.opts.allowedValues?.join(',')}]`;
        }

        throw new Error(message);
    }

    public find(args: string[]): null | string | number | bigint | boolean {
        const result = this.flag.parse(args, this.opts.inputType !== 'boolean');

        if (result === null) {
            if (this.opts.defaultValue) {
                return this.opts.defaultValue;
            }

            if (this.opts.optional) {
                return null;
            }

            this.fail({ kind: 'missing_input' });
        }

        const [key, value] = result;

        let output: any;

        switch (this.opts.inputType) {
            case 'string':
                if (value === null) {
                    this.fail({ kind: 'missing_input' });
                }

                output = value;
                break;

            case 'number':
                if (value === null) {
                    this.fail({ kind: 'missing_input' });
                }

                output = this.asNumber(value);
                break;

            case 'bigint':
                if (value === null) {
                    this.fail({ kind: 'missing_input' }); 
                }

                output = this.asBigInt(value);
                break;

            case 'boolean':
                output = FlagInput.isOffFlag(key);
                break;
        }

        if (this.opts.allowedValues && !this.opts.allowedValues.includes(output)) {
            this.fail({ kind: 'value_not_allowed', value: output });
        }

        return output;
    }

    protected asNumber(value: string): number {
        const result = Number(value);

        if (Number.isNaN(result) || !Number.isFinite(result)) {
            this.fail({ kind: 'invalid', value });
        }

        return result;
    }

    protected asBigInt(value: string): bigint {
        try {
            return BigInt(value);
        } catch (_) {
            this.fail({ kind: 'invalid', value });
        }
    }

    public printUsage() {
        const pad = '  ';
        const names = this.flag.names.join('\t');

        let typeDescription = this.opts.inputType;
        
        if (this.opts.optional) {
            typeDescription += ` [optional]`;
        }
        if (this.opts.defaultValue) {
            typeDescription += ` [default=${this.opts.defaultValue}]`;
        }
        if (this.opts.allowedValues) {
            typeDescription += ` [allowed=${this.opts.allowedValues.join(',')}]`;
        }
        
        console.log('%c' + names, STYLE.bold);
        console.log(pad, typeDescription);

        if (this.opts.description) {
            console.log(pad, this.opts.description);
        }
    }
}

export namespace Cli {
    export type Options<Type, Optional> = {
        defaultValue?: Type;
        optional?: Optional;
        description?: string;
        allowedValues?: Type[];
    };

    export type Output<Type, Default extends Type | undefined, Optional extends boolean> =
        Optional extends true
            ? Default extends undefined
                ? Type | null
                : Type
            : Type;
}

export class Cli<T extends {}> {
    private names: FlagInput.NamesRegistry;
    public args: FlagInput[];
    public output: T;

    public constructor() {
        this.args = [];
        this.output = {} as T;
        this.names = {
            short: new Set(),
            long: new Set(),
        };

        this.names.long.add('help');
        this.names.short.add('h');
    }


    protected addArg(
        name: string,
        inputType: FlagInput.InputType,
        options?: Cli.Options<unknown, boolean>,
        addBoolOffFlag?: boolean,
    ) {
        this.args.push(
            new FlagInput(name, { ...options, optional: options?.optional || false, inputType }, this.names)
        );
    }

    public str<
        Key extends string,
        Def extends string | undefined = undefined,
        Opt extends boolean = false,
    >(
        name: Key,
        options?: Cli.Options<Def, Opt>,
    ): Cli<T & Record<Key, Cli.Output<string, Def, Opt>>> {
        this.addArg(name, 'string', options);

        return this as any;
    }

    public num<
        Key extends string,
        Def extends number | undefined = undefined,
        Opt extends boolean = false,
    >(
        name: Key,
        options?: Cli.Options<Def, Opt>,
    ): Cli<T & Record<Key, Cli.Output<number, Def, Opt>>> {
        this.addArg(name, 'number', options);

        return this as any;
    }

    public bigint<
        Key extends string,
        Def extends bigint | undefined = undefined,
        Opt extends boolean = false,
    >(
        name: Key,
        options?: Cli.Options<Def, Opt>,
    ): Cli<T & Record<Key, Cli.Output<bigint, Def, Opt>>> {
        this.addArg(name, 'bigint', options);

        return this as any;
    }

    public bool<
        Key extends string,
        Def extends boolean | undefined = undefined,
        Opt extends boolean = false,
    >(
        name: Key,
        options?: Cli.Options<Def, Opt>,
    ): Cli<T & Record<Key, Cli.Output<boolean, Def, Opt>>> {
        this.addArg(name, 'boolean', options);

        return this as any;
    }

    public printUsage() {
        console.error('%cUSAGE:', STYLE.bold);
        this.args.forEach((f) => f.printUsage());
    }

    public parse(from: string[]): T {
        if (Deno.args.findIndex((arg) => arg === '-h' || arg === '--help') > -1) {
            this.printUsage();
            Deno.exit(0);
        }

        const output: T = {} as any;

        for (const arg of this.args) {
            (output as Record<string, unknown>)[arg.name] = arg.find(from) as unknown;
        }

        return 0 as any as T;
    }
}
