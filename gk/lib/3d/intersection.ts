import type { Plane } from './plane/mod.js';
import type { Line } from './line/mod.js';
import type { Vector } from './vector/mod.js';
import type { Triangle } from './triangle/mod.js';

/**
 * Calculate the intersection point of a line and a plane.
 *
 * Line:  l₀ + tl⃗
 * Plane: (p - p₀) ∙ n⃗ = 0
 *
 * https://www.wikiwand.com/en/Line%E2%80%93plane_intersection
 *
 * The formula for t is:
 * t = (p₀ - l₀) ∙ n⃗ / (l⃗ ∙ n⃗)
 *
 * p₀ - l₀: a vector from a point on a line to a plane
 *
 * Or:
 *
 * Plane: ax + by + cz + d = 0
 * Line: l₀ + tl⃗
 *
 * l₀ = (x₀, y₀, z₀)
 * l⃗ = (i, j, k)
 *
 * a(x₀ + it) + b(y₀ + jt) + c(z₀ + kt) + d = 0
 */
export function line_plane(l: Line, p: Plane): Opt<Vector> {
    const { point: l0, direction: lv } = l;
    const { point: p0, normal: nv } = p;

    // Check if normal of the plane is perpendicular to the line direction.
    // If so, then there is no intersection point.
    // Line is either parallel to the plane or is on a plane.
    const dotLP = lv.dot(nv);

    if (dotLP === 0) {
        return null;
    }

    // (p₀ - l₀)
    const point = p0.clone().sub(l0);
    // t = (p₀ - l₀) ∙ n⃗ / (l⃗ ∙ n⃗)
    const t = point.dot(nv) / dotLP;

    return l0.clone().add(lv.clone().mul(t));
}

/**
 * Calculate the intersection point of a line and a triangle.
 *
 * https://gdbooks.gitbooks.io/3dcollisions/content/Chapter4/point_in_triangle.html
 */
export function line_triangle(l: Line, t: Triangle): Opt<Vector> {
    const p = line_plane(l, { normal: t.n, point: t.a });

    if (!p) {
        return null;
    }

    const a = t.a.clone().sub(p);
    const b = t.b.clone().sub(p);
    const c = t.c.clone().sub(p);

    const u = b.cross(c);
    const v = c.cross(a);
    const w = a.cross(b);

    if (u.dot(v) <= 0) {
        return null;
    }

    if (v.dot(w) <= 0) {
        return null;
    }

    return p;
}
