import type { CliInput, FlagNames, Added, Cli as ICli } from './types.ts';
import type { InferV } from './generic.ts';
import type { Options as InputOptions } from './inputs.ts';

import { assert } from '../assert.ts';
import * as basic from './inputs.ts';

class Flags implements FlagNames {
    public static empty(): Flags {
        return new Flags();
    }

    set: Set<string>;

    protected constructor() {
        this.set = new Set();
    }

    public add(name: string): this {
        this.validate(name);
        this.set.add(name);
        return this;
    }

    public has(name: string): boolean {
        return this.set.has(name);
    }

    protected validate(name: string): void {
        assert(name !== 'help', 'Invalid option name "help", reserved.');
        assert(/^[a-zA-Z-]+$/.test(name), `Invalid option name "${name}", valid characters a-z, A-Z, -.`);
        assert(this.set.has(name) === false, `Invalid option name "${name}", already exists.`);
    }
}

type Options<T> = Omit<InputOptions<T>, 'name'>;

export class Cli<O extends Record<string, unknown>> implements ICli<O> {
    flags: Flags = Flags.empty();
    inputs: CliInput<string, unknown>[] = [];

    public add<I extends CliInput<string, unknown>>(input: I): Cli<O & Added<I>> {
        input.onAdd(this.flags);
        this.inputs.push(input);

        return this as unknown as Cli<O & Added<I>>;
    }

    public parse(args: string[]): O {
        if (args.includes('-h') || args.includes('--help')) {
            console.log('%cUSAGE', 'font-weight: bold');
            this.inputs.forEach((i) => i.display());

            Deno.exit(0);
        }

        const result = {} as O;

        for (const [k, v] of this.inputs.map((input) => input.parse(args))) {
            (result as Record<string, unknown>)[k] = v;
        }

        return result;
    }

    public bool<K extends string, P extends Options<boolean>>(
        name: K,
        opt?: P,
    ): Cli<O & Record<K, InferV<boolean, P>>> {
        return this.add(basic.bool({ ...opt, name }));
    }

    public int<K extends string, P extends Options<number>>(
        name: K,
        opt?: P,
    ): Cli<O & Record<K, InferV<number, P>>> {
        return this.add(basic.int({ ...opt, name }));
    }

    public num<K extends string, P extends Options<number>>(
        name: K,
        opt?: P,
    ): Cli<O & Record<K, InferV<number, P>>> {
        return this.add(basic.int({ ...opt, name }));
    }

    public str<K extends string, P extends Options<string>>(
        name: K,
        opt?: P,
    ): Cli<O & Record<K, InferV<string, P>>> {
        return this.add(basic.str({ ...opt, name }));
    }
}
