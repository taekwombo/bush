export type Result = null | [flag: string, value: null | string];

export class Flag {
    public flags: RegExp[];

    public constructor(names: string[]) {
        if (names.length === 0) {
            throw new Error('new Arg(names) expects names to have at least one element');
        }

        this.flags = names.map((n) => {
            return new RegExp(`^--?${n}(=.+)?$`);
        });
    }

    public parse(args: string[], expectsValue: boolean = true): Result {
        for (let i = 0; i < args.length; i++) {
            const current = args[i];

            for (const flag of this.flags) {
                if (!flag.test(current)) {
                    continue;
                }

                const name = current.replace(/^-{1,2}/, '').replace(/=.*$/, '');

                if (!expectsValue) {
                    args.splice(i, 1);
                    return [name, null];
                }

                const valueIdx = current.indexOf('=');

                if (valueIdx >= 0) {
                    args.splice(i, 1);
                    return [name, current.slice(valueIdx + 1)];
                }

                const value = args[i + 1];

                args.splice(i, 2);
                return [name, value ?? null];
            }
        }

        return null;
    }
}
