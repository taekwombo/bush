import { Canvas, Img, rad, Segment2, p2 } from '../../lib/mod.js';
import { Mat4, Vector, Triangle } from '../../lib/3d/mod.js';

// More info than in the book:
// https://www.scratchapixel.com
//
// TODO: create interactive example where the scene contains a cube.
// Cube position is fixed.
// Camera position is fixed.
// Screen position can be adjusted using input element.
//
// One part of the screen renders a scene.
// Second part of the screen renders a scene box, camera position and a viewing frustum.

const width = 500;
const height = 500;
const canvas = Canvas.create2(width, height);

const triangles = [
    Triangle.new(
        Vector.new(0.1, 0.1, -0.5),
        Vector.new(-0.1, 0.1, -0.5),
        Vector.new(-0.1, -0.1, -0.5),
    ),
    Triangle.new(
        Vector.new(0.1, 0.1, -0.5),
        Vector.new(0.1, -0.1, -0.5),
        Vector.new(-0.1, -0.1, -0.5),
    ),
    Triangle.new(
        Vector.new(-0.1, 0.1, -0.5),
        Vector.new(-0.1, -0.1, -0.5),
        Vector.new(-0.1, -0.1, -0.7),
    ),
    Triangle.new(
        Vector.new(-0.1, 0.1, -0.5),
        Vector.new(-0.1, 0.1, -0.7),
        Vector.new(-0.1, -0.1, -0.7),
    ),
    Triangle.new(
        Vector.new(0.1, 0.1, -0.5),
        Vector.new(0.1, -0.1, -0.5),
        Vector.new(0.1, -0.1, -0.7),
    ),
    Triangle.new(
        Vector.new(0.1, 0.1, -0.5),
        Vector.new(0.1, 0.1, -0.7),
        Vector.new(0.1, -0.1, -0.7),
    ),
];

const ry = Mat4.rotate({ y: rad(2) });
const t1 = Mat4.translate(Vector.new(0, 0, 0.6));
const t2 = Mat4.translate(Vector.new(0, 0, -0.6));

const r = t1.mul(ry).mul(t2);

// Convert coordinates to raster space.
const toRS = (v: number): number => {
    return (v + 1) * 0.5 * width;
};

function frame(img: Img) {
    for (const { a, b, c } of triangles) {
        a.t(r);
        b.t(r);
        c.t(r);

        const va = p2(toRS(a.x / -a.z), toRS(a.y / -a.z));
        const vb = p2(toRS(b.x / -b.z), toRS(b.y / -b.z));
        const vc = p2(toRS(c.x / -c.z), toRS(c.y / -c.z));
        
        Segment2.pipeDraw(img, [va, vb, vc, va]);
    }
}

function draw() {
    canvas.clear().drawCb(frame);
}

window.addEventListener('mousemove', draw);

