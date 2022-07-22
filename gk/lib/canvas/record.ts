import type { Canvas } from '../canvas.js';

const dot = document.createElement('div');
dot.style.position = 'fixed';
dot.style.top = '0';
dot.style.left = '0';
dot.style.backgroundColor = 'red';
dot.style.borderRadius = '50%';
dot.style.width = '20px';
dot.style.height = '20px';

type P = [x: number, y: number][];

function printPositions(p: P): void {
    let debug: string[] = [];
    let p2s: string[] = [];
    const s = new Set();

    for (const [x, y] of p) {
        const d = `(${x}, ${y})`;
        if (s.has(d)) {
            continue;
        }
        s.add(d);
        debug.push(d);
        p2s.push(`p2(${x}, ${y})`);
    }

    if (debug.length === 0) {
        return;
    }

    console.log(debug.join(', '));
    console.log(p2s.join(', '));
}

export function positionRecording(canvas: Canvas): void {
    const { width, height } = canvas;
    const { clientWidth: xMax, clientHeight: yMax } = canvas.canvas;
    let recording: boolean = false;

    const positions: P = [];

    function clickHandler(event: MouseEvent): void {
        const { offsetX: x, offsetY: y } = event;

        positions.push([
            Math.floor(width * (x / xMax)),
            Math.floor(height * (y / yMax)),
        ]);
    }

    function keyPressHandler(event: KeyboardEvent): void {
        if (event.key === 'r') {
            if (recording) {
                document.body.removeChild(dot);
                recording = false;
                printPositions(positions);
                positions.length = 0;
            } else {
                document.body.appendChild(dot);
                canvas.canvas.addEventListener('click', clickHandler);
                recording = true;
            }

            return;
        }
    }

    window.addEventListener('keypress', keyPressHandler);
}
