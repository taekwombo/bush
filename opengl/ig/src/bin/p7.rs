//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=7

use ig::*;
use gluty::{
    gl,
    Mesh, Obj, BuildOptions, Glindow, Program, Texture,
    FlyCamera, Projection, opengl, uniforms,
    winit::dpi::PhysicalSize,
    glam::{Vec3, Mat4},
};

mod dir_light {
    //! Light with view and projection transformation matrices.
    //! Always looks at the center of the World. 

    use super::*;

    pub struct DirLight {
        pub position: Vec3,
        pub view: Mat4,
        pub proj: Mat4,
        pub light: Light,
    }

    impl DirLight {
        pub fn new(size: PhysicalSize<u32>) -> Self {
            use camera_consts::*;
            let size_f = size_u_to_f32(&size);
            let proj = Projection::perspective(FOV, size_f.width / size_f.height, NEAR, FAR).matrix.clone();
            let position = Vec3::new(10.0, 10.0, 10.0);
            let view = Mat4::IDENTITY;

            let mut light = Self {
                proj,
                view,
                position,
                light: Light::new(),
            };
            light.update();
            light
        }

        pub fn update_uniforms(&mut self, u_view: &Mat4, u_proj: &Mat4) {
            self.light.position = self.view.inverse();
            self.light.update_uniforms(u_view, u_proj);
        }

        pub fn draw(&self) {
            self.light.draw();
        }

        pub fn get_view(&self) -> &Mat4 {
            &self.view
        }

        pub fn get_proj(&self) -> &Mat4 {
            &self.proj
        }

        pub fn update(&mut self) {
            self.view = Mat4::look_at_rh(self.position, Vec3::ZERO, Vec3::Y);
        }

        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            self.position.x += x;
            self.position.y += y;
            self.position.z += z;
            self.update();
        }
        
        pub fn goto(&mut self, x: f32, y: f32, z: f32) {
            self.position.x = x;
            self.position.y = y;
            self.position.z = z;
            self.update();
        }
    }
}
mod shadow {
    use super::*;

    uniforms!(ShadowUniforms; u_model_t, u_view_t, u_proj_t);

    pub struct Shadow {
        pub framebuffer: u32,
        pub program: Program,
        pub uniforms: ShadowUniforms,
        pub texture: Texture,
        pub original_framebuffer: i32,
    }

    impl Shadow {
        pub fn new(size: &PhysicalSize<u32>) -> Self {
            let mut framebuffer: u32 = 0;
            let mut original_framebuffer: i32 = 0;
            let texture = Self::create_texture(size);

            opengl!{
                gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut original_framebuffer);
                gl::GenFramebuffers(1, &mut framebuffer);

                // Configure framebuffer
                gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
                gl::FramebufferTexture(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    texture.gl_id,
                    0,
                );
                gl::DrawBuffer(gl::NONE);
                gl::ReadBuffer(gl::NONE);
            };

            let check = opengl!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER));
            if check != gl::FRAMEBUFFER_COMPLETE {
                eprintln!("Framebuffer check failed.");
                std::process::exit(1);
            }

            opengl!(gl::BindFramebuffer(
                gl::FRAMEBUFFER,
                original_framebuffer.try_into().unwrap(),
            ));

            let program = create_shadow_program().expect("Shadow program compiles");
            let uniforms = ShadowUniforms::new(&program);

            Self {
                program,
                uniforms,
                framebuffer,
                texture,
                original_framebuffer,
            }
        }

        pub fn clear(&self) {
            opengl!(gl::Clear(gl::DEPTH_BUFFER_BIT));
        }

        pub fn bind(&self, size: &PhysicalSize<u32>) {
            opengl! {
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
                gl::Viewport(
                    0,
                    0,
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                );
            }
        }

        pub fn unbind(&self, size: &PhysicalSize<u32>) {
            opengl! {
                gl::BindFramebuffer(
                    gl::FRAMEBUFFER,
                    self.original_framebuffer.try_into().unwrap(),
                );
                gl::Viewport(
                    0,
                    0,
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                );
            }
        }

        pub fn update_uniforms(&self, view: &Mat4, proj: &Mat4) {
            self.program.use_program();
            opengl! {
                gl::UniformMatrix4fv(
                    self.uniforms.u_view_t,
                    1,
                    gl::FALSE,
                    view.as_ref() as *const _,
                );
                gl::UniformMatrix4fv(
                    self.uniforms.u_proj_t,
                    1,
                    gl::FALSE,
                    proj.as_ref() as *const _,
                );
            }
        }

        pub fn create_texture(size: &PhysicalSize<u32>) -> Texture {
            let texture = Texture::new(gl::TEXTURE_2D, 0, size.width, size.height);

            texture.bind();
            opengl! {
                // Default border is 0,0,0.
                texture.parameter(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
                texture.parameter(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
                texture.parameter(gl::TEXTURE_COMPARE_MODE, gl::COMPARE_REF_TO_TEXTURE as i32);
                texture.parameter(gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as i32);
            }
            opengl!(gl::TexImage2D(
                texture.gl_type,
                0,
                gl::DEPTH_COMPONENT.try_into().unwrap(),
                texture.width.try_into().unwrap(),
                texture.height.try_into().unwrap(),
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            ));

            return texture;
        }

        pub fn resize(&mut self, size: &PhysicalSize<u32>) {
            let texture = Self::create_texture(size);

            opengl! {
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
                gl::FramebufferTexture(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    texture.gl_id,
                    0,
                );
            }

            self.texture = texture;
        }
    }
}

