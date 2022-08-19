type Shaders<T extends Record<string, string>> = {
    [K in keyof T]: GPUShaderModule;
};

export async function getShaders<T extends Record<string, string>>(device: GPUDevice, shaders: T): Promise<Shaders<T>> {
    const result = {} as unknown as Shaders<T>;

    for (const name in shaders) {
        result[name] = device.createShaderModule({
            label: name,
            code: await fetch(shaders[name]).then((response) => response.text())
        });
    }

    return result;
}
