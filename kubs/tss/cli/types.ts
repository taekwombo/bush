export interface FlagNames {
    /** Tries to add `name` to the flag list - may throw when name is invalid or already present */
    add(name: string): this;

    /** Checks whether `name` is already present in the list */
    has(name: string): boolean;
}

export interface CliInput<K extends string, V> {
    onAdd(reg: FlagNames): void;
    
    parse(args: string[]): [K, V];

    display(): void;
}

export type Added<T extends CliInput<string, unknown>> = T extends CliInput<infer K, infer V>
    ? Record<K, V>
    : never
    ;

export interface Cli<O extends Record<string, unknown>> {
    add<I extends CliInput<string, unknown>>(input: I): Cli<O & Added<I>>;

    parse(args: string[]): O;
}
