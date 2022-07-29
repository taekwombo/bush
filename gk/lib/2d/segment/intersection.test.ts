import test from 'ava';
import { p2 } from '../point.js';
import { s2 } from '../segment.js';
import { def, cross } from './intersection.js';

const cases = [
    [p2(0, 0), p2(10, 0), p2(5, -5), p2(5, 15)],
    [p2(0, 0), p2(10, 0), p2(0, -5), p2(5, 15)],
    [p2(0, 5), p2(10, 0), p2(0, -5), p2(5, 15)],
    [p2(0, 0), p2(0, 1), p2(0, 0), p2(1, 0)],
    [p2(0, 0), p2(0, 10), p2(0, 0), p2(0, -10)],
];

for (const [a, b, c, d] of cases) {
    const sa = s2(a, b);
    const sb = s2(c, d);

    test(`${sa.debug()} & ${sb.debug()}`, (t) => {
        t.deepEqual(def(sa, sb), cross(sa, sb));
    });
}
