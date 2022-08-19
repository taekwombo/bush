export class Model {
    public static cube(device: GPUDevice): Model {
        return new Model(device, cube);
    }

    public device: GPUDevice;
    public vertices: Float32Array;
    public vb?: GPUBuffer;

    public constructor(device: GPUDevice, vertices: number[]) {
        this.device = device;
        this.vertices = new Float32Array(vertices);
    }

    public vertexBuffer(init: boolean = false): GPUBuffer {
        if (!this.vb) {
            const vb = this.device.createBuffer({
                size: this.vertices.byteLength,
                usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
            });

            if (init) {
                this.device.queue.writeBuffer(vb, 0, this.vertices);
            }

            this.vb = vb;

            return vb;
        }

        return this.vb;
    }
}

const cube = [
    // far face
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
     1.0,  1.0, -1.0,

    -1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
     1.0, -1.0, -1.0,

    // right face
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
     1.0,  1.0,  1.0,

     1.0, -1.0, -1.0,
     1.0,  1.0,  1.0,
     1.0, -1.0,  1.0,

    // behind face
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,

     1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,

    // left face
    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,

    // top face
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,

    -1.0,  1.0, -1.0,
     1.0,  1.0,  1.0,
     1.0,  1.0, -1.0, 

    // bottom face
     1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,

     1.0, -1.0 , 1.0,
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
];
