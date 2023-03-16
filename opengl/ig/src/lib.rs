use gluty::winit::dpi::{PhysicalPosition, PhysicalSize};
use gluty::winit::event::*;
use gluty::{gl, opengl, FlyCamera, Program, Projection};

mod input;
mod light;

pub use input::*;
pub use light::*;

const NEAR: f32 = 0.01;
const FAR: f32 = 1000.0;
const FOV: f32 = 60.0;
const ORHO: [f32; 6] = [-50.0, 50.0, -50.0, 50.0, NEAR, FAR];

#[allow(clippy::result_unit_err)]
pub fn create_program(dir: Option<&'static str>) -> Result<Program, ()> {
    let mut program = Program::create();

    if let Some(dir) = dir {
        program
            .attach_shader_source(format!("{}/shader.vert", dir), gl::VERTEX_SHADER)
            .and_then(|p| {
                p.attach_shader_source(format!("{}/shader.frag", dir), gl::FRAGMENT_SHADER)
            })
            .and_then(|p| p.link())?;
    }

    Ok(program)
}

pub fn get_model_path() -> String {
    let path_arg = std::env::args().nth(1);

    path_arg.map_or_else(|| String::from("./resources/teapot.obj"), |v| v)
}

type CreateProgram = fn() -> Result<Program, ()>;

pub trait Controller {
    fn state(&mut self) -> &mut InputState;
    fn upload_uniforms(&self, camera: &FlyCamera);
    fn program_changed(&mut self, program: &Program);
}

pub struct Project<T: Controller> {
    pub prog: Program,
    pub camera: FlyCamera,
    pub controller: T,
    size: PhysicalSize<f32>,
    create_program: CreateProgram,
}

impl<T: Controller> Project<T> {
    pub fn new(controller: T, win_size: PhysicalSize<u32>, create_program: CreateProgram) -> Self {
        let size = PhysicalSize::new(win_size.width as i32 as f32, win_size.height as i32 as f32);

        let mut result = Self {
            size,
            create_program,
            controller,
            prog: create_program().expect("Program must be created."),
            camera: FlyCamera::new(|| {
                Projection::perspective(FOV, size.width / size.height, NEAR, FAR)
            }),
        };

        result.controller.program_changed(&result.prog);
        result.upload_uniforms();

        result
    }

    pub fn ctrl(&mut self) -> &mut T {
        &mut self.controller
    }

    pub fn upload_uniforms(&mut self) -> &mut Self {
        self.prog.use_program();
        self.controller.upload_uniforms(&self.camera);
        self
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> &mut Self {
        self.size = PhysicalSize::new(size.width as i32 as f32, size.height as i32 as f32);
        self.camera
            .projection
            .resize(self.size.width / self.size.height);
        opengl! {
            gl::Viewport(
                0, 0,
                size.width as i32,
                size.height as i32,
            );
        }

        self.upload_uniforms();

        self
    }

    pub fn handle_key_code(&mut self, input: &KeyboardInput) -> bool {
        let Some(keycode) = input.virtual_keycode else {
            return false;
        };

        let state = self.controller.state();
        state.modifiers(&input.state, &keycode);

        if input.state != ElementState::Pressed {
            return false;
        }

        match keycode {
            // Reload shaders on R key press.
            VirtualKeyCode::R => match (self.create_program)() {
                Ok(prog) => {
                    println!("Reloading shaders.");
                    self.prog = prog;
                    self.controller.program_changed(&self.prog);
                    self.upload_uniforms();
                }
                Err(_) => {
                    println!("Could not reload program.");
                }
            },
            // Toggle projection matrix.
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

                self.upload_uniforms();
            }
            // Initial movement event.
            VirtualKeyCode::W | VirtualKeyCode::S | VirtualKeyCode::A | VirtualKeyCode::D => {
                match state.key_movement(&keycode) {
                    None => return false,
                    Some((axis, change)) => {
                        if let MovementAxis::X = axis {
                            self.camera.accelerate_x(change).update();
                        } else {
                            self.camera.accelerate_z(change).update();
                        }
                        self.upload_uniforms();
                    }
                }
            }
            _ => return false,
        };

        true
    }

    pub fn handle_cursor_move(&mut self, pos: &PhysicalPosition<f64>) -> bool {
        let state = self.controller.state();

        if state.mouse.is_none() {
            return false;
        }

        let Some((delta_x, delta_y)) = state.cursor_move(pos) else {
            return false;
        };

        match state.mouse.unwrap() {
            MouseButton::Right => {
                self.camera
                    .accelerate_z(delta_y)
                    .accelerate_x(delta_x)
                    .update();
            }
            MouseButton::Left => {
                self.camera.rotate(delta_y, delta_x).update();
            }
            _ => (),
        }

        self.upload_uniforms();

        true
    }
}