use dir_light::DirLight;
use shadow::Shadow;

fn load_sphere_mesh() -> Mesh {
    let mut obj = Obj::new();
    obj.parse("./resources/sphere.obj");
    let opts = BuildOptions::default();
    if !obj.cmp_opts(&opts) {
        println!("At least vertices should be available in the model.");
        std::process::exit(1);
    }

    let (vbo, ebo) = obj.build(&opts);
    Mesh::new(&vbo, &ebo, |attrs| {
        attrs.add::<f32>(0, 3, gl::FLOAT);
        attrs.add::<f32>(1, 3, gl::FLOAT);
    })
}

fn create_plane_mesh() -> Mesh {
    let vbo: [f32; 24] = [
        // Position         Normal
        -0.5,  0.0,  0.5,    0.0, 1.0, 0.0,     // 0
         0.5,  0.0,  0.5,    0.0, 1.0, 0.0,     // 1
         0.5,  0.0, -0.5,    0.0, 1.0, 0.0,     // 2
        -0.5,  0.0, -0.5,    0.0, 1.0, 0.0,     // 3
    ];
    let ebo: [u32; 6] = [
        0, 2, 1,
        0, 3, 2,
    ];

    Mesh::new(&vbo, &ebo, |attrs| {
        attrs.add::<f32>(0, 3, gl::FLOAT);
        attrs.add::<f32>(1, 3, gl::FLOAT);
    })
}

fn create_scene_program() -> Result<Program, ()> {
    create_program(Some("./shaders/p7/scene"))
}

fn create_shadow_program() -> Result<Program, ()> {
    create_program(Some("./shaders/p7/shadow"))
}

uniforms!(SceneUniforms; u_model_t, u_view_t, u_proj_t, u_light_view_t, u_light_proj_t, u_texture, u_render_shadow);

struct Scene {
    size: PhysicalSize<u32>,
    light: DirLight,
    camera: FlyCamera,
    sphere: Mesh,
    plane: Mesh,
    program: Program,
    uniforms: SceneUniforms,
    shadow: Shadow,
    light_view: bool,
}

impl Scene {
    fn new(size: PhysicalSize<u32>) -> Self {
        let size_f = size_u_to_f32(&size);
        let camera = FlyCamera::new(|| {
            use camera_consts::*;
            Projection::perspective(FOV, size_f.width / size_f.height, NEAR, FAR)
        });
        let sphere = load_sphere_mesh();
        let mut plane = create_plane_mesh();
        plane.translate(0.0, -10.0, 0.0);
        plane.scale(100.0, 100.0, 100.0);
        let light = DirLight::new(size);
        let program = create_scene_program().expect("Scene program compiles");
        let uniforms = SceneUniforms::new(&program);
        let shadow = Shadow::new(&size);

        Self {
            size,
            light,
            camera,
            sphere,
            plane,
            program,
            uniforms,
            shadow,
            light_view: false,
        }
    }

    fn update_uniforms(&self) {
        self.shadow.update_uniforms(
            self.light.get_view(),
            self.light.get_proj(),
        );
        self.program.use_program();

        let (view_matrix, proj_matrix) = if self.light_view {
            (self.light.get_view(), self.light.get_proj())
        } else {
            (self.camera.get_view(), self.camera.get_proj())
        };

        opengl! {
            gl::UniformMatrix4fv(
                self.uniforms.u_view_t,
                1,
                gl::FALSE,
                view_matrix.as_ref() as *const _,
            );
            gl::UniformMatrix4fv(
                self.uniforms.u_proj_t,
                1,
                gl::FALSE,
                proj_matrix.as_ref() as *const _,
            );
            gl::UniformMatrix4fv(
                self.uniforms.u_light_view_t,
                1,
                gl::FALSE,
                self.light.get_view().as_ref() as *const _,
            );
            gl::UniformMatrix4fv(
                self.uniforms.u_light_proj_t,
                1,
                gl::FALSE,
                self.light.get_proj().as_ref() as *const _,
            );
            gl::Uniform1i(self.uniforms.u_texture, self.shadow.texture.slot.try_into().unwrap());
        }
    }

