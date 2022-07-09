import { Canvas, Line2, p2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    const { width: w, height: h } = img;

    new Line2([
        p2(50, 50),
        p2(50, h - 50),
        p2(w - 50, h - 50),
        p2(w - 50, 50),
        p2(w * 0.5, h * 0.5),
        p2(50, 50),
        p2(w - 50, 50),
    ]).draw(img);
});
