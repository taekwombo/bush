import test from 'ava';
import { p2 } from '../point.js';
import { s2 } from '../segment.js';
import { PSPos, pointSide, normal, contains } from './helpers.js';
import type { Point2 } from '../point.js';

test('pointSide - Colinear', (t) => {
    const a = p2(0, 0);
    const b = p2(0, 10);

    const cases = [
        p2(0, 20),  // out of the line segment
        p2(0, -20), // out of the line segment
        a,          // in the line segment
        b,          // in the line segment
    ];

    for (const c of cases) {
        t.is(pointSide(a, b, c), PSPos.Colinear);
    }
});

test('sadflkj', (t) => {
    t.is(PSPos.Right, pointSide(
        p2(100, 300),
        p2(100, 100),
        p2(74, 270),
    ));
});

{
    const a = p2(0, 0);
    const b = p2(0, 10);
    const cases = [
        p2(1, 0),
        p2(1, 10),
        p2(1, -1000),
        p2(10, 10),
    ];
    for (const c of cases) {
        test(`pointSide - ${a} ${b} ${c}`, (t) => {
            t.is(pointSide(a, b, c), PSPos.Right);
            t.is(pointSide(b, a, c), PSPos.Left);
        });
    }
}

test('normal', (t) => {
    const a = p2(0, 0);
    const b = p2(0, 10);

    const left = p2(-10, 0);
    const right = p2(10, 0);

    t.is(true, left.eq(normal(a, b, true)));
    t.is(pointSide(a, b, left), PSPos.Left);
    t.is(true, right.eq(normal(a, b, false)));
    t.is(pointSide(a, b, right), PSPos.Right);
});

{
    const segment = s2(p2(0, 0), p2(0, 10));
    const cases: [Point2, boolean][] = [
        ...new Array(10).fill(0).map((_, i) => [p2(0, i), true] as [Point2, boolean]),
        [p2(0, -1), false],
        [p2(1, 1), false],
        [p2(1, 0), false],
        [p2(0, 15), false],
    ];

    for (const [c, expected] of cases) {
        test(`contains - ${segment} ${c}`, (t) => {
            t.is(expected, contains(segment, c));
        });
    }
}
