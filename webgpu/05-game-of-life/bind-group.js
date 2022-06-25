export class BindGroup {
    constructor(ctx, index) {
        Object.assign(this, ctx);

        this.cnt = 0;
        this.index = index;
        this.buffers = new Map();
        this.layout = null;
        this.pipelineLayout = null;
        this.bindGroup = null;
    }

    add(index, buffer) {
        this.buffers.set(
            index,
            buffer,
        );

        return this;
    }

    updateLayout() {
        const entries = [];

        for (const [binding, buffer] of this.buffers) {
            entries.push({
                binding,
                visibility: buffer.visibility,
                buffer: { type: buffer.type },
            });
        }

        this.layout = this.device.createBindGroupLayout({
            entries,
        });

        return this;
    }

    updatePipelineLayout() {
        this.pipelineLayout =
            this.device.createPipelineLayout({
                bindGroupLayouts: [this.layout],
            });

        return this;
    }

    update() {
        if (!this.layout) {
            this.updateLayout();
        }

        if (!this.pipelineLayout) {
            this.updatePipelineLayout();
        }

        const entries = [];

        for (const [binding, buffer] of this.buffers) {
            entries.push({
                binding,
                resource: {
                    buffer: buffer.buffer,
                    offset: buffer.offset,
                },
            });
        }

        this.cnt += 1;
        this.bindGroup = this.device.createBindGroup({
            label: '> bind group',
            entries,
            layout: this.layout,
        });

        return this;
    }
}
