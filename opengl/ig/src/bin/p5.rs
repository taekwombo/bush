//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=5

use gluty::*;
use ig::*;
use winit::dpi::PhysicalSize;
use winit::event::*;

mod render_to_texture {
    use super::*;

    pub struct TextureRender {
        pub texture: Texture,
        pub model: Mesh,
        pub light: Light,
        pub camera: FlyCamera,
        framebuffer: u32,
        depthbuffer: u32,
        original_framebuffer: u32,
        program: Program,
    }

    impl TextureRender {
        pub fn new(size: &PhysicalSize<u32>) -> Self {
            debug_assert!(size.height < std::i32::MAX as u32);
            debug_assert!(size.width < std::i32::MAX as u32);

            let (framebuffer, depthbuffer) = Self::create_frame_and_depth_buffers();

            let mut this = Self {
                framebuffer,
                depthbuffer,
                original_framebuffer: Self::get_current_framebuffer() as u32,
                texture: Self::create_texture(size),
                camera: Self::create_camera(size),
                model: Self::create_model(),
                program: Self::create_program(),
                light: Light::new(),
            };

            this.init_framebuffer();
            this.base_setup();
            this.update_uniforms();

            this
        }

        fn base_setup(&mut self) {
            self.camera.goto(0.0, 0.0, 60.0).update();
            self.light.translate(20.0, 20.0, 30.0);
        }

        pub fn draw(&self) {
            let diffuse_tex = self
                .model
                .material
                .as_ref()
                .expect("Model has material.")
                .diffuse_texture
                .as_ref()
                .expect("Model has diffuse texture.");

            self.bind_framebuffer();

            opengl! {
                gl::ClearColor(0.4, 0.1, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            diffuse_tex.bind();
            self.program.use_program();
            self.model.draw();
            diffuse_tex.unbind();

            self.unbind_framebuffer();
        }

        fn bind_framebuffer(&self) {
            opengl!(gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.framebuffer));
            opengl!(gl::Viewport(
                0,
                0,
                self.texture.width as i32,
                self.texture.height as i32
            ));
        }

        fn unbind_framebuffer(&self) {
            opengl!(gl::BindFramebuffer(
                gl::DRAW_FRAMEBUFFER,
                self.original_framebuffer
            ));
        }

        fn init_framebuffer(&self) {
            let width = self.texture.width as i32;
            let height = self.texture.height as i32;
            let depthbuffer = self.depthbuffer;
            let framebuffer = self.framebuffer;

            opengl! {
                gl::Enable(gl::DEPTH_TEST);

                gl::BindRenderbuffer(gl::RENDERBUFFER, depthbuffer);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);

                gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
                gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, self.texture.gl_id, 0);
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depthbuffer);
                check_framebuffer_operation(1);

                gl::DrawBuffers(1, [gl::COLOR_ATTACHMENT0].as_ptr().cast());


                check_framebuffer_operation(2);

                gl::BindFramebuffer(gl::FRAMEBUFFER, self.original_framebuffer);
            }
        }

        pub fn update_uniforms(&self) {
            self.program.use_program();

            let u_view = self.program.get_uniform("u_view_t\0");
            let u_proj = self.program.get_uniform("u_proj_t\0");
            let u_model = self.program.get_uniform("u_model_t\0");
            let u_light = self.program.get_uniform("u_light_pos\0");
            let u_texture = self.program.get_uniform("u_tex_diffuse\0");
            let diffuse_tex = self
                .model
                .material
                .as_ref()
                .expect("Model has material.")
                .diffuse_texture
                .as_ref()
                .expect("Model has diffuse texture.");

            opengl! {
                gl::UniformMatrix4fv(u_view,  1, gl::FALSE, self.camera.get_view().as_ref() as *const _);
                gl::UniformMatrix4fv(u_proj,  1, gl::FALSE, self.camera.get_proj().as_ref() as *const _);
                gl::UniformMatrix4fv(u_model, 1, gl::FALSE, self.model.model_to_world.as_ref() as *const _);
                gl::UniformMatrix4fv(u_light, 1, gl::FALSE, self.light.position.as_ref() as *const _);
                gl::Uniform1i(u_texture, diffuse_tex.slot as i32);
            }
        }

        fn create_program() -> Program {
            let (vert, frag) = assets!(
                "./shaders/p5/tex_render.vert",
                "./shaders/p5/tex_render.frag",
            );
            Program::default()
                .shader(frag.get(), gl::FRAGMENT_SHADER)
                .shader(vert.get(), gl::VERTEX_SHADER)
                .link()
        }

        fn create_texture(size: &PhysicalSize<u32>) -> Texture {
            let tex_width = size.width;
            let tex_height = size.height;
            let texture = Texture::new(gl::TEXTURE_2D, 9, tex_width, tex_height);

            texture.bind();
            opengl!(gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as i32,
                tex_width as i32,
                tex_height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                std::ptr::null(),
            ));
            texture.unbind();

            texture
        }

        fn create_camera(size: &PhysicalSize<u32>) -> FlyCamera {
            FlyCamera::new(|| {
                let fov = 60.0;
                let aspect = size.width as f32 / size.height as f32;
                let near = 0.001;
                let far = 1000.0;
                Projection::perspective(fov, aspect, near, far)
            })
        }

        fn create_model() -> Mesh {
            let (obj_asset, mtl) = assets!("./teapot.obj", "./teapot.mtl");
            let material_assets = assets!(["./brick.png", "./brick-specular.png",]);
            let mut obj = Obj::new();
            obj.parse_obj(&obj_asset).parse_mtl(&mtl, &material_assets);

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

        fn create_frame_and_depth_buffers() -> (u32, u32) {
            let mut framebuffer = 0;
            let mut depthbuffer = 0;

            opengl! {
                gl::GenFramebuffers(1, &mut framebuffer);
                gl::GenRenderbuffers(1, &mut depthbuffer);
            }

            debug_assert!(framebuffer != 0);
            debug_assert!(depthbuffer != 0);

            (framebuffer, depthbuffer)
        }

        fn get_current_framebuffer() -> i32 {
            let mut framebuffer: i32 = 0;
            opengl!(gl::GetIntegerv(
                gl::DRAW_FRAMEBUFFER_BINDING,
                &mut framebuffer
            ));

            framebuffer
        }
    }

    fn check_framebuffer_operation(step: u8) {
        if unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) } != gl::FRAMEBUFFER_COMPLETE {
            eprintln!(">> Framebuffer check failed at step {step}.");
            panic!();
        }
    }
}

