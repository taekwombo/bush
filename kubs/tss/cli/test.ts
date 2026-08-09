import { describe, it } from 'jsr:@std/testing/bdd';
import { expect } from 'jsr:@std/expect';

import { Cli } from './mod.ts';
import { Flag } from './flag.ts';
import * as basic from './inputs/mod.ts';

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

describe('Cli', () => {
    describe('Type Check', () => {
        function checkType<Type>(v: Type) {
            (v);
        }

        it('str / strEnum - type checks', () => {
            () => {
                const args = new Cli()
                    .str('required')
                    .str('optional', { optional: true })
                    .str('default', { optional: true, defaultValue: 'a' })
                    .parse([]);

                checkType<string>(args.required);
                checkType<string | null>(args.optional);
                checkType<string>(args.default);
            }

            () => {
                const args = new Cli()
                    .strEnum('required', ['foo', 'bar'])
                    .strEnum('optional', ['foo', 'bar'], { optional: true })
                    .strEnum('default', ['foo', 'bar'], { optional: true, defaultValue: 'bar' })
                    // @ts-expect-error defaultValue does not match variant type
                    .strEnum('incorrectDefault', ['foo'], { defaultValue: 'baz' })
                    .parse([]);

                checkType<'foo' | 'bar'>(args.required);
                checkType<'foo' | 'bar' | null>(args.optional);
                checkType<'foo' | 'bar'>(args.default);
            }


            // @ts-expect-error variants must be a string array
            () => new Cli().strEnum('test', [1, 0]);
        });

        it('int / num - type checks', () => {
            () => {
                const args = new Cli()
                    .int('required')
                    .int('optional', { optional: true })
                    .int('default', { optional: true, defaultValue: 0 })
                    .parse([]);

                checkType<number>(args.required);
                checkType<number | null>(args.optional);
                checkType<number>(args.default);
            }

            () => {
                const args = new Cli()
                    .num('required')
                    .num('optional', { optional: true })
                    .num('default', { optional: true, defaultValue: 0 })
                    .parse([]);

                checkType<number>(args.required);
                checkType<number | null>(args.optional);
                checkType<number>(args.default);
            }
        });

        it('bool - type checks', () => {
            () => {
                const args = new Cli()
                    .bool('required')
                    .bool('optional', { optional: true })
                    .bool('default', { optional: true, defaultValue: false })
                    .parse([]);

                checkType<boolean>(args.required);
                checkType<boolean | null>(args.optional);
                checkType<boolean>(args.default);
            }
        });

        it('extended', () => {
            () => {
                const range = basic.range({ name: 'required' } as const);
                const rangeOpt = basic.range({ name: 'optional', optional: true } as const);
                const rangeDef = basic.range({ name: 'default', optional: true, defaultValue: [0, 1] } as const);
                const args = new Cli()
                    .add(range)
                    .add(rangeOpt)
                    .add(rangeDef)
                    .parse([]);

                checkType<[number, number]>(args.required);
                checkType<[number, number] | null>(args.optional);
                checkType<[number, number]>(args.default);
            }
        });
    });

    describe('str', () => {
        it('parses <name>=<value>', () => {
            expect(new Cli().str('test').parse(['--test=']).test).toBe('');
            expect(new Cli().str('test').parse(['--test=val']).test).toBe('val');
        });
        it('parses <name> <value>', () => {
            expect(new Cli().str('test').parse(['--test', 'val']).test).toBe('val');
        })
        it ('parses <name?>', () => {
            expect(new Cli().str('test', { optional: true }).parse([]).test).toBe(null);
            expect(new Cli().str('test', { optional: true }).parse(['--test']).test).toBe(null);
        });
        it ('parses <name?=default>', () => {
            expect(new Cli().str('test', { optional: true, defaultValue: 'x' }).parse([]).test).toBe('x');
            expect(new Cli().str('test', { optional: true, defaultValue: 'x' }).parse(['--test']).test).toBe('x');
        });
        it('throw when value missing', () => {
            expect(() => new Cli().str('test').parse([])).toThrow();
            expect(() => new Cli().str('test').parse(['--test'])).toThrow();
        });
    });

    describe('strEnum', () => {
        it('parses <name>=<value>', () => {
            expect(new Cli().strEnum('test', ['x']).parse(['--test=x']).test).toBe('x');
        });
        it('parses <name> <value>', () => {
            expect(new Cli().strEnum('test', ['val']).parse(['--test', 'val']).test).toBe('val');
        })
        it ('parses <name?>', () => {
            expect(new Cli().strEnum('test', ['x'], { optional: true }).parse([]).test).toBe(null);
            expect(new Cli().strEnum('test', ['x'], { optional: true }).parse(['--test']).test).toBe(null);
        });
        it ('parses <name?=default>', () => {
            expect(new Cli().strEnum('test', ['x'], { optional: true, defaultValue: 'x' }).parse([]).test).toBe('x');
            expect(new Cli().strEnum('test', ['x'], { optional: true, defaultValue: 'x' }).parse(['--test']).test).toBe('x');
        });
        it('throw when value missing', () => {
            expect(() => new Cli().strEnum('test', ['x', 'y']).parse([])).toThrow();
            expect(() => new Cli().strEnum('test', ['x', 'y']).parse(['--test'])).toThrow();
        });
        it('expects at least one enum variant', () => {
            expect(() => new Cli().strEnum('day', [])).toThrow();
        });
        it('expects enum variants that are non-empty strings', () => {
            expect(() => new Cli().strEnum('day', [''])).toThrow();
            expect(() => new Cli().strEnum('day', ['one', ''])).toThrow();
        });
        it('accespts non alphabetic variant', () => {
            const cli = new Cli().strEnum('test', ['0', 'a space', '│']);

            expect(cli.parse(['--test', '0'])).toEqual({ test: '0' });
            expect(cli.parse(['--test', 'a space'])).toEqual({ test: 'a space' });
            expect(cli.parse(['--test', '│'])).toEqual({ test: '│' });

            expect(() => {
                cli.parse(['--test', 'x']);
            }).toThrow();
        });
    });

    describe('int', () => {
        it('parses <name>=<value>', () => {
            expect(new Cli().int('test').parse(['--test=0']).test).toBe(0);
            expect(new Cli().int('test').parse(['--test=-1']).test).toBe(-1);
        });
        it('parses <name> <value>', () => {
            expect(new Cli().int('test').parse(['--test', '-10']).test).toBe(-10);
        })
        it ('parses <name?>', () => {
            expect(new Cli().int('test', { optional: true }).parse([]).test).toBe(null);
            expect(new Cli().int('test', { optional: true }).parse(['--test']).test).toBe(null);
        });
        it ('parses <name?=default>', () => {
            expect(new Cli().int('test', { optional: true, defaultValue: 0 }).parse([]).test).toBe(0);
            expect(new Cli().int('test', { optional: true, defaultValue: 0 }).parse(['--test']).test).toBe(0);
        });
        it('throw when value missing', () => {
            expect(() => new Cli().int('test').parse([])).toThrow();
            expect(() => new Cli().int('test').parse(['--test'])).toThrow();
        });
    });

    describe('bool', () => {
        it('true when flag present', () => {
            expect(new Cli().bool('test').parse(['--test']).test).toBe(true);
        });
        it('false when off flag present', () => {
            expect(new Cli().bool('test').parse(['--no-test']).test).toBe(false);
        });
        it('true when flag present - regardless of value', () => {
            expect(new Cli().bool('test').parse(['--test=false']).test).toBe(true);
            expect(new Cli().bool('test').parse(['--test=off']).test).toBe(true);
            expect(new Cli().bool('test').parse(['--test=0']).test).toBe(true);
        });
        it('false when off flag present - regardless of value', () => {
            expect(new Cli().bool('test').parse(['--no-test=false']).test).toBe(false);
            expect(new Cli().bool('test').parse(['--no-test=off']).test).toBe(false);
            expect(new Cli().bool('test').parse(['--no-test=0']).test).toBe(false);
        });
        it('null when optional and not provided', () => {
            expect(new Cli().bool('test', { optional: true }).parse([]).test).toBe(null);
        });
        it('default when optional and not provided', () => {
            expect(new Cli().bool('test', { defaultValue: true }).parse([]).test).toBe(true);
        });
    })

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
