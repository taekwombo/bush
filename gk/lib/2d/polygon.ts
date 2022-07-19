import { Segment2, drawSegment } from './segment.js';
import { p2 } from './point.js';
import type { Draw, Fill, ImageDataExt } from '../types.js';
import type { Color } from '../color.js';
import type { Point2 } from './point.js';


type ActiveEdge = { edge: Segment2; x: number };

export class Polygon2 implements Draw, Fill {
    public vertices: Point2[];
    public color?: Color;

    public constructor(vertices: Point2[], color?: Color) {
        this.vertices = vertices;
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        const points = [...this.vertices, this.vertices[0]];

        for (let i = 1; i < points.length; i++) {
            const pp = points[i - 1];
            const pc = points[i];

            new Segment2(pp, pc).draw(image);
        }

        return this;
    }

    public fill(image: ImageDataExt, color?: Color): this {
        const edges = this.fillEdges();

        let activeEdges: ActiveEdge[] = [];
        let y = edges[0].start.y;

        do {
            activeEdges = this.getActiveEdges(activeEdges, edges, y);

            for (let i = 1; i < activeEdges.length; i += 2) {
                const e1 = activeEdges[i - 1];
                const e2 = activeEdges[i];

                drawSegment(image, p2(e1.x, y), p2(e2.x, y), color || this.color);
            }

            y++;

            activeEdges = activeEdges.filter(({ edge }) => edge.end.y !== y);
        } while (activeEdges.length > 0);

        return this;
    }

    protected getActiveEdges(this: Polygon2, activeEdges: ActiveEdge[], edges: Segment2[], y: number): ActiveEdge[] {
        function intersectionAtY({ start, end }: Segment2, y: number): number {
            let dx: number = end.x - start.x;

            // Vertical line.
            if (dx === 0) {
                return start.x;
            }

            let dy: number = end.y - start.y

            // Should never be the case, since all horizontal lines
            // are filtered out.
            if (dy === 0) {
                throw new Error('Unreachable');
            }

            dx = dy / dx;
            dy = start.y - start.x * dx;

            return (y - dy) / dx;
        }

        const active: ActiveEdge[] = edges
            .filter((edge) => edge.start.y === y)
            // Attach x coordinate of intersection at Y coordinate.
            .map((edge) => ({
                edge,
                x: intersectionAtY(edge, y),
            }))
            .concat(activeEdges.map((e) => {
                e.x = intersectionAtY(e.edge, y);
                return e;
            }))
            // Sort in ascending order of x coordinate of intersection.
            .sort((a, b) => a.x - b.x);

        return active;
    }

    protected fillEdges(): Segment2[] {
        const edges: Segment2[] = [];

        for (let i = 1; i < this.vertices.length; i++) {
            edges.push(new Segment2(
                this.vertices[i - 1],
                this.vertices[i],
            ));
        }

        edges.push(new Segment2(
            this.vertices[0],
            this.vertices[this.vertices.length - 1]
        ));

        return edges
            // Remove horizontal edges.
            .filter((edge) => edge.start.y !== edge.end.y)
            // Make sure edge direction points up.
            .map((segment)=> {
                const { start, end } = segment;

                if (start.y > end.y) {
                    return segment.invert();
                }

                return segment;
            })
            // Sort edges so that first point Y coordinate is ascending.
            .sort(({ start: a }, { start: b }) => a.y - b.y);
    }
}

