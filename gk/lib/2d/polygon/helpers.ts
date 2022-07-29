import { Segment2, s2 } from '../segment.js';
import type { Point2 } from '../point.js';
import type { Polygon2 } from '../polygon.js';

export function contains(polygon: Polygon2, point: Point2): boolean {
    const p1 = point.clone();
    p1.x = point.x + 1;

    const ray = s2(point, p1);

    let ic = 0;
    let ip = polygon.vertices.length - 1;
    let intersections = 0;

    for (; ic < polygon.vertices.length; ip = ++ic - 1) {
        const a = polygon.vertices[ip];
        const b = polygon.vertices[ic];

        if (point.eq(a) || point.eq(b)) {
            return true;
        }

        const maxX = Math.max(a.x, b.x);

        if (maxX >= ray.end.x) {
            ray.end.x = maxX + 1;
        }

        const edge = s2(a, b);
        const intersection = Segment2.intersection.def(ray, edge);

        if (intersection) {
            intersections += 1;
        }
    }

    return intersections % 2 !== 0;
}
