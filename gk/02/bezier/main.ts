import { Bezier2, b4, Canvas, p2 } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    b4(
        p2(100, 100),
        p2(200, 200),
        p2(300, 300),
        p2(300, 100),
        img,
    );

    new Bezier2([
        p2(50, 120),
        p2(10, 80),
        p2(50, 80),
        p2(150, 180),
        p2(250, 180),
        p2(250, 280),
        p2(300, 340),
    ]).draw(img);
});



