import { Canvas, clipLB, clipSC, Segment2, s2, p2, Color } from '../../lib/mod.js';
import type { Range2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    const xMin = 100;
    const xMax = 200;
    const yMin = 100;
    const yMax = 150;
    const r: Range2 = { x: [xMin, xMax], y: [yMin, yMax] };

    // Draw area of the clip window.
    Segment2.pipeDraw(
        img,
        [
            p2(xMin, yMin),
            p2(xMax, yMin),
            p2(xMax, yMax),
            p2(xMin, yMax),
            p2(xMin, yMin),
        ],
        Color.Gray,
    );

    {
        const s = s2(p2(0, 0), p2(300, 300), Color.Aqua);
        clipSC(s, r)?.draw(img);
    }

    {
        const s = s2(p2(120, 100), p2(120, 200), Color.Fuchsia);
        clipLB(s, r)?.draw(img);
    }

});
