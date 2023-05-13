//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=8

use gluty::{
    opengl, uniforms, Program,
    gl, Mesh, FlyCamera, Glindow, Texture
};
use ig::*;

uniforms!(Uniforms; u_texture, u_model_t, u_view_t, u_proj_t, u_light_world_t);

impl SOUniforms for Uniforms {
    fn new(program: &Program) -> Self {
        Uniforms::new(program)
    }

    fn update_camera(&self, camera: &FlyCamera) {
        opengl! {
            gl::UniformMatrix4fv(self.u_view_t, 1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_proj_t, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
        }
    }

    fn update_model(&self, model: &Mesh) {
        opengl!(gl::UniformMatrix4fv(
            self.u_model_t,
            1,
            gl::FALSE,
            model.model_to_world.as_ref() as *const _
        ));
    }

    fn update_light(&self, light: &Light) {
        opengl!(gl::UniformMatrix4fv(
            self.u_light_world_t,
            1,
            gl::FALSE,
            light.position.as_ref() as *const _
        ));
    }
}

enum ProjectStep {
    One,
}

struct Tesselation {
    step: ProjectStep,
}

impl SOController for Tesselation {
    type Uniforms = Uniforms;

    fn create_program(&self) -> Option<Program> {
        let source = match &self.step {
            ProjectStep::One => "./shaders/p8/step_one",
        };

        create_program(Some(source)).ok()
    }

    fn create_light() -> Option<Light> {
        Some(Light::new())
    }

    fn load_mesh() -> Mesh {
        let vbo_data: &[f32] = &[
            // Position        Normals         Texture Coord 
            -1.0,  0.0,  1.0,  0.0, 1.0, 0.0,  0.0, 1.0,
             1.0,  0.0,  1.0,  0.0, 1.0, 0.0,  1.0, 1.0,
             1.0,  0.0, -1.0,  0.0, 1.0, 0.0,  1.0, 0.0,
            -1.0,  0.0, -1.0,  0.0, 1.0, 0.0,  0.0, 0.0,
        ];
        let ebo_data: &[u32] = &[
            0, 2, 1,
            0, 3, 2,
        ];

        Mesh::new(&vbo_data, &ebo_data, |attrs| {
            attrs.add::<f32>(0, 3, gl::FLOAT);
            attrs.add::<f32>(1, 3, gl::FLOAT);
            attrs.add::<f32>(2, 2, gl::FLOAT);
            // Should also pass tangent vector - out of laziness it's hardcoded in fragment shader.
        })
    }
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let texture = {
        let image = Texture::load_file("./resources/teapot_normal.png", false).unwrap();
        let (width, height) = image.dimensions();
        let tex = Texture::new(gl::TEXTURE_2D, 0, width, height);
        tex.bind();
        tex.data(image.as_raw(), None);
        tex
    };
    let mut project = SOProject::new(Tesselation { step: ProjectStep::One }, size);

    project.camera.goto(0.0, 20.0, 40.0).update();
    project.model.scale(20.0, 20.0, 20.0);
    project
        .light
        .as_mut()
        .map(|light| light.translate(0.0, 30.0, -20.0));
    project.update_uniforms();

    opengl! {
        gl::Uniform1i(project.uniforms.u_texture, texture.slot as i32);
        gl::ClearColor(0.3, 0.10, 0.15, 1.0);
        gl::Enable(gl::DEPTH_TEST);
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
        use gluty::winit::event::*;
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
        }
    });
}
