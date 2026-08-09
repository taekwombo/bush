export namespace Cli {
    /** Prints help for a single input arguement */
    export interface Help {
        print(): void;
    }

    /** Keeps track of already registered flags */
    export interface RegisteredFlags {
        /** Tries to add `name` to the flag list - must throw when name is invalid or already present */
        add(name: string): this;

        /** Checks whether `name` is already present in the list */
        has(name: string): boolean;
    }


    export interface Input<K extends string, V> {
        onAdd(reg: RegisteredFlags): void;
    
        parse(args: string[]): [K, V];

        help(): Help;
    }
}

export type Added<T extends Cli.Input<string, unknown>> = T extends Cli.Input<infer K, infer V>
    ? Record<K, V>
    : never
    ;

export interface Cli<O extends Record<string, unknown>> {
    add<I extends Cli.Input<string, unknown>>(input: I): Cli<O & Added<I>>;

    parse(args: string[]): O;
}
