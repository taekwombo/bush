import type { Cli } from '../types.ts';
import type { Flag } from '../flag.ts';
import type { Generic } from './options.ts';

export class Help implements Cli.Help {
    public static fromOptions(
        typeName: string,
        names: Flag.Descriptor[],
        options: Generic<unknown>,
    ): Help {
        const info = [];

        const isOptional = options.defaultValue !== undefined || options.optional;
        info.push(`type=${typeName}${isOptional ? '?' : ''}`);

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
