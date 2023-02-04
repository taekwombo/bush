import { gridSize, cellSize } from './shared.js';
import type { BindGroup } from './bind-group.js';
import type { Buffer } from './buffer.js';

export class Input {
    protected bindGroup: BindGroup;
    protected bufferName: string;
    protected mapBuffer: Buffer;
    protected markAlive: Set<number>;
    protected canvas: HTMLCanvasElement;
    protected work?: [() => void, Promise<void>];

    public constructor(bindGroup: BindGroup, bufferName: string, mapBuffer: Buffer, canvas: HTMLCanvasElement) {
        this.bindGroup = bindGroup;
        this.bufferName = bufferName;
        this.mapBuffer = mapBuffer;
        this.canvas = canvas;
        this.markAlive = new Set();
        this.onMouseMove = this.onMouseMove.bind(this);
        this.onMouseDown = this.onMouseDown.bind(this);
        this.onMouseUp = this.onMouseUp.bind(this);
    }

    public init() {
        window.addEventListener('mousedown', this.onMouseDown); 
        window.addEventListener('mouseup', this.onMouseUp);

        return this;
    }

    protected onMouseMove(event: MouseEvent): void {
        const size = cellSize();
        const { pageX: x, pageY: y } = event;
        const [gx] = gridSize(this.canvas);

        const indexX = Math.floor((x / window.innerWidth) * (window.innerWidth / size));
        const indexY = Math.floor((y / window.innerHeight) * (window.innerHeight / size));

        this.markAlive.add(1 + indexX + (indexY * gx));
    }

    protected onMouseDown(this: Input): void {
        this.createWork();
        window.addEventListener('mousemove', this.onMouseMove);
    }

    protected onMouseUp(this: Input): void {
        window.removeEventListener('mousemove', this.onMouseMove);
        this.finishWork();
    }

    public async readData(this: Input, encoder: GPUCommandEncoder): Promise<void> {
        const source = this.bindGroup.getBuffer(this.bufferName);
        const dest = this.mapBuffer;

        encoder.copyBufferToBuffer(source.get(), 0, dest.get(), 0, dest.size());
    }

    public async copyData(this: Input): Promise<void> {
        const buf = this.mapBuffer.get();
        await buf.mapAsync(GPUMapMode.READ);
        const currentData = Array.from(new Float32Array(buf.getMappedRange()));
        buf.unmap();

        for (const index of this.markAlive.values()) {
            currentData[index] = 1;
        }

        this.bindGroup.getBuffer(this.bufferName).write(new Float32Array(currentData));

        this.markAlive.clear();
    }

    protected createWork(this: Input): Input {
        let resolve: () => void = () => {
            throw new Error('Unreachable - hasWork')
        };

        const promise = new Promise<void>((r) => {
            resolve = r as () => void;
        });

        this.work = [resolve, promise];
        return this;
    }

    public hasWork(this: Input): boolean {
        return this.work !== undefined;
    }

    public waitForFinish(this: Input): Promise<void> {
        if (!this.work) {
            throw new Error('Unreachable');
        }

        return this.work[1];
    }

    protected finishWork(this: Input): void {
        if (!this.work) {
            throw new Error('Unreachable');
        }

        // Resolve pending promise.
        this.work[0]();
        this.work = undefined;
    }
}
