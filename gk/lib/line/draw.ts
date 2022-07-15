import { Color } from '../color.js';
import type { Point2 } from '../point.js';
import type { ImageDataExt } from '../types.js';

export function drawLine(image: ImageDataExt, start: Point2, end: Point2, color?: Color): void {
    // http://www.algorytm.org/podstawy-grafiki/algorytm-bresenhama.html
    let deltaX = end.x - start.x;
    let deltaY = end.y - start.y;
    const stepX = deltaX < 0 ? -1 : 1; // Line is drawn from right to left then -1.
    const stepY = deltaY < 0 ? -1 : 1; // Line is drawn from top to bottom then -1.
    let { x: x0, y: y0 } = start;
    let { x: x1, y: y1 } = end;

    deltaX = Math.abs(Math.round(deltaX));
    deltaY = Math.abs(Math.round(deltaY));

    let decision: number = 0;
    let pxColor: (x: number, y: number) => Color | undefined = () => color;

    // Interpolate colors between start and end.
    if (!color && start.color && end.color && !start.color.eq(end.color)) {
        pxColor = createColorInterpolation(x0, x1, y0, y1, start.color, end.color);
    }

    // Drawing along X asis on each step.
    if (deltaX > deltaY) {
        decision = -deltaX;

        while (x0 !== x1) {
            image.drawPoint(x0, y0, pxColor(x0, y0));
            decision += 2 * deltaY;

            if (decision > 0) {
                y0 += stepY;
                decision -= 2 * deltaX;
            }

            x0 += stepX;
        }
    } else {
        decision = -deltaY;

        while (y0 !== y1) {
            image.drawPoint(x0, y0, pxColor(x0, y0));
            decision += 2 * deltaX;

            if (decision > 0) {
                x0 += stepX;
                decision -= 2 * deltaY;
            }

            y0 += stepY;
        }
    }
}

/**
 * Creates function that transitions color "sc" to "ec" while drawing a line at point
 * P = (x, y)
 */
function createColorInterpolation(xMin: number, xMax: number, yMin: number, yMax: number, sc: Color, ec: Color) {
    const range = {
        r: ec.r - sc.r,
        g: ec.g - sc.g,
        b: ec.b - sc.b,
        a: ec.a - sc.a,
    };
    const my = Math.abs(yMax - yMin);
    const mx = Math.abs(xMax - xMin);

    return (x: number, y: number): Color => {
        const dx = x - xMin;
        const dy = y - xMin;

        if (dx === 0 && dy === 0) {
            return sc;
        }

        let p: number;


        if (dx === 0) {
            p = dy / my;
        } else if (dy === 0) {
            p = dx / mx;
        } else {
            p = ((dy / my) + (dx / mx)) * 0.5;
        }

        return new Color(
            Math.round(sc.r + range.r * p),
            Math.round(sc.g + range.g * p),
            Math.round(sc.b + range.b * p),
            Math.round(sc.a + range.a * p),
        );
    };
}

