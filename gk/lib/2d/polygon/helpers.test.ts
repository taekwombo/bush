import test from 'ava';
import { Polygon2 } from '../polygon.js';
import { p2 } from '../point.js';
import { contains } from './helpers.js';

test('contains', (t) => {
    const polygon = new Polygon2([
        p2(0, 0),
        p2(10, 20),
        p2(20, 0),
    ]);

    for (const v of polygon.vertices) {
        t.is(true, contains(polygon, v));
    }

    t.is(true, contains(polygon, p2(10, 10)));
    t.is(false, contains(polygon, p2(-1, 0)));
});
