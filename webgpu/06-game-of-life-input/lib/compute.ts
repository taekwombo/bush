import { assert } from './shared.js';
import type { BindGroup } from './bind-group.js';
import type { Shaders } from './shaders.js';

export class Compute {
    protected device: GPUDevice;
    protected shaders: Shaders;
    protected srcBuf: string;
    protected dstBuf: string;
    protected pipeline?: GPUComputePipeline;

    public constructor(device: GPUDevice, shaders: Shaders, srcBuf: string, dstBuf: string) {
        this.device = device;
        this.shaders = shaders;
        this.srcBuf = srcBuf;
        this.dstBuf = dstBuf;
    }

    public tick(this: Compute, bindGroup: BindGroup, encoder: GPUCommandEncoder, grid: [number, number]): Compute {
        if (!this.pipeline) {
            this.createPipeline(bindGroup);
        }

        const computePass = encoder.beginComputePass();

        computePass.setPipeline(assert(this.pipeline));
        computePass.setBindGroup(0, bindGroup.getBG());
        computePass.dispatchWorkgroups(grid[0], grid[1]);

        // @ts-expect-error lib typings incorrect
        computePass.endPass();

        return this.copyData(bindGroup, encoder);
    }

    protected createPipeline(this: Compute, bindGroup: BindGroup): Compute {
        this.pipeline = this.device.createComputePipeline({
            layout: bindGroup.getPL(),
            compute: {
                module: this.shaders.compute,
                entryPoint: 'compute_main',
            },
        });

        return this;
    }

    protected copyData(this: Compute, bindGroup: BindGroup, encoder: GPUCommandEncoder): Compute {
        const srcBuf = bindGroup.getBuffer(this.srcBuf);
        const dstBuf = bindGroup.getBuffer(this.dstBuf);

        encoder.copyBufferToBuffer(
            srcBuf.get(),
            0,
            dstBuf.get(),
            0,
            srcBuf.size(),
        );

        return this;
    }
}
