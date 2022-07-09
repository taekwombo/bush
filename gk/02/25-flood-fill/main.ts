import { Circle2, Canvas, p2, Color } from '../../lib/mod.js';

const canvas = Canvas.create2(400, 400);

canvas.drawCb((img) => {
    new Circle2(p2(200, 200), 50).draw(img);
    img.floodFill(200, 200, Color.Blue);
});

