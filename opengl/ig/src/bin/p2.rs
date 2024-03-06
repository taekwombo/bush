//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=2

use gluty::{assets, gl, opengl, Glindow, Mesh, Obj, Program};
use ig::*;

struct Ctrl;

impl SOController for Ctrl {
    type Uniforms = so_uniforms::Uniforms;

    fn create_program(&self) -> Option<Program> {
        let (vert, frag) = assets!("./shaders/p2/shader.vert", "./shaders/p2/shader.frag",);
        let program = Program::default()
            .shader(frag.get(), gl::FRAGMENT_SHADER)
            .shader(vert.get(), gl::VERTEX_SHADER)
            .link();

        match Program::check_errors(&program) {
            Ok(()) => Some(program),
            Err(_) => None,
        }
    }

    fn load_mesh() -> Mesh {
        let obj = assets!("./teapot.obj");
        let (v, i) = Obj::new().parse_obj(&obj).build(&Default::default());
        Mesh::new(&v, &i, |a| {
            a.add::<f32>(0, 3, gl::FLOAT).add::<f32>(1, 3, gl::FLOAT);
        })
    }
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let mut project = SOProject::new(Ctrl, size);

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.update_uniforms();

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }

    #[allow(unused_variables)]
    let Glindow {
        window,
        event_loop,
        display,
        surface,
        context,
    } = glin;

    event_loop.run(move |event, _, control_flow| {
        use gluty::glutin::prelude::*;
        use gluty::winit::event::*;

        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                project.draw();
                surface.swap_buffers(&context).expect("I want to swap!");
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    surface.resize(
                        &context,
                        size.width.try_into().unwrap(),
                        size.height.try_into().unwrap(),
                    );
                    opengl! {
                        gl::Viewport(
                            0, 0,
                            size.width as i32,
                            size.height as i32,
                        );
                    }
                    project.resize(&size);
                }
                event => project.handle_window_events(&event, control_flow, &window),
            },
            _ => (),
        };
    });
}
