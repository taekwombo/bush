import { Canvas, Ellipsis2, p2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    new Ellipsis2(
        p2(200, 200),
        40,
        80,
    ).draw(img);

    new Ellipsis2(
        p2(200, 200),
        80,
        40,
    ).draw(img);
});

