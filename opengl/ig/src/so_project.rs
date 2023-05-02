use super::{size_u_to_f32, InputState, Light};
use gluty::winit::dpi::{PhysicalPosition, PhysicalSize};
use gluty::winit::{event::*, event_loop::ControlFlow, window::Window};
use gluty::{gl, opengl, FlyCamera, Mesh, Program, Projection};

pub mod camera_consts {
    pub const NEAR: f32 = 0.01;
    pub const FAR: f32 = 1000.0;
    pub const FOV: f32 = 60.0;
    pub const ORHO: [f32; 6] = [-50.0, 50.0, -50.0, 50.0, NEAR, FAR];
}

/// Single object project.
/// Mesh, Camera, optional Light.
pub struct SOProject<T: SOController> {
    pub controller: T,
    pub program: Program,
    pub camera: FlyCamera,
    pub model: Mesh,
    pub light: Option<Light>,
    pub input_state: InputState,
    size: PhysicalSize<f32>,
    pub uniforms: T::Uniforms,
}

impl<T: SOController> SOProject<T> {
    pub fn new(controller: T, win_size: PhysicalSize<u32>) -> Self {
        let program = T::create_program();
        let light = T::create_light();
        let model = T::load_mesh();
        let size = size_u_to_f32(&win_size);
        let uniforms = T::Uniforms::new(&program);

        Self {
            model,
            program,
            uniforms,
            light,
            controller,
            size,
            input_state: InputState::new(win_size),
            camera: FlyCamera::new(|| {
                use camera_consts::*;
                Projection::perspective(FOV, size.width / size.height, NEAR, FAR)
            }),
        }
    }

    fn update_light(&self) -> &Self {
        if let Some(ref light) = self.light {
            light.update_uniforms(self.camera.get_view(), self.camera.get_proj());
            self.program.use_program();
            self.uniforms.update_light(light);
        }
        self
    }

    pub fn update_uniforms(&mut self) -> &mut Self {
        self.update_light()
            .update_camera_uniforms()
            .update_model_uniforms();
        self
    }

    fn update_camera_uniforms(&self) -> &Self {
        self.update_light();
        self.program.use_program();
        self.uniforms.update_camera(&self.camera);
        self
    }

    fn update_model_uniforms(&self) -> &Self {
        self.program.use_program();
        self.uniforms.update_model(&self.model);
        self
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) -> &mut Self {
        self.size = size_u_to_f32(size);
        self.camera
            .projection
            .resize(self.size.width / self.size.height);
        self.camera.update();
        self.update_camera_uniforms();
        self
    }

