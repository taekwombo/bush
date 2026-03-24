import type { CliInput, FlagNames, CliOptions } from './types.ts';
import { Flag } from './flag.ts';

export type Result<T> = [false, string] | [true, T];
export type ParseCallback<V> = (key: string, value: string | null) => Result<V>;

export class GenericInput<K extends string, V> implements CliInput<K, V> {
    private parseCb: ParseCallback<V>
    private names: string[] = [];
    private expectsValue: boolean;
    private opt: CliOptions<V>;

    public constructor(
        parseCb: ParseCallback<V>,
        opt: CliOptions<V>,
        extraNames: string[],
        expectsValue: boolean = true,
    ) {
        this.parseCb = parseCb;
        this.expectsValue = expectsValue;
        this.opt = opt;
        this.names = [this.opt.name, this.opt.shortName]
            .filter((v) => v !== undefined)
            .concat(extraNames);
    }

    public onAdd(reg: FlagNames): void {
        for (const name of this.names) {
            reg.add(name);
        }
    }

    public parse(args: string[]): [K, V] {
        const result = new Flag(this.names).parse(args, this.expectsValue);
        const { defaultValue, optional, name } = this.opt;

        if (result === null) {
            if (defaultValue) {
                return [name as K, defaultValue];
            }
            if (optional) {
                return [name as K, null as V];
            }

            throw new Error(`Failed to find input for flag: ${this.names.join('/')}`);
        }

        const [err, value] = this.parseCb(result[0], result[1]);

        if (!err) {
            throw new Error(`Failed to parse value for flag ${this.names.join('/')}: ${value}`);
        }

        return [name as K, value as V];
    }
}
