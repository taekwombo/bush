/* Paints every 20th pixel white, otherwise paints it relative to the viewport. */

if (!('gpu' in navigator)) {
    document.innerHTML = 'WebGPU is not supported';

    throw new Error('WebGPU is not supported');
}

const adapter = await navigator.gpu.requestAdapter();
const device = await adapter.requestDevice();
const canvas = document.querySelector('canvas');
const context = canvas.getContext('webgpu');
const configure = () => context.configure({ device, format: 'bgra8unorm' });

const viewportBuffer = device.createBuffer({
    size: 2 * 4,
    usage: GPUBufferUsage.UNIFORM
        /* For buffer.mapAsync(GPUMapMode.WRITE) */
        // | GPUBufferUsage.MAP_WRITE
        /* For device.queue.writeBuffer(...) */
        | GPUBufferUsage.COPY_DST,
    mappedAtCreation: true,
});

{ // Update data of viewportBuffer
    new Float32Array(viewportBuffer.getMappedRange()).set(new Float32Array([
        window.innerWidth,
        window.innerHeight,
    ]));

    viewportBuffer.unmap();
}

const updateUniformData = async () => {
    /* Use with GPUBufferUsage.MAP_WRITE */
    // await viewportBuffer.mapAsync(GPUMapMode.WRITE);
    // new Float32Array(viewportBuffer.getMappedRange()).set(new Float32Array([
    //     window.innerWidth,
    //     window.innerHeight,
    // ]));
    // viewportBuffer.unmap();
    /* Use with GPUBufferUsage.COPY_DST */
    device.queue.writeBuffer(
        viewportBuffer,
        0,
        new Float32Array([
            window.innerWidth,
            window.innerHeight,
        ]),
    );
};

/* https://gpuweb.github.io/gpuweb/#context-sizing */
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

configure();

let queue = Promise.resolve();

window.addEventListener('resize', debounce(() => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    queue = queue.then(async () => {
        configure();
        await updateUniformData();
        render();
    });
}, 200));

const shader = device.createShaderModule({
    label: 'shader-module',
    code: await fetch('./shader.wgsl').then((r) => r.text()),
});

const bindGroupLayout = device.createBindGroupLayout({
    entries: [{
        binding: 0,
        visibility: GPUShaderStage.FRAGMENT,
        buffer: { type: 'uniform' }
    }],
});

const bindGroup = device.createBindGroup({
    layout: bindGroupLayout,
    entries: [{
        binding: 0,
        resource: {
            buffer: viewportBuffer,
            offset: 0,
        },
    }],
});

const vertices = new Float32Array([
    -1.0, 1.0, 0, 
    -1.0, -1.0, 0, 
    1.0, 1.0, 0, 
    1.0, -1.0, 0, 
]);

const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: true
});

{ // Update data of vertexBuffer
    new Float32Array(vertexBuffer.getMappedRange()).set(vertices);

    vertexBuffer.unmap();
}

const renderPipeline = device.createRenderPipeline({
    layout: device.createPipelineLayout({
        bindGroupLayouts: [bindGroupLayout]
    }),
    vertex: {
        module: shader,
        entryPoint: 'vertex_main',
        buffers: [{
            attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }],
            arrayStride: 12,
            stepMode: 'vertex',
        }],
    },
    fragment: {
        module: shader,
        entryPoint: 'fragment_main',
        targets: [{ format: 'bgra8unorm' }],
    },
    primitive: {
        topology: 'triangle-strip',
    },
});

function render() {
    const enc = device.createCommandEncoder();
    const renderPass = enc.beginRenderPass({
        colorAttachments: [{
            storeOp: 'store',
            view: context.getCurrentTexture().createView(),
            loadValue: { r: 0, g: 0, b: 0, a: 1 },
        }],
    });

    renderPass.setPipeline(renderPipeline);
    renderPass.setBindGroup(0, bindGroup);
    renderPass.setVertexBuffer(0, vertexBuffer);
    renderPass.draw(4);
    renderPass.endPass();

    device.queue.submit([enc.finish()]);
}

render();

function debounce(f, t) {
    let x = null;
    
    return (...a) => {
        if (x) clearTimeout(x);
        x = setTimeout(() => {
            f(...a);
            x = null;
        }, t);
    };
}
