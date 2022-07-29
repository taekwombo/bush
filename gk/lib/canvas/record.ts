import type { Canvas } from '../canvas.js';
import { Segment2 } from '../2d/segment.js';
import { Point2, p2 } from '../2d/point.js';
import { Color } from '../color.js';

const dot = document.createElement('div');
dot.style.position = 'fixed';
dot.style.top = '0';
dot.style.left = '0';
dot.style.backgroundColor = 'red';
dot.style.borderRadius = '50%';
dot.style.width = '20px';
dot.style.height = '20px';

function print(line: Point2[]): void {
    let debug: string[] = [];
    let p2s: string[] = [];

    for (let point of line) {
        debug.push(point.debug());   
        p2s.push(`p2(${point.x}, ${point.y})`);
    }

    if (debug.length === 0) {
        return;
    }

    console.log(debug.join(', '));
    console.log(p2s.join(', '));
}

export function captureSync(canvas: Canvas, cb: (result: Point2[]) => void): void {
    const { width, height } = canvas;
    const { clientWidth: xMax, clientHeight: yMax } = canvas.canvas;
    console.log(width, height, xMax, yMax);
    const line: Point2[] = [];
    let recording = false;

    function keyPressHandler(event: KeyboardEvent) {
        if (event.key === 'r') {
            if (recording) {
                document.body.removeChild(dot);
                recording = false;
                canvas.canvas.removeEventListener('click', clickHandler);
                window.removeEventListener('keypress', keyPressHandler);
                print(line);
                cb(line);
            } else {
                document.body.appendChild(dot);
                recording = true;
                canvas.canvas.addEventListener('click', clickHandler);
            }

            return;
        }
    };

    function clickHandler(event: MouseEvent): void {
        const { offsetX: x, offsetY: y } = event;

        line.push(p2(
            Math.floor(width * (x / xMax)),
            Math.floor(height * (y / yMax)),
        ));

        draw();
    }

    function draw() {
        canvas.drawCb((img) => {
            Segment2.pipeDraw(img, line, Color.Teal);
            for (const p of line) {
                p.draw(img);
            }
        });
    }

    window.addEventListener('keypress', keyPressHandler);
}

export function capture(canvas: Canvas): Promise<Point2[]> {
    return new Promise((resolve) => {
        captureSync(canvas, resolve);
    });
}
