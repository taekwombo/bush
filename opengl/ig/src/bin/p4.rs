//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=4

use gluty::*;
use ig::*;
use winit::event::*;

#[derive(Debug)]
struct Uniforms {
    model: i32,
    view: i32,
    proj: i32,
    light_pos: i32,
    textures: i32,
    specular_component: i32,
    col_ambient: i32,
    col_diffuse: i32,
    col_specular: i32,
    tex_ambient: i32,
    tex_diffuse: i32,
    tex_specular: i32,
    display_textures: u32,
}

impl SOUniforms for Uniforms {
    fn new(program: &Program) -> Self {
        Self {
            view: program.get_uniform("u_view_t\0"),
            proj: program.get_uniform("u_proj_t\0"),
            model: program.get_uniform("u_model_t\0"),
            light_pos: program.get_uniform("u_light_pos\0"),
            textures: program.get_uniform("u_display_textures\0"),
            specular_component: program.get_uniform("u_specular_component\0"),
            col_ambient: program.get_uniform("u_ambient_color\0"),
            col_diffuse: program.get_uniform("u_diffuse_color\0"),
            col_specular: program.get_uniform("u_specular_color\0"),
            tex_ambient: program.get_uniform("u_tex_ambient\0"),
            tex_diffuse: program.get_uniform("u_tex_diffuse\0"),
            tex_specular: program.get_uniform("u_tex_specular\0"),
            display_textures: 1,
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
        opengl!(gl::Uniform1ui(self.textures, self.display_textures));

        if let Some(ref material) = model.material {
            opengl!(gl::Uniform3fv(
                self.col_ambient,
                1,
                material.ambient_color.as_ptr()
            ));
            opengl!(gl::Uniform3fv(
                self.col_diffuse,
                1,
                material.diffuse_color.as_ptr()
            ));
            opengl!(gl::Uniform3fv(
                self.col_specular,
                1,
                material.specular_color.as_ptr()
            ));
            opengl!(gl::Uniform1f(
                self.specular_component,
                material.specular_component
            ));
            if let Some(ref tex) = material.ambient_texture {
                opengl!(gl::Uniform1i(self.tex_ambient, tex.slot as i32));
                tex.bind();
            }
            if let Some(ref tex) = material.diffuse_texture {
                opengl!(gl::Uniform1i(self.tex_diffuse, tex.slot as i32));
                tex.bind();
            }
            if let Some(ref tex) = material.specular_texture {
                opengl!(gl::Uniform1i(self.tex_specular, tex.slot as i32));
                tex.bind();
            }
        }
    }

    fn update_light(&self, light: &Light) {
        opengl!(gl::UniformMatrix4fv(
            self.light_pos,
            1,
            gl::FALSE,
            light.position.as_ref() as *const _
        ));
    }
}

struct Ctrl;

impl SOController for Ctrl {
    type Uniforms = Uniforms;

    fn create_program(&self) -> Option<Program> {
        create_program(Some("./shaders/p4/shader")).ok()
    }

    fn load_mesh() -> Mesh {
        let mut obj = Obj::new();
        let path = get_model_path();
        obj.parse(&path);

        let (vbo, ebo) = obj.build(&BuildOptions::with_tex());
        let mut mesh = Mesh::new(&vbo, &ebo, |attrs| {
            attrs
                .add::<f32>(0, 3, gl::FLOAT)
                .add::<f32>(1, 3, gl::FLOAT)
                .add::<f32>(2, 3, gl::FLOAT);
        });

        mesh.material.replace(obj.material);

        mesh
    }

    fn create_light() -> Option<Light> {
        Some(Light::new())
    }

    fn handled_key_input(&mut self, ctx: KeyInputContext<'_, Self::Uniforms>) -> bool {
        if *ctx.state == ElementState::Pressed && matches!(ctx.keycode, VirtualKeyCode::Space) {
            ctx.uniforms.display_textures = if ctx.uniforms.display_textures == 1 {
                0
            } else {
                1
            };
            return true;
        }

        false
    }
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
        gl::ClearColor(0.2, 0.2, 0.4, 1.0);
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

                    project.draw();

                    surface.swap_buffers(&context).expect("I want to swap!");
                }
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
