import { describe, it } from 'jsr:@std/testing/bdd';
import { expect } from 'jsr:@std/expect';

import { Cli } from './mod.ts';
import { Flag } from './flag.ts';

const ARGS = () => ['-test', 'test'];

function required(v: string): void {
    (v);
}

describe('Types', () => {
    it('produces T when no options provided', () => {
        const v = new Cli().str('test').parse(ARGS());
        required(v.test);
        expect(v.test).toBe('test');
    });

    it('produces T when optional: true', () => {
        const cli = new Cli().str('test', { optional: true });

        // @ts-expect-error: string required, { optional: true } adds null
        required(cli.parse(ARGS()).test);

        expect(cli.parse([]).test).toBe(null);
        expect(cli.parse(ARGS()).test).toBe('test');
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
        const ok: Record<'day', 'monday' | 'sunday'> = cli.parse(['-day', 'monday']);

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

        expect(cli.parse(['-test', '0'])).toEqual({ test: '0' });
        expect(cli.parse(['-test', 'a space'])).toEqual({ test: 'a space' });
        expect(cli.parse(['-test', '│'])).toEqual({ test: '│' });
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

    it('modifies input `args` when flag found', () => {
        const args = ARGS();

        new Flag(['test']).parse(args);

        expect(args.length).toBe(0);
    });

    it('finds -<name> <value>', () => {
        const v = new Flag(['test']).parse(['-test', 'value']);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', 'value']);
    });

    it('finds --<name> <value>', () => {
        const v = new Flag(['test']).parse(['--test', 'value']);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', 'value']);
    });

    it('finds -<name>=<value>', () => {
        const v = new Flag(['test']).parse(['-test=value']);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', 'value']);
    });

    it('finds --<name>=<value>', () => {
        const v = new Flag(['test']).parse(['--test=value']);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', 'value']);
    });

    it('finds --?<name>=<val> when expectsValue: false', () => {
        const v = new Flag(['test']).parse(['-test=value'], false);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', null]);
    });

    it('should remove single element when expects: false', () => {
        const a = ['--test', 'value'];
        const v = new Flag(['test']).parse(a, false);

        expect(a).toEqual(['value']);
        expect(v).not.toBe(null);
        expect(v).toEqual(['test', null]);
    });

    it('should remove at most 2 leading hyphens', () => {
        const a = ['---test'];
        const v = new Flag(['-test']).parse(a, false);

        expect(v).not.toBe(null);
        expect(v).toEqual(['-test', null]);
    });

    it('should remove flag from args', () => {
        const args = ['--test=1', '-t=2'];

        {
            const result = new Flag(['test']).parse(args);
            expect(result).toEqual(['test', '1']);
            expect(args).toEqual(['-t=2']);
        }

        {
            const result = new Flag(['t']).parse(args);
            expect(result).toEqual(['t', '2']);
            expect(args.length).toBe(0);
        }
    });

    it('should ignore sub-string matches', () => {
        const args = ['--super-long', '1', '--longer=h=h', '--x=-x='];

        expect(new Flag(['long']).parse(args)).toBe(null);
        expect(new Flag(['longer']).parse(args)).toEqual(['longer', 'h=h']);
        expect(new Flag(['x']).parse(args)).toEqual(['x', '-x=']);
    });
});

