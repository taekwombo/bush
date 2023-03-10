use gluty::{opengl, gl, Program, Mesh, obj, FlyCamera, Projection, glam::Mat4};
use gluty::winit::event::*;
use gluty::winit::dpi::{PhysicalSize, PhysicalPosition};
use std::time::Instant;

const NEAR: f32 = 0.01;
const FAR: f32 = 1000.0;
const FOV: f32 = 60.0;
const ORHO: [f32; 6] = [
    -50.0, 50.0,
    -50.0, 50.0,
    NEAR, FAR,
];

fn create_program() -> Result<Program, ()> {
    let mut program = Program::create();

    program
        .attach_shader_source("./shaders/p2/shader.vert", gl::VERTEX_SHADER)
        .and_then(|p| p.attach_shader_source("./shaders/p2/shader.frag", gl::FRAGMENT_SHADER))
        .and_then(|p| p.link())?;

    Ok(program)
}

pub fn get_model_path() -> String {
    let path_arg = std::env::args().skip(1).next();

    path_arg.map_or_else(
        || String::from("./resources/teapot.obj"),
        |v| v,
    )
}

pub struct Project {
    pub prog: Program,
    pub mouse: Option<MouseButton>,
    pub mesh: Mesh,
    pub camera: FlyCamera,
    uniforms: Vec<(&'static str, Box<dyn Fn(&Self) -> Mat4>, i32)>,
    size: PhysicalSize<f32>,
    movement_timestamp: Option<Instant>,
    cursor_position: Option<PhysicalPosition<f32>>,
}

impl Project {
    pub fn new(win_size: PhysicalSize<u32>) -> Self {
        let (v, i) = obj::load(&get_model_path());
        let size = PhysicalSize::new(win_size.width as i32 as f32, win_size.height as i32 as f32);

        Self {
            size,
            prog: create_program().unwrap(),
            mouse: None,
            mesh: Mesh::new(&v, &i, |a| {
                a.add::<f32>(0, 3, gl::FLOAT).add::<f32>(1, 3, gl::FLOAT);
            }),
            camera: FlyCamera::new(|| Projection::perspective(FOV, size.width / size.height, NEAR, FAR)),
            uniforms: Vec::new(),
            movement_timestamp: None,
            cursor_position: None,
        }
    }

    pub fn add_uniform(&mut self, name: &'static str, getter: Box<dyn Fn(&Self) -> Mat4>) -> &mut Self {
        self.uniforms.push((name, getter, self.prog.get_uniform(name)));
        self
    }

    pub fn refresh_uniform_locations(&mut self) -> &mut Self {
        for i in 0..self.uniforms.len() {
            let uni = &mut self.uniforms[i];
            uni.2 = self.prog.get_uniform(uni.0);
        }
        self
    }

    pub fn upload_uniforms(&self) -> &Self {
        for i in 0..self.uniforms.len() {
            let uni = &self.uniforms[i];

            debug_assert!(uni.2 >= 0);

            opengl! {
                gl::UniformMatrix4fv(
                    uni.2,        
                    1,
                    gl::FALSE,
                    uni.1(self).as_ref() as *const _,
                );
            }
        }

        self
    }

    pub fn lmb(&self) -> bool {
        if let Some(MouseButton::Left) = self.mouse { true } else { false }
    }

    pub fn rmb(&self) -> bool {
        if let Some(MouseButton::Right) = self.mouse { true } else { false }
    }

    pub fn handle_key_code(&mut self, state: &ElementState, keycode: &VirtualKeyCode) -> &mut Self {
        if *state != ElementState::Pressed {
            match keycode {
                VirtualKeyCode::W | VirtualKeyCode::S | VirtualKeyCode::A | VirtualKeyCode::D => {
                    self.movement_timestamp = None;
                },
                _ => (),
            }

            return self;
        }

        match keycode {
            // Reload shaders on R key press.
            VirtualKeyCode::R => match create_program() {
                Ok(prog) => {
                    println!("Reloading shaders.");
                    self.prog = prog;
                    self.prog.use_program();
                    self.refresh_uniform_locations().upload_uniforms();
                },
                Err(_) => {
                    println!("Could not reload program.");
                },
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
                    self.camera.projection.replace(Projection::orthographic(ORHO));
                }
                self.upload_uniforms();
            },
            // Initial movement event.
            VirtualKeyCode::W
                | VirtualKeyCode::S
                | VirtualKeyCode::A
                | VirtualKeyCode::D if self.movement_timestamp.is_none() => {
                self.movement_timestamp = Some(Instant::now());
            }
            VirtualKeyCode::W | VirtualKeyCode::S => {
                let elapsed = self.movement_timestamp.as_ref().unwrap().elapsed().as_secs_f32();
                let direction = if *keycode == VirtualKeyCode::W { -1.0 } else { 1.0 };
                self.movement_timestamp.replace(Instant::now());

                self.camera.accelerate_z(elapsed * direction).update();
                self.upload_uniforms();
            },
            VirtualKeyCode::A | VirtualKeyCode::D => {
                let elapsed = self.movement_timestamp.as_ref().unwrap().elapsed().as_secs_f32();
                let direction = if *keycode == VirtualKeyCode::A { -1.0 } else { 1.0 };
                self.movement_timestamp.replace(Instant::now());

                self.camera.accelerate_x(elapsed * direction).update();
                self.upload_uniforms();
            },
            _ => (),
        };
        self
    }

    pub fn handle_mouse_btn(&mut self, state: &ElementState, button: &MouseButton) -> &mut Self {
        if *state == ElementState::Released {
            let _ = self.mouse.take();
            let _ = self.cursor_position.take();
            return self;
        }

        match button {
            MouseButton::Left | MouseButton::Right => {
                self.mouse.replace(*button);
            }
            _ => (),
        };

        self
    }

    pub fn handle_cursor_move(&mut self, pos: &PhysicalPosition<f64>) -> bool {
        if self.mouse.is_none() {
            return false;
        }

        if self.cursor_position.is_none() {
            self.cursor_position.replace(PhysicalPosition::new(
                pos.x as f32,
                pos.y as f32,
            ));

            return false;
        }

        let prev = self.cursor_position.as_ref().unwrap();
        let x = pos.x as f32;
        let y = pos.y as f32;
        let delta_x = (x - prev.x) / (self.size.width / 100.0);
        let delta_y = (y - prev.y) / (self.size.height / -100.0);
        self.cursor_position.replace(PhysicalPosition::new(x, y));

        match self.mouse.unwrap() {
            MouseButton::Right => {
                self.camera.accelerate_z(delta_y).accelerate_x(delta_x).update();
            },
            MouseButton::Left => {
                self.camera.rotate(delta_y, delta_x).update();
            },
            _ => (),
        }

        self.upload_uniforms();

        return true;
    }
}
