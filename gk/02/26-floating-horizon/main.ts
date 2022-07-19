import { Canvas, Color, drawSegment, p2, Point2, Segment2, s2 } from '../../lib/mod.js';
import type { ImageDataExt, Range2 } from '../../lib/mod.js';

const canvas = Canvas.create2(900, 900);

const w = 600, h = 600;
canvas.drawCb((img, put) => {
    drawFH(img, {
        put,
        xMin: -100,
        xMax: 100,
        yMin: -100,
        yMax: 100,
        zMax: 100,
        width: w,
        height: h,
        wx: 300,
        hx: 80,
        hz: 450,
        fn: (x: number, y: number) => -Math.sqrt(x ** 2 + Math.abs(y)) + 100,
    });
}, [100, 100, w + 1, h + 1]);

type FHOptions = {
    put: () => void;
    xMin?: number;
    yMin?: number;
    zMin?: number;
    xMax: number;
    yMax: number;
    zMax: number;
    width: number;
    height: number;
    wx: number;
    hx: number;
    hz: number;
    densX?: number;
    densY?: number;
    fn: (x: number, y: number) => number,
};

function drawFH(image: ImageDataExt, options: FHOptions): void {
    let {
        put,
        xMax,
        xMin = 0,
        yMax,
        yMin = 0,
        zMax,
        zMin = 0,
        width,
        height,
        wx,
        hx,
        hz,
        densX = 10,
        densY = 10,
        fn,
    } = options;

    xMax = Math.round(xMax);
    yMax = Math.round(yMax);
    zMax = Math.round(zMax);

    const a = wx / (xMin - xMax);
    const b = (width - wx) / (yMax - yMin);
    const c = 0;
    const d = (-a * xMax) - (b * yMin);
    const e = hx / (xMax - xMin);
    const f = (height - hx - hz) / (yMax - yMin);
    const g = hz / (zMin - zMax);
    const h = (-e * xMin) - (f * yMin) - (g * zMax);

    const p = (x: number, y: number, z: number) => p2(
        Math.floor((a * x) + (b * y) + (c * z) + d),
        Math.floor((e * x) + (f * y) + (g * z) + h),
    );

    const points: Point2[] = [];

    for (let y = yMin; y <= yMax; y += densY) {
        for (let x = xMin; x <= xMax; x += densX) {
            const z = Math.round(fn(x, y));

            points.push(p(x, y, z));
        }
    }

    const horizon = {
        up: [] as number[],
        dn: [] as number[],
    };

    for (let i = 0; i < width; i++) {
        horizon.up[i] = height;
        horizon.dn[i] = -1;
    }

    {
        const yellow = new Color(255, 255, 0, 155);
        const pink = new Color(255, 50, 255, 155);
        const green = new Color(180, 240, 220, 155);
        const range: Range2 = {
            x: [0, width],
            y: [0, height],
        };

        // Draw total drawing area
        Segment2.pipeDraw(image, [
            p2(0, 0),
            p2(width, 0),
            p2(width, height),
            p2(0, height),
            p2(0, 0)
        ]);
        const x0 = Math.max(xMin, 0);
        const y0 = Math.max(yMin, 0);
        const z0 = Math.max(zMin, 0);

        // Draw X axis
        s2(p(x0, y0, z0), p(xMax, y0, z0), green).extend(range).draw(image);
        // Draw Y axis
        s2(p(x0, y0, z0), p(x0, yMax, z0), pink).extend(range).draw(image);
        // Draw Z axis
        s2(p(x0, y0, z0), p(x0, y0, zMax), yellow).extend(range).draw(image);
        put();
    }

    type DP = typeof image.drawPoint;
    const drawPoint = ((original: DP): DP => {
        return function drawPoint(this: ImageDataExt, x: number, y: number): void {
            const up = horizon.up[x];
            const dn = horizon.dn[x];
            if (y > dn || y < up) {
                original.call(this, x, y);
            }
        };
    })(image.drawPoint);
    image.drawPoint = drawPoint;

    const updateHorizonMethod = function updateHorizon(this: ImageDataExt, x: number, y: number): void {
        if (y > horizon.dn[x]) {
            horizon.dn[x] = y;
        }
        if (y < horizon.up[x]) {
            horizon.up[x] = y;
        }
    };

    function updateHorizon (a: Point2, b: Point2): void {
        image.drawPoint = updateHorizonMethod;
        drawSegment(image, a, b);
        image.drawPoint = drawPoint;
    }

    const gridWidth = (xMax - xMin) / densX;

    for (let y = yMax; y >= yMin; y -= densY) {
        for (let x = xMax; x >= xMin; x -= densX) {
            const ix = (x - xMin) / densX;
            const iy = (y - yMin) / densY;
            const index = ix + iy + (iy * gridWidth);

            if (ix > 0) {
                const pa = points[index];
                const pb = points[index - 1];

                drawSegment(image, pa, pb);
                put();
            }

            if (iy > 0) {
                const pa = points[index];
                const pb = points[index - gridWidth - 1];

                drawSegment(image, pa, pb);
                put();
            }

            if (ix > 0) {
                updateHorizon(points[index], points[index - 1]);
            }

            if (iy > 0) {
                updateHorizon(points[index], points[index - gridWidth - 1]);
            }
        }
    }
}
