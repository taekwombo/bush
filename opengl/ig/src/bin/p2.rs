//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=2

use gluty::{gl, Glindow, opengl, obj, Mesh, Program, FlyCamera};
use gluty::winit::dpi::PhysicalSize;
use ig::*;

struct Ctrl {
    mesh: Mesh,
    state: InputState,
    u_model: i32,
    u_view: i32,
    u_proj: i32,
}

impl Ctrl {
    fn new(size: PhysicalSize<u32>) -> Self {
        Ctrl {
            mesh: {
                let (v, i) = obj::load(&get_model_path());
                Mesh::new(&v, &i, |a| {
                    a.add::<f32>(0, 3, gl::FLOAT).add::<f32>(1, 3, gl::FLOAT);
                })
            },
            state: InputState::new(size),
            u_model: -1,
            u_view: -1,
            u_proj: -1,
        }
    }
}

impl Controller for Ctrl {
    fn state(&mut self) -> &mut InputState {
        &mut self.state
    }

    fn upload_uniforms(&self, camera: &FlyCamera) {
        opengl! {
            gl::UniformMatrix4fv(self.u_model,  1, gl::FALSE, self.mesh.model_to_world.as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_view,   1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_proj,   1, gl::FALSE, camera.get_proj().as_ref() as *const _);
        }
    }

    fn program_changed(&mut self, program: &Program) {
        self.u_model = program.get_uniform("u_model_t\0");
        self.u_view = program.get_uniform("u_view_t\0");
        self.u_proj = program.get_uniform("u_proj_t\0");
    }
}

fn main () {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let mut project = Project::new(Ctrl::new(size), size, || create_program(Some("./shaders/p2")));

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.upload_uniforms();

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }

    #[allow(unused_variables)]
    let Glindow { window, event_loop, display, surface, context } = glin;

    event_loop.run(move |event, _, control_flow| {
        use gluty::winit::event::{Event, WindowEvent};
        use gluty::glutin::prelude::*;

        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                project.ctrl().mesh.draw();
                surface.swap_buffers(&context).expect("I want to swap!");
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                },
                WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                    if project.handle_key_code(&input) {
                        window.request_redraw();
                    }
                },
                WindowEvent::MouseInput { state: mouse_state, button, .. } => {
                    project.ctrl().state.mouse_click(&mouse_state, &button);
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if project.handle_cursor_move(&position) {
                        window.request_redraw();
                    }
                },
                _ => (),
            },
            _ => (),
        };
    });
}
