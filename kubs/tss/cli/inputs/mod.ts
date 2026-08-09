import type { Cli } from '../types.ts';
import type { Generic, InferK, InferV } from './options.ts';

import { Flag } from '../flag.ts';
import { Help } from './help.ts';
import { Parse } from './parse.ts';
import * as utils from './utils.ts';

export type Options<T> = Generic<T>;

type Arg<T, O extends Options<T>> = Cli.Input<InferK<O>, InferV<T, O>>;

export function bool<O extends Options<boolean>>(options: O): Arg<boolean, O> {
    type K = InferK<O>;
    type V = InferV<boolean, O>;

    const offName = `no-${options.name}`;
    const names = utils.names([options.name, offName], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: false, ensureUnique: true },
        valueParser: Parse.bool(offName, options.name, options.shortName),
    });
    const help = Help.fromOptions('boolean', names, options);

    return utils.makeInput<K, V>(parser, help, names);
}

export function num<O extends Options<number>>(options: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    const names = utils.names([options.name], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: true, ensureUnique: true },
        valueParser: Parse.number,
    });
    const help = Help.fromOptions('number', names, options);

    return utils.makeInput<K, V>(parser, help, names);
}

export function int<O extends Options<number>>(options: O): Arg<number, O> {
    type K = InferK<O>;
    type V = InferV<number, O>;

    const names = utils.names([options.name], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: true, ensureUnique: true },
        valueParser: Parse.integer,
    });
    const help = Help.fromOptions('integer', names, options);

    return utils.makeInput<K, V>(parser, help, names);
}

export function str<O extends Options<string>>(options: O): Arg<string, O> {
    type K = InferK<O>;
    type V = InferV<string, O>;

    const names = utils.names([options.name], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: true, ensureUnique: true },
        valueParser: Parse.str,
    });
    const help = Help.fromOptions('string', names, options);

    return utils.makeInput<K, V>(parser, help, names);
}

export function strEnum<O extends (Options<E> & { variants: E[] }), E extends string>(options: O): Cli.Input<InferK<O>, InferV<E, O>> {
    if (options.variants.length === 0) {
        throw new Error('strEnum input must have non-empty variants');
    }

    if (options.variants.some((v) => v.length === 0)) {
        throw new Error('stdEnum variants must be non-empty');
    }

    type K = InferK<O>;
    type V = InferV<E, O>;

    const names = utils.names([options.name], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: true, ensureUnique: true },
        valueParser: Parse.strEnum(options.variants),
    });
    const help = Help
        .fromOptions('string', names, options)
        .addInfo(`variants=${options.variants.join(', ')}`);

    return utils.makeInput<K, V>(parser, help, names);
}

export function range<O extends Options<[number, number]>>(options: O): Arg<[number, number], O> {
    type K = InferK<O>;
    type V = InferV<[number, number], O>;

    const names = utils.names([options.name], [options.shortName]);
    const parser = utils.makeParser<K, V>({
        options,
        flag: new Flag(names),
        parseOptions: { expectsValue: true, ensureUnique: true },
        valueParser: Parse.range,
    });
    const help = Help.fromOptions('range', names, options);

    return utils.makeInput<K, V>(parser, help, names);
}
