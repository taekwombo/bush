import { GpuBuffer } from './gpu-buffer.js';

export class Render {
    constructor(ctx, bindGroup, shaders) {
        Object.assign(this, ctx);

        this.shaders = shaders;
        this.bindGroup = bindGroup;
        this.vertexBuffer = new GpuBuffer(ctx, {
            data: new Float32Array([
                -1.0, 1.0, 0, 
                -1.0, -1.0, 0, 
                1.0, 1.0, 0, 
                1.0, -1.0, 0, 
            ]),
            usage: GPUBufferUsage.VERTEX,
            mappedAtCreation: true,
        });
    }

    update() {
        this.pipeline = this.device.createRenderPipeline({
            layout: this.bindGroup.pipelineLayout,
            vertex: {
                module: this.shaders.vertex,
                entryPoint: 'vertex_main',
                buffers: [{
                    attributes: [{
                        shaderLocation: 0,
                        offset: 0,
                        format: 'float32x3',
                    }],
                    arrayStride: 12,
                    stepMode: 'vertex',
                }],
            },
            primitive: {
                topology: 'triangle-strip',
            },
            fragment: {
                module: this.shaders.fragment,
                entryPoint: 'fragment_main',
                targets: [{ format: 'bgra8unorm' }],
            },
        });

        return this;
    }

    tick(encoder) {
        const renderPass = encoder.beginRenderPass({
            colorAttachments: [{
                storeOp: 'store',
                view: this.context.getCurrentTexture().createView(),
                loadValue: { r: 0, g: 0, b: 0, a: 1 },
            }],
        });

        renderPass.setPipeline(this.pipeline);
        renderPass.setBindGroup(0, this.bindGroup.bindGroup);
        renderPass.setVertexBuffer(0, this.vertexBuffer.buffer);
        renderPass.draw(4);

        renderPass.endPass();

        return this;
    }
}
