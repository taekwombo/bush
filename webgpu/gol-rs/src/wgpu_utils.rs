use super::Grid;
use wgpu::util::DeviceExt;
use wgpu::*;

pub struct UserBuffer {
    pub name: &'static str,
    pub buffer: Buffer,
    pub binding: u32,
    pub size: u64,
    pub visibility: ShaderStages,
    pub ty: BufferBindingType,
}

impl UserBuffer {
    fn new(
        device: &Device,
        data: &[u8],
        usage: BufferUsages,
        label: &'static str,
        binding: u32,
        visibility: ShaderStages,
        ty: BufferBindingType,
    ) -> Self {
        Self {
            binding,
            visibility,
            ty,
            name: label,
            size: data.len().try_into().expect("size convert just ok"),
            buffer: device.create_buffer_init(&util::BufferInitDescriptor {
                label: Some(label),
                usage,
                contents: data,
            }),
        }
    }

    pub fn as_bgl(&self) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: BindingType::Buffer {
                ty: self.ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn as_bg(&self) -> BindGroupEntry {
        BindGroupEntry {
            binding: self.binding,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: None,
            }),
        }
    }
}

pub struct UBuffers {
    buffers: Vec<UserBuffer>,
}

impl UBuffers {
    pub fn new(device: &Device, grid: &Grid) -> Self {
        // Initialize cells data.
        let cells = grid.get_cell_buffer();

        // TODO: better debug.
        // let total = f32::from_ne_bytes([
        //    cells[0],
        //    cells[1],
        //    cells[2],
        //    cells[3],
        // ]);
        // println!("{:?} -- total {}", grid, total);
        // println!("len {}, -- elems {}", cells.len(), (cells.len()) / 4);

        // Buffers hardcoded to declutter main.
        let cell_size_buffer = UserBuffer::new(
            device,
            &grid.get_cell_size_data(),
            BufferUsages::UNIFORM,
            "cell_size",
            0,
            ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            BufferBindingType::Uniform,
        );
        let viewport_buffer = UserBuffer::new(
            device,
            &grid.get_viewport_data(),
            BufferUsages::UNIFORM,
            "viewport",
            1,
            ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            BufferBindingType::Uniform,
        );
        let cells_buffer = UserBuffer::new(
            device,
            &cells,
            BufferUsages::UNIFORM | BufferUsages::STORAGE | BufferUsages::COPY_DST,
            "cells",
            2,
            ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            BufferBindingType::Storage { read_only: true },
        );
        let next_cells_buffer = UserBuffer::new(
            device,
            &cells,
            BufferUsages::UNIFORM
                | BufferUsages::STORAGE
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
            "next_cells",
            3,
            ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            BufferBindingType::Storage { read_only: false },
        );

        Self {
            buffers: vec![
                cell_size_buffer,
                viewport_buffer,
                cells_buffer,
                next_cells_buffer,
            ],
        }
    }

    pub fn get(&self, name: &'static str) -> &UserBuffer {
        for buf in &self.buffers {
            if buf.name == name {
                return buf;
            }
        }

        let names = self.buffers.iter().map(|n| n.name).collect::<Vec<_>>();

        println!(
            "Couldn't find buffer {}. Did you mean one of {}",
            name,
            names.join(", ")
        );
        panic!();
    }

    pub fn get_bgl(&self, device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("the_only_one_bgl"),
            entries: &self.buffers.iter().map(|b| b.as_bgl()).collect::<Vec<_>>(),
        })
    }

    pub fn get_bg(&self, device: &Device, layout: &BindGroupLayout) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            layout,
            label: Some("the_only_one_bg"),
            entries: &self.buffers.iter().map(|b| b.as_bg()).collect::<Vec<_>>(),
        })
    }

    pub fn get_pl(&self, device: &Device, bgls: &[&BindGroupLayout]) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("the_only_one_pl"),
            bind_group_layouts: bgls,
            push_constant_ranges: &[],
        })
    }
}

pub struct Program {
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub vb_size: u64,
    pub compute_pipeline: ComputePipeline,
}

impl Program {
    pub fn new(device: &Device, format: TextureFormat, pl: &PipelineLayout) -> Self {
        macro_rules! init_vb {
            ($vec:ident, [$($elem:literal),+ $(,)*]) => {
                $(
                    $vec.extend_from_slice(&$elem.to_ne_bytes());
                )+
            };
        }

        let mut vertices: Vec<u8> = Vec::new();

        #[rustfmt::skip]
        init_vb!(vertices, [
            -1.0_f32,  1.0_f32, 0.0_f32,
            -1.0_f32, -1.0_f32, 0.0_f32,
             1.0_f32,  1.0_f32, 0.0_f32,
             1.0_f32, -1.0_f32, 0.0_f32,
        ]);

        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            usage: BufferUsages::VERTEX,
            contents: &vertices,
        });

        Self {
            vertex_buffer,
            vb_size: vertices.len() as u64,
            render_pipeline: create_render_pipeline(device, format, pl),
            compute_pipeline: create_compute_pipeline(device, pl),
        }
    }
}

fn create_shader_module(device: &Device, label: &'static str, path: &'static str) -> ShaderModule {
    device.create_shader_module(ShaderModuleDescriptor {
        label: Some(label),
        source: ShaderSource::Wgsl(String::from_utf8_lossy(
            &std::fs::read(path).unwrap_or_else(|_| panic!("{} loaded", label)),
        )),
    })
}

fn create_compute_pipeline(device: &Device, pl: &PipelineLayout) -> ComputePipeline {
    let compute_shader = create_shader_module(device, "compute_shader", "./src/compute.wgsl");

    device.create_compute_pipeline(&ComputePipelineDescriptor {
        layout: Some(pl),
        label: None,
        entry_point: "compute_main",
        module: &compute_shader,
    })
}

fn create_render_pipeline(
    device: &Device,
    format: TextureFormat,
    pl: &PipelineLayout,
) -> RenderPipeline {
    let vertex_shader = create_shader_module(device, "vertex_shader", "./src/vertex.wgsl");
    let fragment_shader = create_shader_module(device, "fragment_shader", "./src/fragment.wgsl");

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("the_only_one_rpl"),
        layout: Some(pl),
        vertex: VertexState {
            module: &vertex_shader,
            entry_point: "vertex_main",
            buffers: &[VertexBufferLayout {
                attributes: &[VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                }],
                array_stride: 12,
                step_mode: VertexStepMode::Vertex,
            }],
        },
        fragment: Some(FragmentState {
            module: &fragment_shader,
            entry_point: "fragment_main",
            targets: &[Some(format.into())],
        }),
        multiview: None,
        depth_stencil: None,
        multisample: MultisampleState::default(),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
            front_face: FrontFace::Ccw,
            unclipped_depth: false,
            conservative: false,
        },
    })
}
