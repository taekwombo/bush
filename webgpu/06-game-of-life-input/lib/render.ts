import { Buffer } from './buffer.js';
import { assert } from './shared.js';
import type { Shaders } from './shaders.js';
import type { BindGroup } from './bind-group.js';

export class Render {
    protected device: GPUDevice;
    protected context: GPUCanvasContext;
    protected shaders: Shaders;
    protected vertexBuffer: Buffer;
    protected pipeline?: GPURenderPipeline;

    public constructor(device: GPUDevice, context: GPUCanvasContext, shaders: Shaders) {
        this.context = context;
        this.device = device;
        this.shaders = shaders;

        const vertices = new Float32Array([
            -1.0, 1.0, 0, 
            -1.0, -1.0, 0, 
            1.0, 1.0, 0, 
            1.0, -1.0, 0, 
        ]);

        this.vertexBuffer = Buffer.init(device, vertices, {
            binding: 0,
            usage: GPUBufferUsage.VERTEX,
            mappedAtCreation: true,
        });
    }

    public tick(this: Render, bindGroup: BindGroup, encoder: GPUCommandEncoder): Render {
        if (!this.pipeline) {
            this.createPipeline(bindGroup);
        }

        const renderPass = encoder.beginRenderPass({
            colorAttachments: [{
                storeOp: 'store',
                loadOp: 'clear',
                view: this.context.getCurrentTexture().createView(),
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
                // @ts-expect-error type definition library mis-typed
                loadValue: { r: 0, g: 0, b: 0, a: 1 },
            }],
        });

        renderPass.setPipeline(assert(this.pipeline));
        renderPass.setBindGroup(0, bindGroup.getBG());
        renderPass.setVertexBuffer(0, this.vertexBuffer.get());
        renderPass.draw(4);
        // @ts-expect-error type definition library mis-typed
        renderPass.endPass();

        return this;
    }

    protected createPipeline(this: Render, bindGroup: BindGroup): Render {
        this.pipeline = this.device.createRenderPipeline({
            layout: bindGroup.getPL(),
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
}
