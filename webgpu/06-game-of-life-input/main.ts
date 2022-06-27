import {
    Buffer,
    BindGroup,
    Shaders,
    Render,
    Compute,
    Input,
    viewportData,
    cellSize,
    gridSize,
    gridData,
    updateCanvas,
    debounce,
} from './lib.js';

if (!('gpu' in navigator)) {
    alert('GPUWeb not supported');
    throw 1;
}

const adapter = await navigator.gpu.requestAdapter();

if (!adapter) {
    alert('Could not access GPU adapter');
    throw 1;
}

const device = await adapter.requestDevice();
const shaders = await Shaders.init(device);
const canvas = document.querySelector('canvas') as HTMLCanvasElement;

updateCanvas(canvas);

const context = canvas.getContext('webgpu');

if (!context) {
    alert('Could not get canvas context');
    throw 1;
}

context.configure({
    device,
    format: 'bgra8unorm',
});

const bindGroup = BindGroup
    .build(device)
    .add('cell_size', Buffer.init(device, new Float32Array([cellSize()]), {
        binding: 0,
        type: 'uniform',
        visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
        usage: GPUBufferUsage.UNIFORM,
    }))
    .add('viewport', Buffer.init(device, viewportData(canvas), {
        binding: 1,
        type: 'uniform',
        visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    }))
    .add('grid', Buffer.init(device, gridData(canvas), {
        binding: 2,
        type: 'read-only-storage',
        visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
        usage: GPUBufferUsage.UNIFORM
            | GPUBufferUsage.STORAGE
            | GPUBufferUsage.MAP_READ
            | GPUBufferUsage.COPY_DST,
    }))
    .add('compute', Buffer.init(device, gridData(canvas), {
        binding: 3,
        type: 'storage',
        visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
        usage: GPUBufferUsage.UNIFORM
            | GPUBufferUsage.STORAGE
            | GPUBufferUsage.COPY_DST
            | GPUBufferUsage.COPY_SRC,
    }))
    .finish();

const render = new Render(device, context, shaders);
const compute = new Compute(device, shaders, 'compute', 'grid');
const input = new Input(bindGroup, 'grid', canvas).init();

async function tick() {
    await input.hasWork();

    const enc = device.createCommandEncoder();

    render.tick(bindGroup, enc);
    compute.tick(bindGroup, enc, gridSize(canvas));

    device.queue.submit([enc.finish()]);
}

let queue: undefined | Promise<void> = Promise.resolve();

async function step(): Promise<void> {
    await queue;
    await tick();
    await new Promise((resolve) => setTimeout(resolve, 50));
    window.queueMicrotask(step);
}

step();
    
window.addEventListener('resize', debounce(async () => {
   const update = async () => {
        updateCanvas(canvas);
        context.configure({
            device,
            format: 'bgra8unorm',
        });

        await bindGroup.getBuffer('viewport').update(viewportData(canvas));

        await bindGroup.getBuffer('grid').update(gridData(canvas));

        await bindGroup.getBuffer('compute').update(gridData(canvas));

        bindGroup.update();

        queue = undefined;
    };

    if (queue) queue = queue.then(update);
    else queue = update();
}, 200));
