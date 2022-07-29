import { Canvas, Polygon2, p2, Color } from '../../lib/mod.js';
import type { Img } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);
// @ts-ignore
window.c = canvas;

function drawSH(img: Img): void {
    const xMin = 100;
    const xMax = 300;
    const yMin = 100;
    const yMax = 300

    const window = new Polygon2([
        p2(xMin, yMin),
        p2(xMax, yMin),
        p2(xMax, yMax),
        p2(xMin, yMax)
    ], Color.Yellow).draw(img).fill(img);

    const p = new Polygon2([
        p2(82, 150),
        p2(125, 142),
        p2(148, 120),
        p2(115, 56),
        p2(197, 59),
        p2(203, 120),
        p2(245, 124),
        p2(285, 47),
        p2(284, 145),
        p2(325, 179),
        p2(269, 199),
        p2(347, 274),
        p2(261, 256),
        p2(225, 321),
        p2(180, 223),
        p2(135, 340),
        p2(79, 234),
    ], Color.White);

    p.draw(img);

    p.fill(img, Color.Teal);

    const clipped = Polygon2.clip.SH(p, window);

    if (clipped) {
        clipped.color = Color.Lime;
        clipped.draw(img).fill(img);
    }
}

function drawWA(img: Img): void {
    const xMin = 100;
    const xMax = 300;
    const yMin = 100;
    const yMax = 300

    const window = new Polygon2([
        p2(xMin, yMin),
        p2(xMax, yMin),
        p2(xMax, yMax),
        p2(xMin, yMax)
    ], Color.Yellow).draw(img).fill(img);

    const p = new Polygon2([
        p2(82, 150),
        p2(125, 142),
        p2(148, 120),
        p2(115, 56),
        p2(197, 59),
        p2(203, 120),
        p2(245, 124),
        p2(285, 47),
        p2(284, 145),
        p2(325, 179),
        p2(269, 199),
        p2(347, 274),
        p2(261, 256),
        p2(225, 321),
        p2(180, 223),
        p2(135, 340),
        p2(79, 234),
    ], Color.White);

    // TODO:
    // Add examples when window is inside polygon.
    // Add examples when window is outside polygon.
    const clipped = Polygon2.clip.WA(p, window);

    if (clipped) {
        clipped.color = Color.White;
        clipped.draw(img).fill(img, Color.White);
    }
}

async function drawCustom(canvas: Canvas): Promise<(img: Img) => void> {
    const polygon = new Polygon2(await Canvas.record.capture(canvas), Color.Gray);
    const window = new Polygon2(await Canvas.record.capture(canvas), Color.Lime);

    return (img: Img) => {
        polygon.draw(img).fill(img);
        window.draw(img).fill(img);

        const clipped = Polygon2.clip.WA(polygon, window);

        if (clipped) {
            clipped.color = Color.Red;

            clipped.draw(img).fill(img);
        }
    };
}

const fns = [
    { type: 'sync', fn: drawSH },
    { type: 'sync', fn: drawWA },
    { type: 'async', fn: drawCustom },
] as const;

let i = 0;

async function drawNext() {
    canvas.clear();

    const it = fns[i];

    if (it.type === 'async') {
        window.removeEventListener('click', drawNext);
        canvas.drawCb(await it.fn(canvas));
        window.addEventListener('click', drawNext);
    } else {
        canvas.drawCb(it.fn);
    }
    
    i = (i + 1) % fns.length;
}

drawNext();

window.addEventListener('click', drawNext);

