type Context = {
    adapter: GPUAdapter;
    context: GPUCanvasContext;
    canvas: HTMLCanvasElement;
    device: GPUDevice;
};

export async function init(): Promise<Context> {
    if (!('gpu' in navigator)) {
        throw new Error('WebGPU not supported');
    }

    const adapter = await navigator.gpu.requestAdapter();

    if (!adapter) {
        throw new Error('Could not retrieve GPU Adapter');
    }

    const device = await adapter.requestDevice();
    const canvas = document.querySelector('canvas');

    if (!canvas) {
        throw new Error('Invalid canvas element selector');
    }

    const context = canvas.getContext('webgpu');

    if (!context) {
        throw new Error('Could not get "webgpu" canvas context');
    }

    return {
        adapter,
        canvas,
        context,
        device,
    };
}
