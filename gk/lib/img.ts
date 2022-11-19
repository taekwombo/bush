import { Color } from './color.js';

export class Img {
    public image = ImageData['prototype'];

    public constructor(image: ImageData) {
        this.image = image;
    }

    public validate(this: Img, x: number, y: number): boolean {
        const { width, height } = this.image;

        const yMin = 0;
        const xMin = 0;

        return x <= width - 1 && y <= height - 1 && x >= xMin && y >= yMin;

    }

    public index(this: Img, x: number, y: number): number {
        return x * 4 + (y * this.image.width * 4);
    }

    public drawPoint(this: Img, x: number, y: number, color = Color.White): Img {
        if (!this.validate(x, y)) {
            return this;
        }

        if (x % 1 !== 0 || y % 1 !== 0) {
            throw new Error(`Position must be an integer (${x}, ${y})`);
        }

        const index = this.index(x, y);
        const { data } = this.image;

        if (index >= data.length) {
            throw new Error(`Invalid drawing point position x=${x} y=${y} len=${data.length} index=${index}`);
        }

        data[index] = color.r;
        data[index + 1] = color.g;
        data[index + 2] = color.b;
        data[index + 3] = color.a;

        return this;
    }

    public floodFill(this: Img, x: number, y: number, color: Color, mode: 4 | 8 = 4): Img {
        const checked: Map<number, Set<number>> = new Map();

        const unpainted = (x: number, y: number): boolean => {
            const i = this.index(x, y);
            const data = this.image.data;

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
}