    fn draw(&self) {
        {
            // Bind shadow framebuffer, use shadow program, and for each model update u_model_t
            // uniform.
            self.shadow.bind(&self.size);
            self.shadow.clear();
            self.shadow.program.use_program();

            // Draw Sphere.
            opengl!(gl::UniformMatrix4fv(self.shadow.uniforms.u_model_t, 1, gl::FALSE, self.sphere.model_to_world.as_ref() as *const _));
            self.sphere.draw();

            // Draw Plane.
            opengl!(gl::UniformMatrix4fv(self.shadow.uniforms.u_model_t, 1, gl::FALSE, self.plane.model_to_world.as_ref() as *const _));
            self.plane.draw();

            // Restore default framebuffer.
            self.shadow.unbind(&self.size);
        }

        // Clear default framebuffer.
        opengl!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

        // Draw light cube.
        if !self.light_view {
            self.light.draw();
        }

        self.program.use_program();

        // Draw Plane.
        opengl! {
            gl::UniformMatrix4fv(self.uniforms.u_model_t, 1, gl::FALSE, self.plane.model_to_world.as_ref() as *const _);
            gl::Uniform1ui(self.uniforms.u_render_shadow, 1_u32);
        };
        self.plane.draw();

        // Draw Sphere.
        opengl! {
            gl::UniformMatrix4fv(self.uniforms.u_model_t, 1, gl::FALSE, self.sphere.model_to_world.as_ref() as *const _);
            gl::Uniform1ui(self.uniforms.u_render_shadow, 0_u32);
        }
        self.sphere.draw();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        let size_f = size_u_to_f32(&self.size);
        self.shadow.resize(&self.size);
        self.camera.projection.resize(
            size_f.width / size_f.height
        );
        self.camera.update();
        self.update_uniforms();
    }
}

fn main() {
    let glin = Glindow::new();

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    }

    let size = glin.window.inner_size();
    let mut scene = Scene::new(size);
    let mut input_state = InputState::new(size);

    scene.light.goto(10.0, 50.0, 10.0);
    scene.light
        .update_uniforms(
            scene.camera.get_view(),
            scene.camera.get_proj(),
        );
    scene.camera.goto(0.0, 0.0, 50.0).update();
    scene.update_uniforms();

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
                scene.draw();

                surface.swap_buffers(&context).expect("I want to swap!");
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                    let Some(keycode) = input.virtual_keycode else {
                        return;
                    };

                    if input.state == ElementState::Released {
                        return;
                    }

                    match keycode {
                        VirtualKeyCode::L => {
                            scene.light.goto(10.0, 50.0, 10.0);
                            scene.update_uniforms();
                            window.request_redraw();
                        }
                        VirtualKeyCode::S => {
                            scene.light_view = !scene.light_view;
                            scene.update_uniforms();
                            window.request_redraw();
                        }
                        VirtualKeyCode::R => {
                            match create_scene_program() {
                                Ok(program) => {
                                    println!("Reloading shaders.");
                                    scene.program = program;
                                    scene.uniforms.update_program(&scene.program);
                                    scene.update_uniforms();
                                    window.request_redraw();
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    };
                }
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
                    scene.resize(size);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    input_state.mouse_click(&state, &button);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if input_state.mouse.is_none() {
                        return;
                    }

                    let Some((delta_x, delta_y)) = input_state.cursor_move(&position) else {
                        return;
                    };

                    let (x, y, z) = match input_state.mouse.unwrap() {
                        MouseButton::Left => {
                            let x = delta_x;
                            let y = delta_y;
                            let z = 0.0;

                            (x, y, z)
                        }
                        MouseButton::Right => {
                            let x = delta_x;
                            let y = 0.0;
                            let z = delta_y;

                            (x, y, z)
                        }
                        _ => unreachable!(),
                    };

                    scene.light.translate(x, y, z);
                    scene.light.update_uniforms(scene.camera.get_view(), scene.camera.get_proj());
                    scene.update_uniforms();
                    window.request_redraw();
                }
                _ => (),
            }
            _ => (),
        }
    });
}