    pub fn handle_window_events(
        &mut self,
        event: &WindowEvent,
        control_flow: &mut ControlFlow,
        window: &Window,
    ) {
        match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::KeyboardInput {
                input,
                is_synthetic: false,
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    if self.should_render_key_input(&input.state, &keycode) {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input(state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.should_render_cursor_move(position) {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }

    fn handle_mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
        self.input_state.mouse_click(state, button);
    }

    fn should_render_cursor_move(&mut self, position: &PhysicalPosition<f64>) -> bool {
        if self.input_state.mouse.is_none() {
            return false;
        }

        let Some((delta_x, delta_y)) = self.input_state.cursor_move(position) else {
            return false;
        };

        let input_ctx = CursorInputContext {
            light: self.light.as_mut(),
            uniforms: &mut self.uniforms,
            input_state: &self.input_state,
            mouse_delta: (delta_x, delta_y),
        };

        if self.controller.handled_cursor_move(input_ctx) {
            return true;
        }

        let speed = 4.0_f32;

        if !(self.input_state.ctrl || self.input_state.alt) {
            return match self.input_state.mouse.unwrap() {
                MouseButton::Right => {
                    self.camera
                        .accelerate_z(delta_y)
                        .accelerate_x(delta_x)
                        .update();
                    self.update_camera_uniforms();
                    true
                }
                MouseButton::Left => {
                    self.camera.rotate(delta_y, delta_x).update();
                    self.update_camera_uniforms();
                    true
                }
                _ => false,
            };
        }

        let (x, y, z) = match self.input_state.mouse.unwrap() {
            MouseButton::Left => {
                let x = (self.camera.right * delta_x * speed).x;
                let y = (self.camera.right.cross(self.camera.front) * delta_y * speed).y;
                let z = 0.0;

                (x, y, z)
            }
            MouseButton::Right => {
                let x = 0.0;
                let y = 0.0;
                let z = (self.camera.front * delta_y * speed).z;

                (x, y, z)
            }
            _ => todo!(),
        };

        if self.input_state.ctrl {
            if let Some(ref mut light) = self.light {
                light.translate(x, y, z);
                self.update_light();
            }
        } else {
            if self.input_state.shift {
                self.model.rotate_x(delta_y);
                self.model.rotate_y(delta_x);
            } else {
                self.model.translate(x, y, z);
            }
            self.update_model_uniforms();
        }

        true
    }

    fn should_render_key_input(&mut self, state: &ElementState, keycode: &VirtualKeyCode) -> bool {
        use camera_consts::*;

        self.input_state.modifiers(state, keycode);

        if *state != ElementState::Pressed {
            match keycode {
                VirtualKeyCode::R => {
                    println!("Reloading shaders.");
                    self.program = T::create_program();
                    self.uniforms = T::Uniforms::new(&self.program);
                    self.update_uniforms();
                }
                VirtualKeyCode::P => {
                    if self.camera.projection.is_orthographic() {
                        self.camera.projection.replace(Projection::perspective(
                            FOV,
                            self.size.width / self.size.height,
                            NEAR,
                            FAR,
                        ));
                    } else {
                        self.camera
                            .projection
                            .replace(Projection::orthographic(ORHO));
                    }
                    self.update_camera_uniforms();
                }
                _ => return false,
            }

            return true;
        }

        let input_ctx = KeyInputContext {
            state,
            keycode,
            light: self.light.as_mut(),
            uniforms: &mut self.uniforms,
            input_state: &self.input_state,
        };

        if self.controller.handled_key_input(input_ctx) {
            self.update_uniforms();
            return true;
        }

        false
    }

    pub fn draw(&self) {
        self.program.use_program();
        self.model.draw();
        if let Some(ref light) = self.light {
            light.draw();
        }
    }
}

pub trait SOUniforms {
    fn new(program: &Program) -> Self;
    fn update_camera(&self, camera: &FlyCamera);
    fn update_model(&self, model: &Mesh);
    fn update_light(&self, light: &Light);
}

pub mod so_uniforms {
    use super::*;

    pub struct Uniforms {
        // View transformation matrix.
        pub view: i32,
        // Projection matrix.
        pub proj: i32,
        // Model transformation matrix.
        pub model: i32,
        // Light position.
        pub light: i32,
    }

    impl SOUniforms for Uniforms {
        fn new(program: &Program) -> Self {
            Self {
                view: program.get_uniform("u_view_t\0"),
                proj: program.get_uniform("u_proj_t\0"),
                model: program.get_uniform("u_model_t\0"),
                light: program.get_uniform("u_light_t\0"),
            }
        }

        fn update_model(&self, model: &Mesh) {
            opengl!(gl::UniformMatrix4fv(
                self.model,
                1,
                gl::FALSE,
                model.model_to_world.as_ref() as *const _
            ));
        }

        fn update_camera(&self, camera: &FlyCamera) {
            opengl! {
                gl::UniformMatrix4fv(self.view, 1, gl::FALSE, camera.get_view().as_ref() as *const _);
                gl::UniformMatrix4fv(self.proj, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
            }
        }

        fn update_light(&self, light: &Light) {
            opengl!(gl::UniformMatrix4fv(
                self.light,
                1,
                gl::FALSE,
                light.position.as_ref() as *const _
            ));
        }
    }
}

pub struct KeyInputContext<'a, U> {
    pub state: &'a ElementState,
    pub keycode: &'a VirtualKeyCode,
    pub light: Option<&'a mut Light>,
    pub uniforms: &'a mut U,
    pub input_state: &'a InputState,
}

pub struct CursorInputContext<'a, U> {
    pub light: Option<&'a mut Light>,
    pub uniforms: &'a mut U,
    pub input_state: &'a InputState,
    pub mouse_delta: (f32, f32),
}

pub trait SOController {
    type Uniforms: SOUniforms;

    fn create_program() -> Program;
    fn load_mesh() -> Mesh;
    fn create_light() -> Option<Light> {
        None
    }
    #[allow(unused_variables)]
    fn handled_key_input(&mut self, context: KeyInputContext<'_, Self::Uniforms>) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn handled_cursor_move(&mut self, context: CursorInputContext<'_, Self::Uniforms>) -> bool {
        false
    }
}
