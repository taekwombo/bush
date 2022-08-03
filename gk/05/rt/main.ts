import { Canvas, Color, Segment2, s2, p2 } from '../../lib/mod.js';
import { Line, Vector, Triangle, int, Plane } from '../../lib/3d/mod.js';

const width = 400;
const height = 400;
const dist = 100;

const camera = Vector.new(width / 2, height / 2, dist);

const canvas = Canvas.create2(width, height);

// Z = <0; -240>
const t = Triangle.new(
    Vector.new(130, 50, -50),
    Vector.new(360, 100, -50),
    Vector.new(210, 200, -50),
);

window.addEventListener('keypress', (event) => {
    if (event.key === 'u') {
        t.a.z = Math.min(0, Math.max(t.a.z, t.a.z + 10));
        t.b.z = Math.min(0, Math.max(t.b.z, t.b.z + 10));
        t.c.z = Math.min(0, Math.max(t.c.z, t.c.z + 10));
    } else if (event.key === 'd') {
        t.a.z = Math.max(-240, Math.min(t.a.z, t.a.z - 10));
        t.b.z = Math.max(-240, Math.min(t.b.z, t.b.z - 10));
        t.c.z = Math.max(-240, Math.min(t.c.z, t.c.z - 10));
    }

    redraw();
});

function redraw() {
    canvas.clear().drawCb((img, draw) => {
        for (let x = 0; x < width; x++) {
            for (let y = 0; y < height; y++) {
                const ray = createRay(camera, x, y);
                const v = int.line_triangle(ray, t);
                if (v) {
                    img.drawPoint(x, y, new Color(255 + t.a.z, 255 + t.a.z, 255 + t.a.z, 255));
                }
            }
        }
    });
};

function createRay(c: Vector, x: number, y: number): Line {
    const dir = Vector.new(
        x - c.x,
        y - c.y,
        -c.z,
    );

    return Line.new(c.clone(), dir);
}

redraw();
