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
});

describe('Flag', () => {
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

        expect(a.length).toBe(1);
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
            expect(args.length).toBe(1);
        }

        {
            const result = new Flag(['t']).parse(args);
            expect(result).toEqual(['t', '2']);
            expect(args.length).toBe(0);
        }
    });

    it('should ignore sub-expressions', () => {
        const args = ['--super-long', '1', '--longer=h=h', '--x=-x='];

        expect(new Flag(['long']).parse(args)).toBe(null);
        expect(new Flag(['longer']).parse(args)).toEqual(['longer', 'h=h']);
        expect(new Flag(['x']).parse(args)).toEqual(['x', '-x=']);
    });
});

