import { Canvas, Cyclic, p2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    new Cyclic(
        p2(200, 200),
        50,
        20,
        20,
    ).draw(img);
    new Cyclic(
        p2(200, 200),
        50,
        20,
        20,
        true,
    ).draw(img);
});


