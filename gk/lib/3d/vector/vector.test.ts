import test from 'ava';
import { Vector } from './mod.js';

test('invalid', (t) => {
    t.throws(() => Vector.new(1, 1, 2, 0));
    t.throws(() => Vector.new(1, 1, -Infinity));
    t.throws(() => Vector.new(Infinity, 1, 1));
    t.throws(() => Vector.new(NaN, 1, 1));
});

const addCases = [
    [Vector.new(1, 1, 1), Vector.new(2, 2, 2), Vector.new(3, 3, 3)],
    [Vector.new(1, 1, 1), Vector.new(1, 1, 1), Vector.new(2, 2, 2)],
];

for (const [a, b, r] of addCases) {
    test(`${a} + ${b}`, (t) => {
        const c = a.add(b);

        t.is(c.x, r.x);
        t.is(c.y, r.y);
        t.is(c.z, r.z);
    });
}

const dotCases = [
    [Vector.new(1, 0, 0), Vector.new(0, 1, 1), 0],
    [Vector.new(1, 0, 0), Vector.new(0, -1, 1), 0],
    [Vector.new(1, 1, 1), Vector.new(1, 1, 1), 3],
] as const;

for (const [a, b, r] of dotCases) {
    test(`${a} ∙ ${b}`, (t) => {
        t.is(r, a.dot(b));
    });
}

test('.normalize()', (t) => {
    t.deepEqual(Vector.new(1, 0, 0), Vector.new(2, 0, 0).normalize());
    t.deepEqual(Vector.new(-1, 0, 0), Vector.new(-2, 0, 0).normalize());
});

test('.mul()', (t) => {
    t.deepEqual(
        Vector.new(2, 2, 2),
        Vector.new(1, 1, 1).mul(2),
    );
});

test('.mag()', (t) => {
    t.is(1, Vector.new(1, 0, 0).mag());
    t.is(1, Vector.new(-1, 0, 0).mag());
    t.is(20, Vector.new(20, 0, 0).mag());

    const v = Vector.new(1, 0, 0);
    v.mag(5);

    t.is(v.x, 5);
});

