import { p2, Segment2, Canvas, nn } from '../../lib/mod.js';
import { Line, Plane, Vector, int } from '../../lib/3d/mod.js';

// Project a tetrahedron ABCD on a plane that includes point (0, 0, 0) and has a normal (0, 0, 1).
// A = (10, 10, 5)
// B = (12, 15, 5)
// C = (15, 10, 5)
// D = (15, 10, 10)

const canvas = Canvas.create2(500, 500);

canvas.drawCb((img) => {
    const dir = Vector.new(0, 0, 1);
    const plane = Plane.new(Vector.new(0, 0, 0), dir);
    const a = Vector.new(10, 10, 5); // base
    const b = Vector.new(12, 15, 5); // base
    const c = Vector.new(15, 10, 5); // base
    const d = Vector.new(13, 12, 10); // top

    const mv = Vector.new(50, 50, 50);
    // For each line that crosses one of the points (ABCD) with direction (0, 0, 1)
    // find the intersection point with the plane.
    // This means, all lines are normals to the plane.
    const ap = nn(int.line_plane(Line.new(a, dir), plane)).mul(20).add(mv);
    const bp = nn(int.line_plane(Line.new(b, dir), plane)).mul(20).add(mv);
    const cp = nn(int.line_plane(Line.new(c, dir), plane)).mul(20).add(mv);
    const dp = nn(int.line_plane(Line.new(d, dir), plane)).mul(20).add(mv);

    const p = (v: Vector) => p2(v.x, v.y);

    // For now, let's skip z coordinate.
    // Draw base.
    Segment2.pipeDraw(img, [
        p(ap),
        p(bp),
        p(cp),
        p(ap),
    ]);

    // Draw sides.
    Segment2.pipeDraw(img, [
        [p(dp), p(ap)],
        [p(dp), p(bp)],
        [p(dp), p(cp)],
    ]);
});
