export namespace Flag {
    export type Descriptor = {
        name: string;
        short?: boolean;
    };

    export type NamedDescriptor = Descriptor & {
        arg: string;
    };

    export type ParseOptions = {
        expectsValue?: boolean;
        ensureUnique?: boolean;
    };

    export type ParseResult = null | [flag: string, value: null | string];
}

export class Flag {
    public static new(longName: string, shortName?: string): Flag {
        const flags: Flag.Descriptor[] = [{ name: longName }];

        if (shortName) {
            flags.push({ name: shortName, short: true });
        }

        return new Flag(flags);
    }

    private flags: Flag.NamedDescriptor[];

    public constructor(flags: Flag.Descriptor[]) {
        if (flags.length === 0) {
            throw new Error('new Flag(flags[]) expects names to have at least one element');
        }

        this.flags = [];

        for (const flag of flags) {
            const name = flag.name.trim();

            if (name.length == 0) {
                throw new Error('Flag.Descriptor.name cannot be empty');
            }

            this.flags.push({
                name,
                short: flag.short,
                arg: (flag.short ? '-' : '--') + name,
            });
        }
    }

    public parse(args: string[], options?: Flag.ParseOptions): Flag.ParseResult {
        const result = this.findValue(args, options?.expectsValue);

        if (options?.ensureUnique) {
            if (this.findValue(args, options?.expectsValue) !== null) {
                throw new Error(`Duplicate values for flag ${this.flags.map((f) => f.name).join(', ')} found`);
            }
        }

        return result;
    }

    private findValue(args: string[], expectsValue: boolean = true): Flag.ParseResult {
        for (let i = 0; i < args.length; i++) {
            const arg = args[i];

            if (arg[0] !== '-') {
                continue;
            }

            for (const flag of this.flags) {
                if (!arg.startsWith(flag.arg)) {
                    continue;
                }

                const isMatching = arg == flag.arg || arg[flag.arg.length] == '=';

                if (!isMatching) {
                    continue;
                }

                args.splice(i, 1);

                if (!expectsValue) {
                    return [flag.name, null];
                }

                if (arg == flag.arg) {
                    const value = args[i];

                    args.splice(i, 1);

                    return [flag.name, value ?? null];
                }

                return [flag.name, arg.slice(flag.arg.length + 1)];
            }
        }

        return null;
    }
}
