import { Color } from './color.js';
import type { ImageDataExt } from './types.js';

export class Canvas {
    protected static getCanvas(width: number, height: number): HTMLCanvasElement {
        let canvas: HTMLCanvasElement = document.querySelector('canvas')!;

        if (!canvas) {
            canvas = document.createElement('canvas');
            document.body.appendChild(canvas);
        }

        {   // Make canvas appear to be bigger than drawing buffer.
            const size = Math.min(window.innerWidth, window.innerHeight);
            canvas.style.width = `${size}px`;
            canvas.style.height = `${size}px`;
        }

        canvas.width = width;
        canvas.height = height;

        return canvas;
    }

    protected static getContext2(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
        const context = canvas.getContext('2d');

        if (!context) {
            throw new Error('Could not get canvas context');
        }

        return context;
    }

    public static create2(width: number, height: number): Canvas {
        const canvas = this.getCanvas(width, height);
        const context = this.getContext2(canvas);

        return new Canvas(canvas, context, width, height);
    }

    protected static validCoord(this: ImageData, x: number, y: number): boolean {
        const ly = 0;
        const lx = 0;

        return x <= this.width - 1 && y <= this.height - 1 && x >= lx && y >= ly;
    }

    protected static getPointIndex(this: ImageData, x: number, y: number): number {
        return x * 4 + (y * this.width * 4);
    }

    protected static drawPoint(this: ImageDataExt, x: number, y: number, color = Color.White): void {
        if (!this.validate(x, y)) {
            return;
        }

        if (x % 1 !== 0 || y % 1 !== 0) {
            throw new Error(`Position must be an integer (${x}, ${y})`);
        }

        const index = this.index(x, y);
        const { data } = this;

        if (index >= data.length) {
            throw new Error(`Invalid drawing point position x=${x} y=${y} len=${data.length} index=${index}`);
        }

        data[index] = color.r;
        data[index + 1] = color.g;
        data[index + 2] = color.b;
        data[index + 3] = color.a;
    }

    protected static floodFill(this: ImageDataExt, x: number, y: number, color: Color, mode: 4 | 8 = 4): ImageDataExt {
        const checked: Map<number, Set<number>> = new Map();

        const unpainted = (x: number, y: number): boolean => {
            const i = this.index(x, y);
            const data = this.data;

            return data[i] === 0 && data[i + 1] === 0 && data[i + 2] === 0 && data[i + 3] === 0;
        };

        function getNeighbours(x: number, y: number): number[] {
            const allNeighbours: number[] = [
                x, y + 1,
                x, y - 1,
                x + 1, y,
                x - 1, y,
            ];

            if (mode === 8) {
                allNeighbours.push(x + 1, y + 1);
                allNeighbours.push(x - 1, y + 1);
                allNeighbours.push(x + 1, y - 1);
                allNeighbours.push(x - 1, y - 1);
            }

            const neighbours: number[] = [];

            for (let i = 0; i < allNeighbours.length; i += 2) {
                const x = allNeighbours[i];
                const y = allNeighbours[i + 1];

                if (checked.has(x)) {
                    const s = checked.get(x)!;

                    if (s.has(y)) {
                        continue;
                    }

                    s.add(y);
                } else {
                    checked.set(x, new Set<number>().add(y));
                }

                neighbours.push(x, y);
            }

            return neighbours;
        }

        const queue: number[] = [x, y];

        while (queue.length > 0) {
            const y = queue.pop();
            const x = queue.pop();

            if (!(x !== undefined && y !== undefined && this.validate(x, y) && unpainted(x, y))) {
                continue;
            }

            this.drawPoint(x, y, color);

            queue.push(...getNeighbours(x, y));
        }

        return this;
    }

    public canvas: HTMLCanvasElement;
    public context: CanvasRenderingContext2D;
    public width: number;
    public height: number;

    protected constructor(
        canvas: HTMLCanvasElement,
        context: CanvasRenderingContext2D,
        width: number,
        height: number,
    ) {
        this.canvas = canvas;
        this.context = context;
        this.width = width;
        this.height = height;
    }

    public getImageData(this: Canvas, [x, y, w, h]: [number, number, number, number]): ImageDataExt {
        const imageData = this.context.getImageData(x, y, w, h) as ImageDataExt;

        imageData.validate = Canvas.validCoord.bind(imageData);
        imageData.index = Canvas.getPointIndex.bind(imageData);
        imageData.drawPoint = Canvas.drawPoint.bind(imageData);
        imageData.floodFill = Canvas.floodFill.bind(imageData);

        return imageData as ImageDataExt;
    }

    public putImageData(this: Canvas, img: ImageData, [x, y]: [number, number]): void {
        this.context.putImageData(img, x, y);
    }

    public drawCb(this: Canvas, cb: (img: ImageDataExt, put: () => void) => void, dimensions: [number?, number?, number?, number?] = []): void {
        const [x = 0, y = 0, w = this.width, h = this.height] = dimensions;
        const img = this.getImageData([x, y, w, h]);
        const put = () => this.putImageData(img, [x, y]);

        cb(img, put);

        put();
    }
}