#[derive(Debug)]
pub struct Uniforms {
    view: i32,
    proj: i32,
    model: i32,
    texture: i32,
}

impl SOUniforms for Uniforms {
    fn new(program: &Program) -> Self {
        Self {
            view: program.get_uniform("u_view_t\0"),
            proj: program.get_uniform("u_proj_t\0"),
            model: program.get_uniform("u_model_t\0"),
            texture: program.get_uniform("u_texture\0"),
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

    fn update_light(&self, _light: &Light) {}
}

struct Ctrl {
    tex_render: render_to_texture::TextureRender,
    size: PhysicalSize<u32>,
}

impl Ctrl {
    fn new(size: &PhysicalSize<u32>) -> Self {
        Self {
            size: *size,
            tex_render: render_to_texture::TextureRender::new(size),
        }
    }
}

impl SOController for Ctrl {
    type Uniforms = Uniforms;

    fn create_program(&self) -> Option<Program> {
        let (vert, frag) = assets!("./shaders/p5/shader.vert", "./shaders/p5/shader.frag",);
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
        #[rustfmt::skip]
        let positions: &[f32] = &[
            -40.0,  40.0, 0.0, 1.0,
             40.0,  40.0, 1.0, 1.0,
             40.0, -40.0, 1.0, 0.0,
            -40.0, -40.0, 0.0, 0.0,
        ];
        #[rustfmt::skip]
        let indices: &[u32] = &[
            0, 2, 1,
            0, 3, 2,
        ];

        Mesh::new(positions, indices, |attrs| {
            attrs
                .add::<f32>(0, 2, gl::FLOAT)
                .add::<f32>(1, 2, gl::FLOAT);
        })
    }

    fn handled_cursor_move(&mut self, ctx: CursorInputContext<'_, Self::Uniforms>) -> bool {
        if !(ctx.input_state.ctrl && ctx.input_state.shift) {
            return false;
        }

        let (delta_x, delta_y) = ctx.mouse_delta;
        if let Some(mouse) = ctx.input_state.mouse {
            if mouse == MouseButton::Left {
                self.tex_render
                    .camera
                    .accelerate_z(delta_y)
                    .accelerate_x(delta_x)
                    .update();
            } else {
                self.tex_render.light.translate(delta_x, delta_y, 0.0);
            }
            self.tex_render.update_uniforms();
            self.tex_render.draw();
            // Restore viewport to the "outside" window size.
            opengl!(gl::Viewport(
                0,
                0,
                self.size.width as i32,
                self.size.height as i32
            ));
        }

        true
    }
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let ctrl = Ctrl::new(&size);

    ctrl.tex_render.draw();
    let tex_slot = ctrl.tex_render.texture.slot;

    let mut project = SOProject::new(ctrl, size);

    project.controller.tex_render.texture.bind();
    project.program.use_program();
    opengl!(gl::Uniform1i(project.uniforms.texture, tex_slot as i32));

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.update_uniforms();

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
                    gl::ClearColor(0.2, 0.2, 0.4, 1.0);
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
                    project.controller.size = size;
                }
                event => project.handle_window_events(&event, control_flow, &window),
            },
            _ => (),
        }
    });
}
