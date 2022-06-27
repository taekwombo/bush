import { assert } from './shared.js';
import type { Buffer } from './buffer.js';

class Builder {
    protected buffers: Map<string, Buffer>;
    protected device: GPUDevice;

    public constructor(device: GPUDevice) {
        this.device = device;
        this.buffers = new Map();
    }

    public add(this: Builder, label: string, buffer: Buffer): Builder {
        this.buffers.set(label, buffer);

        return this;
    }

    protected layout(this: Builder): GPUBindGroupLayout {
        const entries: GPUBindGroupLayoutEntry[] = [];

        for (const buffer of this.buffers.values()) {
            entries.push(buffer.asBindGroupLayoutEntry());
        }

        return this.device.createBindGroupLayout({
            entries 
        });
    }

    protected pipelineLayout(this: Builder, layouts: GPUBindGroupLayout[]): GPUPipelineLayout {
        return this.device.createPipelineLayout({
            bindGroupLayouts: layouts
        });
    }

    protected bindGroup(this: Builder, layout: GPUBindGroupLayout): GPUBindGroup {
        const entries: GPUBindGroupEntry[] = [];

        for (const buffer of this.buffers.values()) {
            entries.push(buffer.asBindGroupEntry());
        }

        entries.sort((a, b) => a.binding - b.binding);

        return this.device.createBindGroup({
            layout,
            entries,
        });
    }

    public finish(this: Builder): BindGroup {
        const layout = this.layout();
        const pipelineLayout = this.pipelineLayout([layout]);
        const bindGroup = this.bindGroup(layout);

        return new BindGroup({
            device: this.device,
            layout,
            bindGroup,
            pipelineLayout,
            buffers: this.buffers,
        });
    }
}

type BindGroupOptions = {
    buffers: Map<string, Buffer>;
    device: GPUDevice;
    bindGroup: GPUBindGroup;
    layout: GPUBindGroupLayout;
    pipelineLayout: GPUPipelineLayout;
}

export class BindGroup {
    public static build(device: GPUDevice): Builder {
        return new Builder(device);
    }

    protected buffers: Map<string, Buffer>;
    protected device: GPUDevice;
    protected bindGroup: GPUBindGroup;
    protected layout: GPUBindGroupLayout;
    protected pipelineLayout: GPUPipelineLayout;

    public constructor({ device, buffers, bindGroup, layout, pipelineLayout }: BindGroupOptions) {
        this.device = device;
        this.buffers = buffers;
        this.bindGroup = bindGroup;
        this.layout = layout;
        this.pipelineLayout = pipelineLayout;
    }

    public getBuffer(this: BindGroup, buffer: string): Buffer {
        return assert(this.buffers.get(buffer));
    }

    public getPL(this: BindGroup): GPUPipelineLayout {
        return this.pipelineLayout;
    }

    public getLayout(this: BindGroup): GPUBindGroupLayout {
        return this.layout;
    }

    public getBG(this: BindGroup): GPUBindGroup {
        return this.bindGroup;
    }

    public update(this: BindGroup): BindGroup {
        const entries: GPUBindGroupEntry[] = [];

        for (const buffer of this.buffers.values()) {
            entries.push(buffer.asBindGroupEntry());
        }

        entries.sort((a, b) => a.binding - b.binding);

        this.bindGroup = this.device.createBindGroup({
            layout: this.layout,
            entries,
        });

        return this;
    }
}
