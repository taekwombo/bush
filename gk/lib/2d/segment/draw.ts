import { Color } from '../../color.js';
import type { Point2 } from '../point.js';
import type { Img } from '../../img.js';

export function drawSegment(image: Img, start: Point2, end: Point2, color?: Color): void {
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

    // Drawing along X asis on each step.
    if (deltaX > deltaY) {
        decision = -deltaX;

        while (x0 !== x1) {
            image.drawPoint(x0, y0, color);
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
            image.drawPoint(x0, y0, color);
            decision += 2 * deltaX;

            if (decision > 0) {
                x0 += stepX;
                decision -= 2 * deltaY;
            }

            y0 += stepY;
        }
    }
}

