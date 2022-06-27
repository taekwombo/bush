export class Shaders {
    private static async fetch(path: string): Promise<string> {
        return fetch(path).then((r) => r.text());
    }

    public static async init(device: GPUDevice): Promise<Shaders> {
        const fragment = await Shaders.fetch('./fragment-shader.wgsl');
        const vertex = await Shaders.fetch('./vertex-shader.wgsl');
        const compute = await Shaders.fetch('./compute-shader.wgsl');

        return new Shaders(device, vertex, fragment, compute);
    }

    public fragment: GPUShaderModule;
    public vertex: GPUShaderModule;
    public compute: GPUShaderModule;

    protected constructor(device: GPUDevice, v: string, f: string, c: string) {
        this.fragment = device.createShaderModule({ code: f });
        this.vertex = device.createShaderModule({ code: v });
        this.compute = device.createShaderModule({ code: c });
    }
}
