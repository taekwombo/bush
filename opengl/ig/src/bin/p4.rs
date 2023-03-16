//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=4

use gluty::*;
use ig::*;
use std::path::PathBuf;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::*;

struct Ctrl {
    state: InputState,
    mesh: Mesh,
    light: Light,
    material: Material,
    textures: [Texture; 3],
    u_model: i32,
    u_view: i32,
    u_proj: i32,
    u_light_pos: i32,
    u_textures: i32,
    display_textures: u32,
}

impl Ctrl {
    fn bind_textures(&self) {
        for tex in &self.textures {
            tex.bind();
        }
    }

    fn new(size: PhysicalSize<u32>) -> Self {
        let mut obj = Obj::new();
        let path = get_model_path();
        obj.parse(&path);

        let mut path = PathBuf::from(path);

        let textures = [
            {
                path.set_file_name(&obj.material.ambient_texture);
                Texture::create(&path, gl::TEXTURE_2D, 0).expect("Ambient texture.")
            },
            {
                path.set_file_name(&obj.material.diffuse_texture);
                Texture::create(&path, gl::TEXTURE_2D, 1).expect("Diffuse texture.")
            },
            {
                path.set_file_name(&obj.material.specular_texture);
                Texture::create(&path, gl::TEXTURE_2D, 2).expect("Specular texture.")
            },
        ];

        let (vbo, ebo) = obj.build(true);

        Self {
            u_model: -1,
            u_view: -1,
            u_proj: -1,
            u_light_pos: -1,
            u_textures: -1,
            display_textures: 1,
            textures,
            light: Light::new(),
            state: InputState::new(size),
            material: obj.material,
            mesh: Mesh::new(&vbo, &ebo, |attrs| {
                attrs
                    .add::<f32>(0, 3, gl::FLOAT)
                    .add::<f32>(1, 3, gl::FLOAT)
                    .add::<f32>(2, 3, gl::FLOAT);
            }),
        }
    }

    fn handle_key_press(&mut self, state: &ElementState, keycode: &VirtualKeyCode) -> bool {
        if *state == ElementState::Pressed && matches!(keycode, VirtualKeyCode::Space) {
            self.display_textures = if self.display_textures == 1 { 0 } else { 1 };
            return true;
        }

        false
    }

    fn handle_cursor_move(&mut self, position: &PhysicalPosition<f64>) -> bool {
        if self.state.mouse.is_none() || !(self.state.ctrl || self.state.alt) {
            return false;
        }

        let Some((delta_x, delta_y)) = self.state.cursor_move(position) else {
            return false;
        };

        let speed = 4.0_f32;

        match self.state.mouse.unwrap() {
            MouseButton::Right => {
                if self.state.ctrl {
                    self.light.translate(0.0, 0.0, delta_y * speed);
                } else {
                    self.mesh.rotate_y(delta_x);
                    self.mesh.rotate_x(delta_y);
                }
            }
            MouseButton::Left => {
                if self.state.ctrl {
                    self.light.translate(delta_x * speed, delta_y * speed, 0.0);
                } else {
                    self.mesh.translate(delta_x * speed, delta_y * speed, 0.0);
                }
            }
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
        opengl! {
            gl::UniformMatrix4fv(self.u_model,  1, gl::FALSE, self.mesh.model_to_world.as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_view,   1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_proj,   1, gl::FALSE, camera.get_proj().as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_light_pos, 1, gl::FALSE, self.light.position.as_ref() as *const _);
            gl::Uniform1ui(self.u_textures, self.display_textures);
        }

        self.light
            .upload_uniforms(camera.get_view(), camera.get_proj());
    }

    fn program_changed(&mut self, program: &Program) {
        self.u_model = program.get_uniform("u_model_t\0");
        self.u_view = program.get_uniform("u_view_t\0");
        self.u_proj = program.get_uniform("u_proj_t\0");
        self.u_light_pos = program.get_uniform("u_light_pos\0");
        self.u_textures = program.get_uniform("u_display_textures\0");

        let ka = program.get_uniform("u_ambient_color\0");
        let kd = program.get_uniform("u_diffuse_color\0");
        let ks = program.get_uniform("u_specular_color\0");
        let ns = program.get_uniform("u_specular_component\0");
        let tex_a = program.get_uniform("u_tex_ambient\0");
        let tex_d = program.get_uniform("u_tex_diffuse\0");
        let tex_s = program.get_uniform("u_tex_specular\0");

        program.use_program();

        opengl!(gl::Uniform1f(ns, self.material.specular_component));
        opengl!(gl::Uniform3fv(ka, 1, self.material.ambient_color.as_ptr()));
        opengl!(gl::Uniform3fv(kd, 1, self.material.diffuse_color.as_ptr()));
        opengl!(gl::Uniform3fv(ks, 1, self.material.specular_color.as_ptr()));
        opengl!(gl::Uniform1i(tex_a, 0));
        opengl!(gl::Uniform1i(tex_d, 1));
        opengl!(gl::Uniform1i(tex_s, 2));
    }
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let mut project = Project::new(Ctrl::new(size), size, || {
        create_program(Some("./shaders/p4"))
    });

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.ctrl().light.translate(-20.0, 20.0, 30.0);
    project.upload_uniforms();

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

                    project.prog.use_program();
                    project.ctrl().bind_textures();
                    project.ctrl().mesh.draw();
                    project.ctrl().light.draw();

                    surface.swap_buffers(&context).expect("I want to swap!");
                }
            }
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
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    let Some(keycode) = input.virtual_keycode else {
                        return;
                    };

                    let controller = project.ctrl();
                    if controller.handle_key_press(&input.state, &keycode)
                        || project.handle_key_code(&input)
                    {
                        project.upload_uniforms();
                        window.request_redraw();
                    }
                }
                WindowEvent::MouseInput {
                    state: mouse_state,
                    button,
                    ..
                } => {
                    project.ctrl().state.mouse_click(&mouse_state, &button);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if project.ctrl().handle_cursor_move(&position)
                        || project.handle_cursor_move(&position)
                    {
                        project.upload_uniforms();
                        window.request_redraw();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}
