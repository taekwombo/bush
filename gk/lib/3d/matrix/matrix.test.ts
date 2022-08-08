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

test('inverse', (t) => {
    const l = Mat4.new([
        2, 0, 0, 0,
        0, 2, 0, 0,
        0, 0, 2, 0,
        0, 0, 0, 1,
    ]);

    const r = Mat4.new([
        0.5, 0, 0, 0,
        0, 0.5, 0, 0,
        0, 0, 0.5, 0,
        0, 0, 0, 1,
    ]);

    t.deepEqual(l.inverse().v, r.v);
    t.deepEqual(r.inverse().v, l.v);
});

test('mul', (t) => {
    const translate = Mat4.translate([0, 4, 0]);
    const scale = Mat4.scale(10);

    t.notDeepEqual(translate.mul(scale), scale.mul(translate));
    t.deepEqual(translate.mul(scale), Mat4.new([
        10, 0, 0, 0,
        0, 10, 0, 0,
        0, 0, 10, 0,
        0, 40, 0, 1,
    ]));
    t.deepEqual(scale.mul(translate), Mat4.new([
        10, 0, 0, 0,
        0, 10, 0, 0,
        0, 0, 10, 0,
        0, 4, 0, 1,
    ]));
    t.deepEqual(Mat4.mul({ translate, scale }), Mat4.new([
        10, 0, 0, 0,
        0, 10, 0, 0,
        0, 0, 10, 0,
        0, 4, 0, 1,
    ]));
});
