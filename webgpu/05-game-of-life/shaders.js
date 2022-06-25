export class Shaders {
    constructor(ctx) {
        Object.assign(this, ctx);
    }

    async compile(path) {
        const code = await fetch(path).then((r) => r.text());

        return this.device.createShaderModule({
            code,
            label: path,
        });
    }

    async init() {
        this.vertex = await this.compile(
            './vertex-shader.wgsl',
        );
        this.fragment = await this.compile(
            './fragment-shader.wgsl',
        );
        this.compute = await this.compile(
            './compute-shader.wgsl',
        );
    }
}
