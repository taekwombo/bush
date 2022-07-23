import { nn } from '../../utils.js';
import { Polygon2 } from '../polygon.js';
import { Segment2, s2 } from '../segment.js';
import type { Point2 } from '../point.js';

/**
 * 3.2.1. [resources/grafika_komputerowa.pdf]
 *
 * Sutherland-Hodgman algorithm.
 *
 * Creates a new, clipped Polygon2 instance or returns null if the polygon
 * is outside of clipping window.
 */
export function SH(polygon: Polygon2, window: Polygon2): Polygon2 | null {
    /** Intersection of AB and CD line segments. */
    const intersection = (a: Point2, b: Point2, c: Point2, d: Point2): Point2 | null => {
        return Segment2.intersection.def(s2(a, b), s2(c, d));
    };

    const inside = (a: Point2, b: Point2, c: Point2): boolean => {
        return (b.x - a.x) * (c.y - a.y) > (b.y - a.y) * (c.x - a.x);
    };

    let outVertices: Point2[] = polygon.clone().vertices;

    // clip polygon edge start
    let ces = window.vertices[window.vertices.length - 1];
    for (let ci = 0; ci < window.vertices.length; ci++) {
        // clip polygon edge end
        const cee = window.vertices[ci];

        const vertices = outVertices;
        outVertices = [];

        if (vertices.length === 0) {
            break;
        }

        // polygon edge start point
        let sp = vertices[vertices.length - 1];
        let spIn = inside(ces, cee, sp);
        for (let i = 0; i < vertices.length; i++) {
            // polygon edge end point
            const ep = vertices[i];
            const epIn = inside(ces, cee, ep);

            if (spIn) {
                if (epIn) {
                    outVertices.push(ep);
                } else {
                    outVertices.push(nn(intersection(ces, cee, sp, ep)));
                }
            } else if (epIn) {
                outVertices.push(nn(intersection(ces, cee, sp, ep)));
                outVertices.push(ep);
            }

            sp = ep;
            spIn = epIn;
        }

        ces = cee;
    }

    if (outVertices.length === 0) {
        return null;
    }

    return new Polygon2(outVertices, polygon.color);
}
