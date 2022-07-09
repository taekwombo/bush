import { Canvas, Circle2, p2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    new Circle2(
        p2(200, 200),
        40,
    ).draw(img);

    new Circle2(
        p2(200, 200),
        80,
    ).draw(img);
});
