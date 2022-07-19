import { Canvas, clipSC, s2, p2, Color } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    { // Draw a line from the origin to (100, 100) clipped to (50, 50) from the origin.
        clipSC(s2(p2(0, 0), p2(100, 100), Color.Aqua), { x: [0, 50], y: [0, 50] })?.draw(img);
    }
});
