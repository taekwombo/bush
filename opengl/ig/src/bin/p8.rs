//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=8

use gluty::{
    winit::dpi::PhysicalSize,
    opengl, uniforms, Program,
    gl, Mesh, FlyCamera, Projection, Glindow, Texture
};
use ig::*;

const DEFAULT_TESS_LEVEL: f32 = 16.0;

uniforms!(Uniforms; u_tess_level, /* u_texture_n, */ u_texture_d, u_model_t, u_view_t, u_proj_t/*, u_light_world_t*/);

struct Tesselation {
    camera: FlyCamera,
    light: Light,
    model: Mesh,
    normal_tex: Texture,
    displacement_tex: Texture,
    tri_program: Program,
    uniforms: Uniforms,
    tess_level: f32,
} 

impl Tesselation {
    fn new(size: &PhysicalSize<u32>) -> Self {
        let tri_program = Self::create_triangulation_program().unwrap();
        let uniforms = Uniforms::new(&tri_program);
        let (normal_tex, displacement_tex) = Self::load_textures();
        let model = Self::load_mesh();
        let camera = FlyCamera::new(|| {
            use camera_consts::*;
            let size_f = size_u_to_f32(size);
            Projection::perspective(FOV, size_f.width / size_f.height, NEAR, FAR)
        });

        Self {
            tri_program,
            uniforms,
            normal_tex,
            displacement_tex,
            model,
            camera,
            light: Light::new(),
            tess_level: DEFAULT_TESS_LEVEL,
        }
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

    fn load_textures() -> (Texture, Texture) {
        let normal = Texture::load_file(
            "./resources/teapot_normal.png",
            true,
        ).unwrap();
        let displacement = Texture::load_file(
            "./resources/teapot_disp.png",
            true,
        ).unwrap();

        (
            {
                let (width, height) = normal.dimensions();
                let tex = Texture::new(gl::TEXTURE_2D, 0, width, height);
                tex.bind();
                tex.data(normal.as_raw(), None);
                tex.unbind();
                tex
            },
            {
                let (width, height) = displacement.dimensions();
                let tex = Texture::new(gl::TEXTURE_2D, 1, width, height);
                tex.bind();
                tex.data(displacement.as_raw(), None);
                tex.unbind();
                tex
            }
        )
    }

    fn create_triangulation_program() -> Result<Program, ()> {
        let mut program = Program::create();

        program.attach_shader_source("./shaders/p8/tesselation.vert", gl::VERTEX_SHADER)?;
        program.attach_shader_source("./shaders/p8/tesselation.tesc", gl::TESS_CONTROL_SHADER)?;
        program.attach_shader_source("./shaders/p8/tesselation.tese", gl::TESS_EVALUATION_SHADER)?;
        program.attach_shader_source("./shaders/p8/triangulation.geom", gl::GEOMETRY_SHADER)?;
        program.attach_shader_source("./shaders/p8/triangulation.frag", gl::FRAGMENT_SHADER)?;
        program.link()?;

        Ok(program)
    }

    fn create_tess_program() -> Result<Program, ()> {
        let mut program = Program::create();

        program.attach_shader_source("./shaders/p8/tesselation.vert", gl::VERTEX_SHADER)?;
        program.attach_shader_source("./shaders/p8/tesselation.tesc", gl::TESS_CONTROL_SHADER)?;
        program.attach_shader_source("./shaders/p8/tesselation.tese", gl::TESS_EVALUATION_SHADER)?;
        program.attach_shader_source("./shaders/p8/tesselation.frag", gl::FRAGMENT_SHADER)?;
        program.link()?;

        Ok(program)
    }

    fn update(&self) {
        let view_mat = self.camera.get_view();
        let proj_mat = self.camera.get_proj();

        self.light.update_uniforms(view_mat, proj_mat);

        self.tri_program.use_program();
        opengl! {
            gl::Uniform1i(self.uniforms.u_texture_d, self.displacement_tex.slot as i32);
            gl::Uniform1f(self.uniforms.u_tess_level, self.tess_level);
            gl::UniformMatrix4fv(
                self.uniforms.u_model_t,
                1,
                gl::FALSE,
                self.model.model_to_world.as_ref() as *const _,
            );
            gl::UniformMatrix4fv(
                self.uniforms.u_view_t,
                1,
                gl::FALSE,
                view_mat.as_ref() as *const _,
            );
            gl::UniformMatrix4fv(
                self.uniforms.u_proj_t,
                1,
                gl::FALSE,
                proj_mat.as_ref() as *const _,
            );
            // gl::UniformMatrix4fv(
            //     self.uniforms.u_light_world_t,
            //     1,
            //     gl::FALSE,
            //     self.light.position.as_ref() as *const _,
            // );
        }
    }
}

fn main() {
    let glin = Glindow::new();
    let size = glin.window.inner_size();
    let mut tess = Tesselation::new(&size);
    let mut input_state = InputState::new(size);

    tess.camera.goto(0.0, 20.0, 40.0).rotate(-20.0, 0.0).update();
    tess.model.scale(20.0, 20.0, 20.0);
    tess.light.translate(10.0, 10.0, -20.0);
    tess.update();

    tess.normal_tex.bind();
    tess.displacement_tex.bind();
    opengl! {
        // gl::Uniform1i(tess.uniforms.u_texture_n, tess.normal_tex.slot as i32);
        gl::Uniform1i(tess.uniforms.u_texture_d, tess.displacement_tex.slot as i32);
        gl::ClearColor(0.1, 0.1, 0.15, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::PatchParameteri(gl::PATCH_VERTICES, 3);
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

                // tess.light.draw();
                tess.tri_program.use_program();
                tess.model.draw();
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
                    let size = size_u_to_f32(&size);
                    tess.camera
                        .projection
                        .resize(size.width / size.height);
                    tess.camera.update();
                    tess.update();
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

                    if input_state.shift {
                        tess.light.translate(x, y, z);
                    } else if input_state.alt {
                        tess.model.rotate_x(y).rotate_y(x).rotate_z(z);
                    } else {
                        if z != 0.0 {
                            tess.camera
                                .accelerate_z(z)
                                .accelerate_x(x)
                                .update();
                        } else {
                            tess.camera
                                .rotate(y, x)
                                .update();
                        }
                    }

                    tess.update();
                    window.request_redraw();
                }
                WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                    let Some(keycode) = input.virtual_keycode else {
                        return;
                    };

                    input_state.modifiers(&input.state, &keycode);

                    if input.state == ElementState::Released {
                        return;
                    }

                    match keycode {
                        VirtualKeyCode::Equals | VirtualKeyCode::Minus | VirtualKeyCode::Key0 => {
                            if keycode == VirtualKeyCode::Key0 {
                                tess.tess_level = 2.0;
                            } else {
                                let mut change = if matches!(keycode, VirtualKeyCode::Equals) { 1.0 } else { -1.0 };
                                tess.tess_level = 2.0_f32.max(tess.tess_level + change);
                            }
                            tess.update();
                            window.request_redraw();
                        }
                        VirtualKeyCode::R => {
                            match Tesselation::create_triangulation_program() {
                                Ok(program) => {
                                    println!("Reloading shaders.");
                                    tess.tri_program = program;
                                    tess.uniforms = Uniforms::new(&tess.tri_program);
                                    tess.update();
                                    window.request_redraw();
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    };
                }
                _ => (),
            },
            _ => (),
        }
    });
}
