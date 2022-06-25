export class GpuBuffer {
    constructor(ctx, options) {
        Object.assign(this, ctx);

        this.label = options.label;

        this.descriptor = {
            size: options.data?.byteLength || options.size,
            usage: options.usage,
            mappedAtCreation: options.mappedAtCreation,
        };

        this.data = options.data;
        this.type = options.type;
        this.visibility = options.visibility;
        this.offset = options.offset || 0;

        this.create();

        if (options.data) {
            if (this.descriptor.mappedAtCreation) {
                this.mapWrite(options.data);
                this.unmap();
            } else if (this.can(GPUBufferUsage.COPY_DST)) {
                this.write(options.data);
            }
        }
    }

    can(usage) {
        return (this.descriptor.usage & usage) === usage;
    }

    assertUsage(usage) {
        const ok = this.can(usage);

        if (!ok) {
            throw new Error('Missing usage flag:' + usage);
        }
    }

    create() {
        this.buffer = this.device.createBuffer(this.descriptor);
    }

    async read() {
        await this.buffer.mapAsync(GPUMapMode.READ, 0);
        const data = Array.from(new Float32Array(this.buffer.getMappedRange()));
        this.unmap();

        return data;
    }

    async update(data) {
        if (data.byteLength !== this.descriptor.size) {
            this.buffer.destroy();
            this.descriptor.size = data.byteLength;
            this.create();

            if (this.descriptor.mappedAtCreation) {
                this.mapWrite(data);
                this.unmap();
            } else {
                this.write(data);
            }
        } else {
            if ((this.descriptor.usage & GPUBufferUsage.COPY_DST) === GPUBufferUsage.COPY_DST) {
                this.write();
            } else {
                await this.buffer.mapAsync(GPUMapMode.WRITE);
                this.mapWrite();
                this.unmap();
            }
        }
    }

    mapWrite(data) {
        new Float32Array(this.buffer.getMappedRange()).set(data);

        return this;
    }

    unmap() {
        this.buffer.unmap();

        return this;
    }

    write(data) {
        this.device.queue.writeBuffer(
            this.buffer,
            0,
            data || this.data,
        );

        return this;
    }
}

