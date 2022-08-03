import test from 'ava';
import { Mat4 } from './mod.js';

test('transpose', (t) => {
    const l = Mat4.new([
        1, 2, 3, 4,
        4, 3, 2, 1,
        1, 2, 3, 4,
        4, 3, 2, 1,
    ]);

    const r = Mat4.new([
        1, 4, 1, 4,
        2, 3, 2, 3,
        3, 2, 3, 2,
        4, 1, 4, 1,
    ]);

    t.deepEqual(l.v, r.transpose().v);
    t.deepEqual(r.v, l.transpose().v);
});
