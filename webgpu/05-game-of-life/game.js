export class Game {
    async init() {
        const adapter = await navigator.gpu.requestAdapter();
        const device = await adapter.requestDevice();
        const canvas = document.querySelector('canvas');
        const context = canvas.getContext('webgpu');

        /* https://gpuweb.github.io/gpuweb/#context-sizing */
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;

        this.ctx = { adapter, device, canvas, context };

        return this;
    }

    configure() {
        this.ctx.canvas.width = window.innerWidth;
        this.ctx.canvas.height = window.innerHeight;

        this.ctx.context.configure({
            device: this.ctx.device,
            format: 'bgra8unorm',
        });


        return this;
    }

    async tick(fn) {
        const encoder = this.ctx.device.createCommandEncoder();

        await fn(encoder);

        this.ctx.device.queue.submit([encoder.finish()]);
    }
}
