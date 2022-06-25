import { getGridSize } from './grid.js';

export class Compute {
    constructor(ctx, bindGroup, shaders, [dstBuffer, srcBuffer]) {
        Object.assign(this, ctx);

        this.bindGroup = bindGroup;
        this.shaders = shaders;
        this.srcBuffer = bindGroup.buffers.get(srcBuffer);
        this.dstBuffer = bindGroup.buffers.get(dstBuffer);
    }

    update() {
        this.pipeline = this.device.createComputePipeline({
            layout: this.bindGroup.pipelineLayout,
            compute: {
                module: this.shaders.compute,
                entryPoint: 'compute_main',
            },
        });

        return this;
    }

    tick(encoder) {
        const computePass = encoder.beginComputePass();
        const { x, y } = getGridSize();

        computePass.setPipeline(this.pipeline);
        computePass.setBindGroup(0, this.bindGroup.bindGroup);
        computePass.dispatchWorkgroups(x, y);

        computePass.endPass();

        return this;
    }

    copyBufferData(encoder) {
        encoder.copyBufferToBuffer(
            this.srcBuffer.buffer,
            0,
            this.dstBuffer.buffer,
            0,
            this.srcBuffer.descriptor.size,
        );
    }
}
