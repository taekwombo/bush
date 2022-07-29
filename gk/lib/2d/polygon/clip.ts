import { nn } from '../../utils.js';
import { Polygon2 } from '../polygon.js';
import { Segment2, s2 } from '../segment.js';
import { contains } from './helpers.js';
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
        const { PSPos, pointSide } = Segment2.helpers;
        return PSPos.Right > pointSide(a, b, c);
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

/**
 * Used in Wiler-Atherton algorithm.
 * Stores:
 * - intersection point,
 * - direction of the intersecting segment with window edge,
 * - starting point of the window edge,
 * - starting point of the polygon edge
 */
type WAInts = {
    entering: boolean;
    windowStart: Point2;
    windowDist: number;
    polygonStart: Point2;
    polygonDist: number;
    intersection: Point2;
};
/**
 * Stores vertices of a polygon and intersection points in between
 * them if any.
 *
 * Example:
 * Polygon ABCD with an intersection I0 in between A and B would have
 * the following representation.
 * ```
 * [A, I0, B, C, D]
 * ```
 */
type WAVertexNode = {
    type: 'vertex',
    index: number;
    vertex: Point2;
} | {
    type: 'intersection',
    windowIndex: number;
    polygonIndex: number;
    int: WAInts;
};

/**
 * 3.2.1. [resources/grafika_komputerowa.pdf]
 *
 * Weiler-Atherton algorithm.
 *
 * Creates a new, clipped Polygon2 instance or returns null if the polygon
 * is outside of clipping window.
 */
export function WA(polygon: Polygon2, window: Polygon2): Polygon2 | null {
    const ints: WAInts[] = [];

    const { PSPos, pointSide } = Segment2.helpers;

    let ic = 0; // window edge end point index
    let ip = window.vertices.length - 1; // window edge start point index
    for (; ic < window.vertices.length; ip = ++ic - 1) {
        const pwv = window.vertices[ip];
        const cwv = window.vertices[ic];

        let jc = 0; // polygon edge end point index
        let jp = polygon.vertices.length - 1; // polygon edge start point index
        for (; jc < polygon.vertices.length; jp = ++jc - 1) {
            const ppv = polygon.vertices[jp];
            const cpv = polygon.vertices[jc];

            const intersection = Segment2.intersection.def(s2(pwv, cwv), s2(ppv, cpv));

            // If there is no intersection
            if (!intersection) {
                continue;
            }

            ints.push({
                entering: pointSide(pwv, cwv, cpv) !== PSPos.Right,
                intersection,
                windowStart: pwv,
                windowDist: pwv.distance(intersection),
                polygonStart: ppv,
                polygonDist: ppv.distance(intersection),
            });
        }
    }

    if (ints.length === 0) {
        // If there are no intersection then either:
        // - polygon is inside window
        // - window is inside polygon
        // So, if one edge of a polygon, has all edges of the window on the right then
        // the window is inside polygon.
        const pa = polygon.vertices[0];
        const pb = polygon.vertices[1];
           
        let ic = 0;
        let ip = window.vertices.length - 1;

        let xGt = true; // Polygon edge has greater x value than all window edges.
        let xLt = true; // Polygon edge has smaller x value than all window edges.
        let yLt = true; // Polygon edge has smaller y value than all window edges.
        let yGt = true; // Polygon edge has greater y value than all window edges.

        for (; ic < window.vertices.length; ip = ++ic - 1) {
            const wa = window.vertices[ip]; 
            const wb = window.vertices[ic];
            
            xGt = xGt && (pa.x >= wa.x && pb.x >= wb.x);
            xLt = xLt && (pa.x <= wa.x && pb.x <= wb.x);
            yLt = yLt && (pa.y <= wa.y && pb.y <= wb.y);
            yGt = yGt && (pa.y >= wa.y && pb.y >= wb.y);
        }

        if (xGt || xLt || yLt || yGt) {
            return window.clone();
        }

        return polygon.clone();
    }

    const windowGraph: WAVertexNode[] = [];

    for (let i = 0; i < window.vertices.length; i++) {
        const vertex = window.vertices[i];

        windowGraph.push({
            type: 'vertex',
            vertex,
            index: windowGraph.length,
        });

        const intersections = ints
            .filter(({ windowStart }) => windowStart === vertex)
            .sort((a, b) => a.windowDist - b.windowDist);

        for (let j = 0; j < intersections.length; j++) {
            const v = intersections[j];
            windowGraph.push({
                int: v,
                type: 'intersection',
                windowIndex: windowGraph.length,
                polygonIndex: NaN,
            });
        }
    }

    const polygonGraph: WAVertexNode[] = [];

    for (let i = 0; i < polygon.vertices.length; i++) {
        const vertex = polygon.vertices[i];

        polygonGraph.push({
            vertex,
            type: 'vertex',
            index: polygonGraph.length,
        });

        const intersections = ints
            .filter(({ polygonStart }) => polygonStart === vertex)
            .sort((a, b) => a.polygonDist - b.polygonDist);

        for (let j = 0; j < intersections.length; j++) {
            const v = intersections[j];
            const winInt = windowGraph.find((e) => e.type === 'intersection' && e.int === v)!;

            (winInt as { polygonIndex: number }).polygonIndex = polygonGraph.length;

            polygonGraph.push(winInt);
        }
    }

    const resultVertices: Point2[] = [];
    const visited: Set<WAVertexNode> = new Set();
    // Find index of first entering intersection point.
    // If none, then first index of polygon vertex inside window.
    let idx = polygonGraph.findIndex((p) => p.type === 'intersection' && p.int.entering)
        || polygon.vertices.findIndex((p) => contains(window, p));
    let v = polygonGraph[idx];
    let g = polygonGraph;

    while (!visited.has(v)) {
        visited.add(v);

        if (v.type === 'intersection') {
            resultVertices.push(v.int.intersection);

            if (v.int.entering && g === windowGraph) {
                g = polygonGraph;
                idx = v.polygonIndex;
            } else if (!v.int.entering && g === polygonGraph) {
                g = windowGraph;
                idx = v.windowIndex;
            }
        } else {
            resultVertices.push(v.vertex);
        }


        idx = (idx + 1) % g.length;;
        v = g[idx];
    }

    return new Polygon2(resultVertices, polygon.color);
}
