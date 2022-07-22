import { Canvas, Line2, Segment2, s2, p2, Color } from '../../lib/mod.js';
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

    { // Draw lines inside the clip window using Sutherland-Cohen algorithm in AQUA color.
        [
            s2(p2(0, 0), p2(300, 300), Color.Aqua),
            s2(p2(0, 120), p2(200, 120), Color.Aqua),
            s2(p2(140, 100), p2(140, 200), Color.Aqua),
        ].map((s) => Segment2.clip.SC(s, r)?.draw(img));
    }

    { // Draw lines inside the clip window using Liang-Barsky algorithm in FUCHSIA color.
        [
            s2(p2(50, 0), p2(350, 300), Color.Fuchsia),
            s2(p2(120, 100), p2(120, 200), Color.Fuchsia),
            s2(p2(0, 140), p2(200, 140), Color.Fuchsia),
        ].map((s) => Segment2.clip.LB(s, r)?.draw(img));
    }

    { // Draw segment of clipped line using Skala algorithm in LIME color.
        [
            s2(p2(80, 80), p2(160, 200)),
            s2(p2(200, 50), p2(125, 300)),
        ].map((s) => {
            Line2.clip.SKALA(Line2.fromSegment(s, Color.Lime), r)?.draw(img);
        });
    }

    { // Draw clipped segment using DMVD in RED color.
        [
            s2(p2(50, 50), p2(250, 200), Color.Red),
            s2(p2(160, 100), p2(160, 200), Color.Red),
            s2(p2(0, 130), p2(200, 130), Color.Red),
        ].map((s) => Segment2.clip.DMVD(s, r)?.draw(img));
    }
});
