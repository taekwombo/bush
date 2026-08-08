import type { CliInput, FlagNames, PrintHelp } from './types.ts';
import { Flag } from './flag.ts';

export namespace Options {
    export interface Base<T> {
        optional?: boolean;
        defaultValue?: T;
    }

    export interface Names {
        name: string;
        shortName?: string;
    }

    export interface Description {
        typeName?: string;
        description?: string;
    }

    export interface Generic<T> extends Base<T>, Names, Description {}

    export interface FlagOptions extends Flag.ParseOptions {
        extraNames?: Flag.Descriptor[],
    }
}

export type InferK<O extends Options.Names> = O['name'];
export type InferV<T, O extends Options.Base<T>> = O['optional'] extends true
    ? O['defaultValue'] extends T
        ? T
        : T | null
    : T
    ;

export type Result<T> = [false, string] | [true, T];
export type ParseCallback<V> = (key: string, value: string | null) => Result<V>;

export class GenericInput<K extends string, V> implements CliInput<K, V> {
    private parseCb: ParseCallback<V>
    private names: Flag.Descriptor[] = [];
    private opt: Options.Generic<V>;
    private parseOpt?: Flag.ParseOptions;
    private helpInfo: Help;

    public constructor(
        parseCb: ParseCallback<V>,
        opt: Options.Generic<V>,
        flagOptions?: Options.FlagOptions,
    ) {
        this.parseCb = parseCb;
        this.opt = opt;
        this.parseOpt = flagOptions || { expectsValue: true, ensureUnique: true };

        this.names = [{ name: this.opt.name }];

        if (this.opt.shortName) {
            this.names.push({ name: this.opt.shortName, short: true });
        }

        if (flagOptions?.extraNames) {
            this.names.push(...flagOptions.extraNames);
        }

        this.helpInfo = Help.fromOptions(this.names, this.opt);
    }

    public onAdd(reg: FlagNames): void {
        for (const { name } of this.names) {
            reg.add(name);
        }
    }

    public parse(args: string[]): [K, V] {
        const result = new Flag(this.names).parse(args, this.parseOpt);
        const { defaultValue, optional, name } = this.opt;

        if (result === null) {
            if (defaultValue !== undefined) {
                return [name as K, defaultValue];
            }
            if (optional) {
                return [name as K, null as V];
            }

            throw new Error(`Failed to find input for flag: ${this.names.map((f) => f.name).join('/')}`);
        }

        const [err, value] = this.parseCb(result[0], result[1]);

        if (!err) {
            throw new Error(`Failed to parse value for flag ${this.names.map((f) => f.name).join('/')}: ${value}`);
        }

        return [name as K, value as V];
    }

    public help(): Help {
        return this.helpInfo;
    }
}

export class Help implements PrintHelp {
    public static fromOptions(names: Flag.Descriptor[], options: Options.Generic<unknown>): Help {
        const info = [];

        if (options.typeName) {
            const isOptional = options.defaultValue !== undefined || options.optional;

            info.push(`type=${options.typeName}${isOptional ? '?' : ''}`);
        }

        if (options.defaultValue !== undefined) {
            info.push(`default=${options.defaultValue}`);
        }

        return new Help(names, info, options.description);
    }

    private names: Flag.Descriptor[];
    private info: string[];
    private desc?: string;

    private constructor(names: Flag.Descriptor[], info: string[], desc?: string) {
        this.names = names;
        this.info = info;
        this.desc = desc;
    }

    public addInfo(value: string): this {
        this.info.push(value);

        return this;
    }

    public print() {
        const pad = '  ';

        console.log(pad + '%c' + this.names.map((f) => f.name).join(', '), 'font-weight: bold');

        for (const info of this.info) {
            console.log(pad + pad + info);
        }

        if (this.desc) {
            console.log(pad + pad + this.desc);
        }
    }
}
