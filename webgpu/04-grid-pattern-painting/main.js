/* Toggle state of grid cells when moving over the grid while LMB is pressed down. */

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
        /* For buffer mapping */
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

const cellSize = 20.0;

const cellSizeBuffer = device.createBuffer({
    size: 4,
    usage: GPUBufferUsage.UNIFORM,
    mappedAtCreation: true,
});

{
    new Float32Array(cellSizeBuffer.getMappedRange()).set(new Float32Array([cellSize]));

    cellSizeBuffer.unmap();
}


let gridBuffer, grid;
updateGridBuffer();

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
        updateGridBuffer();
        updateBindGroup();
        render();
    });
}, 200));

const fragmentShader = device.createShaderModule({
    label: 'fragment-shader-module',
    code: await fetch('./fragment-shader.wgsl').then((r) => r.text()),
});
const vertexShader = device.createShaderModule({
    label: 'vertex-shader-module',
    code: await fetch('./vertex-shader.wgsl').then((r) => r.text()),
});

const bindGroupLayout = device.createBindGroupLayout({
    entries: [
        {
            binding: 0,
            visibility: GPUShaderStage.FRAGMENT,
            buffer: { type: 'uniform' }
        },
        {
            binding: 1,
            visibility: GPUShaderStage.FRAGMENT,
            buffer: { type: 'uniform' },
        },
        {
            binding: 2,
            visibility: GPUShaderStage.FRAGMENT,
            buffer: { type: 'read-only-storage' },
        },
    ],
});

let bindGroup;
updateBindGroup();

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
        module: vertexShader,
        entryPoint: 'vertex_main',
        buffers: [{
            attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }],
            arrayStride: 12,
            stepMode: 'vertex',
        }],
    },
    fragment: {
        module: fragmentShader,
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

function createGrid(x, y) {
    const grid = new Array(1 + (x * y));
    grid[0] = x * y;

    for (let i = 0; i < y; i++) {
        for (let j = 0; j < x; j++) {
            const value = 0.0;
            const index = 1 + j + (i * x);

            grid[index] = value;
        }
    }

    return grid;
}

function updateGridBuffer(data) {
    const grid_x = Math.ceil(window.innerWidth / cellSize);
    const grid_y = Math.ceil(window.innerHeight / cellSize);
    grid = new Float32Array(data || createGrid(grid_x, grid_y));

    gridBuffer = device.createBuffer({
        size: grid.byteLength,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });

    device.queue.writeBuffer(gridBuffer, 0, grid);
}

function updateBindGroup() {
    bindGroup = device.createBindGroup({
        layout: bindGroupLayout,
        entries: [
            {
                binding: 0,
                resource: {
                    buffer: viewportBuffer,
                    offset: 0,
                },
            },
            {
                binding: 1,
                resource: {
                    buffer: cellSizeBuffer,
                    offset: 0,
                },
            },
            {
                binding: 2,
                resource: {
                    buffer: gridBuffer,
                    offset: 0,
                },
            },
        ],
    });
}

const toggle = new Set();

function onMouseMove(event) {
    const { pageX: x, pageY: y } = event;

    const grid_x = Math.ceil(window.innerWidth / cellSize);
    const grid_y = Math.ceil(window.innerHeight / cellSize);

    const x_n = x / window.innerWidth;
    const y_n = y / window.innerHeight;

    const index_x = Math.floor(x_n * (window.innerWidth / cellSize));
    const index_y = Math.floor(y_n * (window.innerHeight / cellSize));

    const index = 1.0 + index_x + (index_y * grid_x);

    toggle.add(index);
}

async function toggleCells() {
    for (const index of toggle.values()) {
        grid[index] = grid[index] === 1.0 ? 0.0 : 1.0;
    }

    device.queue.writeBuffer(gridBuffer, 0, grid);
    render();
    toggle.clear();
}

canvas.addEventListener('mousedown', (event) => {
    canvas.addEventListener('mousemove', onMouseMove);
});

canvas.addEventListener('mouseup', (event) => {
    canvas.removeEventListener('mousemove', onMouseMove);

    queue = queue.then(toggleCells);
});

