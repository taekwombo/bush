import { assert, cmp, assert_cmp } from './shared.js';

export type BufferOptions = GPUBufferDescriptor & {
    binding?: number;
    type?: GPUBufferBindingType;
    visibility?: number;
    offset?: number;
}

export class Buffer {
    public static init(device: GPUDevice, data: Float32Array, options: Omit<BufferOptions, 'size'>): Buffer {
        const size = data.byteLength;

        const copyDst = cmp(options.usage, GPUBufferUsage.COPY_DST);

        const buffer = new Buffer(device, {
            ...options,
            size,
            mappedAtCreation: !copyDst,
        });

        if (copyDst) {
            buffer.write(data);
        } else {
            buffer.mapWrite(data);
            buffer.unmap();
        }

        return buffer;
    }

    protected device: GPUDevice;
    protected buffer: GPUBuffer;
    protected descriptor: GPUBufferDescriptor;
    protected binding?: number;
    protected type?: GPUBufferBindingType;
    protected visibility?: number;
    protected offset: number;

    public constructor(device: GPUDevice, options: BufferOptions) {
        this.device = device;
        this.descriptor = {
            size: options.size,
            usage: options.usage,
            mappedAtCreation: options.mappedAtCreation,
        };
        this.buffer = device.createBuffer(this.descriptor);
        this.binding = options.binding;
        this.type = options.type;
        this.visibility = options.visibility;
        this.offset = options.offset || 0;
    }

    public get(this: Buffer): GPUBuffer {
        return this.buffer;
    }

    public size(this: Buffer): number {
        return this.descriptor.size;
    }

    public write(this: Buffer, data: Float32Array): Buffer {
        assert_cmp(this.descriptor.usage, GPUBufferUsage.COPY_DST);

        this.device.queue.writeBuffer(this.buffer, 0, data);
        return this;
    }

    public mapWrite(this: Buffer, data: Float32Array): Buffer {
        new Float32Array(this.buffer.getMappedRange()).set(data);
        return this;
    }

    public unmap(this: Buffer): Buffer {
        this.buffer.unmap();
        return this;
    }

    public asBindGroupEntry(this: Buffer): GPUBindGroupEntry {
        return {
            binding: assert(this.binding),
            resource: {
                buffer: this.buffer,
                offset: this.offset,
            },
        };
    }

    public asBindGroupLayoutEntry(this: Buffer): GPUBindGroupLayoutEntry {
        return {
            binding: assert(this.binding),
            visibility: assert(this.visibility),
            buffer: {
                type: assert(this.type),
            },
        };
    }
    
    public async update(data: Float32Array): Promise<Buffer> {
        const create = data.byteLength !== this.descriptor.size;

        if (create) {
            this.buffer.destroy();
            this.descriptor.size = data.byteLength;
            this.buffer = this.device.createBuffer(this.descriptor);
        }

        if (create && this.descriptor.mappedAtCreation) {
            this.mapWrite(data);
            this.unmap();
        } else {
            this.write(data);
        }

        return this;
    }

    public async read(): Promise<number[]> {
        assert_cmp(this.descriptor.usage, GPUBufferUsage.MAP_READ);

        await this.buffer.mapAsync(GPUMapMode.READ);

        const data = Array.from(new Float32Array(this.buffer.getMappedRange()));

        this.buffer.unmap();

        return data;
    }
}
