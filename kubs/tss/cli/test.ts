import { describe, it } from 'jsr:@std/testing/bdd';
import { expect } from 'jsr:@std/expect';

import { Cli } from './mod.ts';
import { Flag } from './flag.ts';

function required(v: string): void {
    (v);
}

describe('Types', () => {
    it('produces T when no options provided', () => {
        const v = new Cli().str('test').parse(['--test', 'val']);

        required(v.test);
        expect(v.test).toBe('val');
    });

    it('produces T when optional: true', () => {
        const cli = new Cli().str('test', { optional: true });

        expect(() => {
            // @ts-expect-error: string required, { optional: true } adds null
            required(cli.parse(ARGS()).test);
        }).toThrow();

        expect(cli.parse([]).test).toBe(null);
        expect(cli.parse(['--test', 'val']).test).toBe('val');
    });

    it('produces T when defaultValue: T', () => {
        const v = new Cli().str('test', { defaultValue: 'hi' }).parse([]);

        required(v.test);
        expect(v.test).toBe('hi');
    });

    it('expects at least one enum variant', () => {
        expect(() => new Cli().strEnum('day', [])).toThrow();
    });

    it('expects enum variants that are non-empty strings', () => {
        expect(() => new Cli().strEnum('day', [''])).toThrow();
        expect(() => new Cli().strEnum('day', ['one', ''])).toThrow();
    });

    it('enum - required', () => {
        const cli = new Cli().strEnum('day', ['monday', 'sunday']);
        const ok: Record<'day', 'monday' | 'sunday'> = cli.parse(['--day', 'monday']);

        expect(ok).toEqual({ day: 'monday' });
        expect(() => cli.parse(['--day', 'tuesday'])).toThrow();
    });

    it('enum - optional', () => {
        const cli = new Cli().strEnum('day', ['monday'], { optional: true });
        const ok: Record<'day', 'monday' | null> = cli.parse([]);

        expect(ok).toEqual({ day: null });
    });

    it('enum - non alphabetic variant', () => {
        const cli = new Cli().strEnum('test', ['0', 'a space', '│']);

        expect(cli.parse(['--test', '0'])).toEqual({ test: '0' });
        expect(cli.parse(['--test', 'a space'])).toEqual({ test: 'a space' });
        expect(cli.parse(['--test', '│'])).toEqual({ test: '│' });

        expect(() => {
            cli.parse(['--test', 'x']);
        }).toThrow();
    });

    it('enum - variant must be a string', () => {
        // @ts-expect-error variants must be a string array
        new Cli().strEnum('test', [1, 0]);
    });

    it('reserves help and h', () => {
        expect(() => new Cli().int('test', { shortName: 'h' })).toThrow();
        expect(() => new Cli().int('test', { shortName: 'help' })).toThrow();
        expect(() => new Cli().int('help')).toThrow();
        expect(() => new Cli().int('h')).toThrow();
    });

    it('expects valid names', () => {
        expect(() => new Cli().int('with spaces')).toThrow();
        expect(() => new Cli().int('numbers00000')).toThrow();
        expect(() => new Cli().int('/')).toThrow();
        expect(() => new Cli().int('\n')).toThrow();
        expect(() => new Cli().int('')).toThrow();
    });

    it('expects unique names', () => {
        expect(() => new Cli().int('test', { shortName: 'test' })).toThrow();
        expect(() => new Cli().int('test').int('test')).toThrow();
    });
});

describe('Flag', () => {
    it('expects at least one name', () => {
        expect(() => new Flag([])).toThrow();
    });

    it('expects flags that are not empty', () => {
        expect(() => new Flag([{ name: '' }])).toThrow();
        expect(() => new Flag([{ name: ' ' }])).toThrow();
    });

    it('modifies input args when flag found', () => {
        const args_1 = ['--test', 'value'];
        const args_2 = ['--test', 'value'];

        const flag = Flag.new('test');

        flag.parse(args_1, { expectsValue: true });
        flag.parse(args_2, { expectsValue: false });

        expect(args_1.length).toBe(0);
        expect(args_2.length).toBe(1);
    });

    it('expects value by default', () => {
        expect(
            Flag.new('test').parse(['--test'])
        ).toEqual(
            Flag.new('test').parse(['--test'], { expectsValue: true })
        )
    });

    it('finds long name --<name> <value>', () => {
        const flag = Flag.new('test');

        expect(flag.parse(['--test', 'val'])).toEqual(['test', 'val']);
    });

    it('finds long name --<name>=<value>', () => {
        const flag = Flag.new('test');

        expect(flag.parse(['--test=val'])).toEqual(['test', 'val']);
    });

    it('finds short name -<name> <value>', () => {
        const flag = Flag.new('long-test', 'test');

        expect(flag.parse(['-test', 'val'])).toEqual(['test', 'val']);
    });

    it('finds short name -<name>=<value>', () => {
        const flag = Flag.new('long-test', 'test');

        expect(flag.parse(['-test=val'])).toEqual(['test', 'val']);
    });

    it('finds <name>=<value> when expectsValue: true', () => {
        const flag = Flag.new('test');

        expect(flag.parse(['--test=val'], { expectsValue: false })).toEqual(['test', null]);
    });

    it('removes single arg only when expectsValue: true', () => {
        const flag = Flag.new('test');
        const args = ['--test', 'value'];

        expect(flag.parse(args, { expectsValue: false })).toEqual(['test', null]);

        expect(args).toEqual(['value']);
    });

    it('finds flags starting with dashes', () => {
        const flag = Flag.new('-test', '--t');

        expect(flag.parse(['---test'])).toEqual(['-test', null]);
        expect(flag.parse(['---t'])).toEqual(['--t', null]);
    });

    it('removes flags when found', () => {
        const args = ['--apple=1', '--orange='];

        expect(Flag.new('apple').parse(args)).toEqual(['apple', '1']);
        expect(args.length).toBe(1);

        expect(Flag.new('pear').parse(args)).toEqual(null);
        expect(args.length).toBe(1);

        expect(Flag.new('banana').parse(args)).toEqual(null);
        expect(args.length).toBe(1);

        expect(Flag.new('orange').parse(args)).toEqual(['orange', '']);
        expect(args.length).toBe(0);
    });

    it('handles multiple = in values', () => {
        const flag = Flag.new('test');

        expect(flag.parse(['--test=one=two'])).toEqual(['test', 'one=two']);
    });

    it('can throw on multiple values provided for a flag', () => {
        const flag = Flag.new('test');

        expect(() => {
            flag.parse(['--test=1', '--test=2'], { ensureUnique: true });
        }).toThrow();
    });
});

