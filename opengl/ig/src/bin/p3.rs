//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=3

use gluty::winit::event::*;
use gluty::{assets, gl, opengl, FlyCamera, Glindow, Mesh, Obj, Program};
use ig::*;

struct Uniforms {
    model: i32,
    view: i32,
    proj: i32,
    lighting: i32,
    light_pos: i32,
}

impl SOUniforms for Uniforms {
    fn new(program: &Program) -> Self {
        Self {
            view: program.get_uniform("u_view_t\0"),
            proj: program.get_uniform("u_proj_t\0"),
            model: program.get_uniform("u_model_t\0"),
            lighting: program.get_uniform("u_lighting\0"),
            light_pos: program.get_uniform("u_light_pos\0"),
        }
    }

    fn update_camera(&self, camera: &FlyCamera) {
        opengl! {
            gl::UniformMatrix4fv(self.view, 1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.proj, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
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

    fn update_light(&self, light: &Light) {
        opengl! {
            gl::UniformMatrix4fv(self.light_pos, 1, gl::FALSE, light.position.as_ref() as *const _);
            gl::Uniform4f(self.lighting, light.color.x, light.color.y, light.color.z, light.color.w);
        }
    }
}

struct Ctrl;

impl SOController for Ctrl {
    type Uniforms = Uniforms;

    fn create_program(&self) -> Option<Program> {
        let (vert, frag) = assets!("./shaders/p3/shader.vert", "./shaders/p3/shader.frag",);
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

    fn create_light() -> Option<Light> {
        let mut light = Light::new();
        light.color.w = 0.0;

        Some(light)
    }

    fn handled_key_input(&mut self, ctx: KeyInputContext<'_, Self::Uniforms>) -> bool {
        if *ctx.state == ElementState::Released {
            return false;
        }

        let Some(light) = ctx.light else {
            return false;
        };

        match ctx.keycode {
            VirtualKeyCode::Key1 => {
                light.color.w = Lighting::Normal as i32 as f32;
            }
            VirtualKeyCode::Key2 => {
                light.color.w = Lighting::Ambient as i32 as f32;
            }
            VirtualKeyCode::Key3 => {
                light.color.w = Lighting::Diffuse as i32 as f32;
            }
            VirtualKeyCode::Key4 => {
                light.color.w = Lighting::Specular as i32 as f32;
            }
            VirtualKeyCode::Key5 => {
                light.color.w = Lighting::Phong as i32 as f32;
            }
            VirtualKeyCode::Key6 => {
                light.color.w = Lighting::Blinn as i32 as f32;
            }
            _ => return false,
        }

        true
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
    let mut project = SOProject::new(Ctrl, size);

    project.camera.goto(0.0, 0.0, 60.0).update();
    project
        .light
        .as_mut()
        .map(|light| light.translate(-20.0, 20.0, 30.0));
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
