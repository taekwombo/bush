/* Game of life on compute shader. */

import { Queue } from './queue.js';
import { Shaders } from './shaders.js';
import { Game } from './game.js';
import { GpuBuffer } from './gpu-buffer.js';
import { BindGroup } from './bind-group.js';
import { Render } from './render.js';
import { Compute } from './compute.js';
import { createGrid, GridRule, cellSize } from './grid.js';
import { debounce } from './utils.js';

if (!('gpu' in navigator)) {
    document.innerHTML = 'WebGPU is not supported';

    throw new Error('WebGPU is not supported');
}

const game = new Game();

await game.init();

const shaders = new Shaders(game.ctx);

await shaders.init();

const viewportBuffer = new GpuBuffer(game.ctx, {
    label: 'viewport',
    data: new Float32Array([window.innerWidth, window.innerHeight]),
    type: 'uniform',
    offset: 0,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
});

const cellSizeBuffer = new GpuBuffer(game.ctx, {
    label: 'cell_size',
    data: new Float32Array([cellSize]),
    type: 'uniform',
    offset: 0,
    mappedAtCreation: true,
    usage: GPUBufferUsage.UNIFORM,
    visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
});

const gridBuffer = new GpuBuffer(game.ctx, {
    label: 'grid',
    data: new Float32Array(createGrid(GridRule.Pattern)),
    type: 'read-only-storage',
    offset: 0,
    usage: GPUBufferUsage.STORAGE
        | GPUBufferUsage.COPY_DST,
    visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE,
});

const computeBuffer = new GpuBuffer(game.ctx, {
    label: 'compute',
    data: new Float32Array(createGrid()),
    type: 'storage',
    offset: 0,
    visibility: GPUShaderStage.COMPUTE,
    usage: GPUBufferUsage.STORAGE
        | GPUBufferUsage.COPY_DST
        | GPUBufferUsage.COPY_SRC,
});

const bindGroup = new BindGroup(game.ctx, 0)
    .add(0, cellSizeBuffer)
    .add(1, viewportBuffer)
    .add(2, gridBuffer)
    .add(3, computeBuffer)
    .update();

const render = new Render(game.ctx, bindGroup, shaders).update();
const compute = new Compute(game.ctx, bindGroup, shaders, [2, 3]).update();
const queue = new Queue();

const tick = async () => {
    await game.tick(async (encoder) => {
        render.tick(encoder);
        compute.tick(encoder);

        compute.copyBufferData(encoder);

    });
};

async function microtaskAnimate() {
    await queue.push(tick);
    await new Promise((r) => setTimeout(r, 900));
    window.queueMicrotask(microtaskAnimate);
}

async function animationAnimate() {
    window.requestAnimationFrame(() => {
        queue.push(async () => {
            await tick();
            return animationAnimate();
        });
    });
}

async function configure() {
    game.configure();

    viewportBuffer.write(new Float32Array([window.innerWidth, window.innerHeight]));

    const grid = new Float32Array(createGrid());
    await gridBuffer.update(grid);
    await computeBuffer.update(grid);
    bindGroup.update();
}

window.addEventListener('resize', debounce(() => queue.push(configure), 200));

{ // Main block starting up the program.
    configure()
        .then(() => animationAnimate());
}
