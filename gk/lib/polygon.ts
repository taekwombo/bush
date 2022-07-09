import { Line2, drawLine } from './line.js';
import { p2 } from './point.js';
import type { Draw, Fill, ImageDataExt } from './types.js';
import type { Color } from './color.js';
import type { Point2 } from './point.js';


type ActiveEdge = { edge: Line2; x: number };

export class Polygon2 implements Draw, Fill {
    public vertices: Point2[];
    public color?: Color;

    public constructor(vertices: Point2[], color?: Color) {
        this.vertices = vertices;
        this.color = color;
    }

    public draw(image: ImageDataExt): this {
        const points = [...this.vertices, this.vertices[0]];

        new Line2(points, this.color).draw(image);

        return this;
    }

    public fill(image: ImageDataExt, color?: Color): this {
        const edges = this.fillEdges();

        let activeEdges: ActiveEdge[] = [];
        let y = edges[0].points[0].y;

        do {
            activeEdges = this.getActiveEdges(activeEdges, edges, y);

            for (let i = 1; i < activeEdges.length; i += 2) {
                const e1 = activeEdges[i - 1];
                const e2 = activeEdges[i];

                drawLine(image, p2(e1.x, y), p2(e2.x, y), color || this.color);
            }

            y++;

            activeEdges = activeEdges.filter(({ edge }) => edge.points[1].y !== y);
        } while (activeEdges.length > 0);

        return this;
    }

    protected getActiveEdges(this: Polygon2, activeEdges: ActiveEdge[], edges: Line2[], y: number): ActiveEdge[] {
        function intersectionAtY({ points: [start, end] }: Line2, y: number): number {
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
            .filter((edge) => edge.points[0].y === y)
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

    protected fillEdges(): Line2[] {
        const edges: Line2[] = [];

        for (let i = 1; i < this.vertices.length; i++) {
            edges.push(new Line2([
                this.vertices[i - 1],
                this.vertices[i],
            ]));
        }

        edges.push(new Line2([
            this.vertices[0],
            this.vertices[this.vertices.length - 1]
        ]));

        return edges
            // Remove horizontal edges.
            .filter((edge) => edge.points[0].y !== edge.points[1].y)
            // Make sure edge direction points up.
            .map((line)=> {
                const [s, e] = line.points;

                if (s.y > e.y) {
                    return line.invert();
                }

                return line;
            })
            // Sort edges so that first point Y coordinate is ascending.
            .sort((a, b) => {
                const [as] = a.points;
                const [bs] = b.points;

                return as.y - bs.y;
            });
    }
}

