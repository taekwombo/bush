import * as record from './canvas/record.js';
import { Img } from './img.js';

export class Canvas {
    public static record = record;

    protected static getCanvas(width: number, height: number, selector: string, stretch: boolean): HTMLCanvasElement {
        let canvas: HTMLCanvasElement = document.querySelector(selector)!;

        if (!canvas) {
            canvas = document.createElement('canvas');
            document.body.appendChild(canvas);
        }

        if (stretch) {   // Make canvas appear to be bigger than drawing buffer.
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

    public static create2(width: number, height: number, selector: string = 'canvas', stretch: boolean = true): Canvas {
        const canvas = this.getCanvas(width, height, selector, stretch);
        const context = this.getContext2(canvas);

        return new Canvas(canvas, context, width, height);
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

    public getImageData(this: Canvas, [x, y, w, h]: [number, number, number, number]): Img {
        const imageData = this.context.getImageData(x, y, w, h);

        return new Img(imageData);
    }

    public putImageData(this: Canvas, img: ImageData, [x, y]: [number, number]): void {
        this.context.putImageData(img, x, y);
    }

    public drawCb(this: Canvas, cb: (img: Img, draw: () => void) => void, dimensions: [number?, number?, number?, number?] = []): Canvas {
        const [x = 0, y = 0, w = this.width, h = this.height] = dimensions;
        const img = this.getImageData([x, y, w, h]);
        const draw = () => this.putImageData(img.image, [x, y]);

        cb(img, draw);

        draw();

        return this;
    }
    
    public clear(this: Canvas): Canvas {
        this.context.clearRect(0, 0, this.width, this.height);

        return this;
    }
}

