import { init, getShaders } from '../utils/module.js';
import { proj, onUpdate } from './projection.js';
import { Model } from './model.js';

declare global {
    /** Imported from gk/lib/3d/matrix/mod.js. */
    export const Mat4: any;
}

const { canvas, context, device } = await init();

const shaders = await getShaders(device, {
    fragment: './shaders/fragment.wgsl',
    vertex: './shaders/vertex.wgsl',
});

const projectionBuffer = createBufferSync(
    proj(),
    GPUBufferUsage.COPY_DST | GPUBufferUsage.UNIFORM
);

const cube = Model.cube(device);
const vertexBuffer = cube.vertexBuffer(true);
const colorBuffer = createBufferSync(
    // There is for sure a better way to pass face colors to a vertex shader.
    new Float32Array([
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
    ]),
    GPUBufferUsage.COPY_DST | GPUBufferUsage.VERTEX,
);

canvas.width = 500;
canvas.height = 500;
context.configure({ device, format: 'bgra8unorm' });

const bgl = device.createBindGroupLayout({
    entries: [
        {
            binding: 0,
            visibility: GPUShaderStage.VERTEX,
            buffer: { type: 'uniform' },
        }
    ],
});
const bg = device.createBindGroup({
    layout: bgl,
    entries: [
        {
            binding: 0,
            resource: {
                buffer: projectionBuffer,
                offset: 0,
            },
        },
    ],
});

const pl = device.createPipelineLayout({
    bindGroupLayouts: [bgl],
});

const renderPipeline = device.createRenderPipeline({
    layout: pl,
    vertex: {
        module: shaders.vertex,
        entryPoint: 'vertex_main',
        buffers: [
            {
                attributes: [{
                    shaderLocation: 0,
                    offset: 0,
                    format: 'float32x3',
                }],
                arrayStride: 12,
                stepMode: 'vertex',
            },
            {
                attributes: [{
                    shaderLocation: 1,
                    offset: 0,
                    format: 'float32x3',
                }],
                arrayStride: 12,
                stepMode: 'vertex',
            },
        ],
    },
    depthStencil: {
        format: 'depth24plus-stencil8',
        depthCompare: 'less',
        depthWriteEnabled: true,
    },
    primitive: {
        topology: 'triangle-list',
    },
    fragment: {
        module: shaders.fragment,
        entryPoint: 'fragment_main',
        targets: [{ format: 'bgra8unorm' }],
    },
});


function render() {
    const encoder = device.createCommandEncoder();
    const renderPass = encoder.beginRenderPass({
        depthStencil: {
            depthWriteEnabled: true,
        },
        depthStencilAttachment: {
            view: device.createTexture({
                size: { width: canvas.width, height: canvas.height, depthOrArrayLayers: 1 },
                dimension: '2d',
                format: 'depth24plus-stencil8',
                usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            }).createView(),
            depthReadOnly: false,
            depthClearValue: 1.0,
            // @ts-expect-error
            depthLoadValue: 1.0,
            depthStoreOp: 'store',
            // Missing in docs and in types.
            stencilLoadValue: 1.0,
            stencilClearValue: 0.0,
            // Missing in docs and in types.
            stencilStoreOp: 'store',
            stencilReadOnly: false,
        },
        colorAttachments: [{
            storeOp: 'store',
            loadOp: 'clear',
            view: context.getCurrentTexture().createView(),
            // @ts-expect-error type definition library mistyped
            loadValue: { r: 0, g: 0, b: 0, a: 1 },
        }],
    });

    renderPass.setBindGroup(0, bg);
    renderPass.setPipeline(renderPipeline);
    renderPass.setVertexBuffer(0, vertexBuffer);
    renderPass.setVertexBuffer(1, colorBuffer);
    renderPass.draw(cube.vertices.length / 3);

    // @ts-expect-error
    renderPass.endPass();

    device.queue.submit([encoder.finish()]);
}

render();

onUpdate(() => {
    device.queue.writeBuffer(
        projectionBuffer,
        0,
        proj(),
    );
    render();
});

function createBufferSync(data: Float32Array, usage: number): GPUBuffer {
    const buffer = device.createBuffer({
        size: data.byteLength,
        usage,
    });

    if (usage & GPUBufferUsage.COPY_DST) {
        device.queue.writeBuffer(buffer, 0, data);
    }

    return buffer;
}
