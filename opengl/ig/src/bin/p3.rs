use gluty::{gl, Glindow, opengl, obj, Mesh, FlyCamera, Program};
use gluty::winit::event::*;
use gluty::winit::dpi::{PhysicalSize, PhysicalPosition};
use ig::*;

// TODO: Add lighting (ambient, diffuse, specular)
// TODO: Render lighting cube.
// TODO: Add light movement with Control.

struct Ctrl {
    state: InputState,
    mesh: Mesh,
    light: Light,
    u_model: i32,
    u_view: i32,
    u_proj: i32,
    u_lighting: i32,
    u_light_pos: i32,
    ctrl_pressed: bool,
}

impl Ctrl {
    fn new(size: PhysicalSize<u32>) -> Self {
        let mut light = Light::new();
        light.color.w = 0.0;
        Self {
            ctrl_pressed: false,
            u_model: -1,
            u_view: -1,
            u_proj: -1,
            u_light_pos: -1,
            u_lighting: -1,
            state: InputState::new(size),
            light,
            mesh: {
                let (v, i) = obj::load(&get_model_path());
                Mesh::new(&v, &i, |a| {
                    a.add::<f32>(0, 3, gl::FLOAT).add::<f32>(1, 3, gl::FLOAT);
                })
            },
        }
    }

    fn change_lighting(&mut self, state: &ElementState, keycode: &VirtualKeyCode) -> bool {
        if *state == ElementState::Released {
            return false;
        }

        match keycode {
            VirtualKeyCode::Key1 => {
                self.light.color.w = Lighting::Normal as i32 as f32;
            },
            VirtualKeyCode::Key2 => {
                self.light.color.w = Lighting::Ambient as i32 as f32;
            },
            VirtualKeyCode::Key3 => {
                self.light.color.w = Lighting::Diffuse as i32 as f32;
            },
            VirtualKeyCode::Key4 => {
                self.light.color.w = Lighting::Specular as i32 as f32;
            },
            VirtualKeyCode::Key5 => {
                self.light.color.w = Lighting::Phong as i32 as f32;
            },
            VirtualKeyCode::Key6 => {
                self.light.color.w = Lighting::Blinn as i32 as f32;
            },
            _ => return false,
        }
        true
    }

    fn handle_control_keypress(&mut self, state: &ElementState, keycode: &VirtualKeyCode) {
        match keycode {
            VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                self.ctrl_pressed = *state == ElementState::Pressed;
            },
            _ => (),
        }
    }

    fn handle_cursor_move(&mut self, position: &PhysicalPosition<f64>) -> bool {
        if !self.ctrl_pressed || self.state.mouse.is_none() {
            return false;
        }

        let Some((delta_x, delta_y)) = self.state.cursor_move(position) else {
            return false;
        };

        let speed = 4.0_f32;

        match self.state.mouse.unwrap() {
            MouseButton::Right => {
                self.light.translate(0.0, 0.0, delta_y * speed);
            },
            MouseButton::Left => {
                self.light.translate(delta_x * speed, delta_y * speed, 0.0);
            },
            _ => (),
        }

        true
    }
}

impl Controller for Ctrl {
    fn state(&mut self) -> &mut InputState {
        &mut self.state
    }

    fn upload_uniforms(&self, camera: &FlyCamera) {
        let color = self.light.color;
        opengl! {
            gl::UniformMatrix4fv(self.u_model,  1, gl::FALSE, self.mesh.model_to_world.as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_view,   1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_proj,   1, gl::FALSE, camera.get_proj().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_light_pos, 1, gl::FALSE, self.light.position.as_ref() as *const _);
            gl::Uniform4f(self.u_lighting, color.x, color.y, color.z, color.w);
        }
        // Lighting has own program bind automatically when calling Light::upload_uniforms;
        self.light.upload_uniforms(
            camera.get_view(),
            camera.get_proj()
        );
    }

    fn program_changed(&mut self, program: &Program) {
        self.u_model = program.get_uniform("u_model_t\0");
        self.u_view = program.get_uniform("u_view_t\0");
        self.u_proj = program.get_uniform("u_proj_t\0");
        self.u_light_pos = program.get_uniform("u_light_pos\0");
        self.u_lighting = program.get_uniform("u_lighting\0");
    }
}

#[derive(Copy, Clone)]
enum Lighting {
    /// Display colors of the model based on the surface normals.
    /// Normal(x, y, z) => Color(x, y, z, 1.0)
    /// Key1
    Normal,
    /// Display only ambient lighting.
    /// Key2
    Ambient,
    /// Display only diffuse color.
    /// Key3
    Diffuse,
    /// Display only specular lighting.
    /// Key4
    Specular,
    /// Use Phong material model.
    /// Key5
    Phong,
    /// Use Blinn material model.
    /// Key6
    Blinn,
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let mut project = Project::new(Ctrl::new(size), size, || create_program(Some("./shaders/p3")));

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.ctrl().light.translate(-20.0, 20.0, 30.0);
    project.upload_uniforms();

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }

    #[allow(unused_variables)]
    let Glindow { window, event_loop, display, surface, context } = glin;

    event_loop.run(move |event, _, control_flow| {
        use gluty::glutin::prelude::*;

        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                project.prog.use_program();
                project.ctrl().mesh.draw();
                project.ctrl().light.draw();
                surface.swap_buffers(&context).expect("I want to swap!");
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        surface.resize(
                            &context,
                            size.width.try_into().unwrap(),
                            size.height.try_into().unwrap(),
                        );
                        project.resize(size);
                        project.upload_uniforms();
                        window.request_redraw();
                    }
                },
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                },
                WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                    let Some(keycode) = input.virtual_keycode else {
                        return;
                    };

                    let controller = project.ctrl();
                    controller.handle_control_keypress(&input.state, &keycode);

                    if controller.change_lighting(&input.state, &keycode) {
                        project.upload_uniforms();
                        window.request_redraw();
                        return;
                    }

                    if project.handle_key_code(&input) {
                        project.upload_uniforms();
                        window.request_redraw();
                    }
                },
                WindowEvent::MouseInput { state: mouse_state, button, .. } => {
                    project.ctrl().state.mouse_click(&mouse_state, &button);
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if project.ctrl().handle_cursor_move(&position)
                        || project.handle_cursor_move(&position) {
                        project.upload_uniforms();
                        window.request_redraw();
                    }
                },
                _ => (),
            },
            _ => (),
        };
    });
}
