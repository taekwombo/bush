#![feature(int_roundings)]

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod grid;
mod wgpu_utils;

use grid::Grid;

fn main() {
    // GOL thingies.
    let (cell_size, mut paused) = get_arguments();
    let mut compute_single_tick = false;

    // Winit thingies.
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("window created");
    let init_size = window.inner_size();

    // WGPU thingies.
    let (adapter, device, queue, surface) = smol::block_on(init_wgpu(&window));
    let swapchain_format = surface.get_supported_formats(&adapter)[0];

    // GOL thingies.
    let grid = Grid::new(cell_size, init_size);

    let buffers = wgpu_utils::UBuffers::new(&device, &grid);

    let bind_group_layout = buffers.get_bgl(&device);
    let bind_group = buffers.get_bg(&device, &bind_group_layout);
    let pipeline_layout = buffers.get_pl(&device, &[&bind_group_layout]);

    let program = wgpu_utils::Program::new(&device, swapchain_format, &pipeline_layout);

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: init_size.width,
            height: init_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        match event {
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().expect("get_current_texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        depth_stencil_attachment: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        })],
                    });

                    render_pass.set_pipeline(&program.render_pipeline);
                    render_pass.set_bind_group(0, &bind_group, &[]);
                    render_pass
                        .set_vertex_buffer(0, program.vertex_buffer.slice(0..program.vb_size));
                    render_pass.draw(0..4, 0..1);
                }

                if !paused || compute_single_tick {
                    let mut compute_pass =
                        encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

                    compute_pass.set_pipeline(&program.compute_pipeline);
                    compute_pass.set_bind_group(0, &bind_group, &[]);
                    compute_pass.dispatch_workgroups(grid.cols as u32, grid.rows as u32, 1);

                    let src = buffers.get("next_cells");
                    let dst = buffers.get("cells");

                    drop(compute_pass);

                    encoder.copy_buffer_to_buffer(&src.buffer, 0, &dst.buffer, 0, src.size);

                    if compute_single_tick {
                        compute_single_tick = false;
                    }
                }

                queue.submit(Some(encoder.finish()));
                frame.present();

                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // TODO: reset cell buffers.
                // configure surface
                // grid.resize(size);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match keycode {
                VirtualKeyCode::R => {
                    let cells = grid.get_cell_buffer();
                    let next_cells_buffer = buffers.get("next_cells");
                    let cells_buffer = buffers.get("cells");

                    queue.write_buffer(&cells_buffer.buffer, 0, &cells);
                    queue.write_buffer(&next_cells_buffer.buffer, 0, &cells);
                }
                VirtualKeyCode::Space => {
                    if paused {
                        compute_single_tick = true;
                    }
                }
                VirtualKeyCode::P => {
                    paused = !paused;
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

async fn init_wgpu(window: &Window) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue, wgpu::Surface) {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed at requesting adapter.");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("gol-device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default().using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed at requesting device.");

    (adapter, device, queue, surface)
}

fn get_arguments() -> (u8, bool) {
    use clap::{Arg, ArgAction, Command};
    use std::process;

    let args = Command::new("game of life")
        .version("0.1.0")
        .about(
            "
Game of Life.
Key bindings:
    [p] pause/resume
    [r] reset
    [space] compute next tick when paused",
        )
        .arg(
            Arg::new("cell_size")
                .short('c')
                .long("cell_size")
                .default_value("4")
                .help("Set cell_size [u8]"),
        )
        .arg(
            Arg::new("paused")
                .action(ArgAction::SetTrue)
                .short('p')
                .long("paused")
                .help("Start program with paused state"),
        )
        .get_matches();

    let paused = args.get_one::<bool>("paused").copied().unwrap();
    let default_cell_size = String::from("4");
    let cell_size_arg: &String = args
        .get_one::<String>("cell_size")
        .unwrap_or(&default_cell_size);

    let cell_size = match cell_size_arg.parse::<u8>() {
        Ok(c) => {
            if c > 0 && c < 21 {
                c
            } else {
                println!("Expected cell_size to be in range [1..20]");
                process::exit(1);
            }
        }
        Err(_) => {
            println!("Invalid cell_size value ({}), expected u8.", cell_size_arg);
            process::exit(1);
        }
    };

    (cell_size, paused)
}
