import {
    Polygon2,
    Canvas,
    p2,
    Color,
} from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    new Polygon2([
        p2(50, 120),
        p2(10, 80),
        p2(50, 80),
    ]).draw(img).fill(img);

    new Polygon2([
        p2(100, 100),
        p2(60, 200),
        p2(100, 150),
        p2(140, 200),
    ]).draw(img).fill(img, Color.Red);

    new Polygon2([
        p2(300, 200),
        p2(340, 300),
        p2(240, 240),
        p2(360, 240),
        p2(260, 300),
    ]).draw(img).fill(img, Color.Blue);
});

