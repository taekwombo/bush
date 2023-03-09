//! Displaying .obj model with lighting and camera.

use gluty::{Mesh, Glindow, Program, Attributes, opengl, obj};

fn main() {
    // https://users.cs.utah.edu/~natevm/newell_teaset/
    let (vertices, indices) = obj::load("./examples/resources/teapot_newell.obj");
    let glin = Glindow::new();

    let mut program = Program::create();
    program
        .attach_shader_source("./examples/shaders/teapot.vert", gl::VERTEX_SHADER)
        .and_then(|p| p.attach_shader_source("./examples/shaders/teapot.frag", gl::FRAGMENT_SHADER))
        .and_then(|p| p.link())
        .expect("Program created.")
        .use_program();

    let mut attrs = Attributes::new();
    attrs
        // Position attribute.
        .add::<f32>(0, 3, gl::FLOAT)
        // Vertex normal attribute.
        .add::<f32>(1, 3, gl::FLOAT);

    let mut teapot = Mesh::new(
        &vertices,
        &indices,
        attrs,
    );
    teapot
        .translate(0.0, -0.5, 0.0)
        .scale(0.3, 0.3, 0.3);

    let mut camera = {
        let size = glin.window.inner_size();
        camera::Camera::new(60.0, size.width, size.height)
    };
    camera.translate(0.0, 0.0, 5.0);

    let u_proj = program.get_uniform("u_proj\0");
    let u_model = program.get_uniform("u_model\0");
    let u_light = program.get_uniform("u_light\0");

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Update projection matrix.
        gl::UniformMatrix4fv(u_proj, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
        gl::UniformMatrix4fv(u_model, 1, gl::FALSE, teapot.model_to_world.as_ref() as *const _);
        gl::Uniform4f(u_light, 0.0, 3.0, 2.0, 1.0);
    }

    #[allow(unused_variables)]
    let Glindow { window, event_loop, display, surface, context } = glin;
    let mut rotating = false;
    let mut prev_x: f64 = -1.0;
    let mut prev_y: f64 = -1.0;

    event_loop.run(move |event, _, control_flow| {
        use winit::event::{Event, WindowEvent};
        use glutin::prelude::*;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ReceivedCharacter(ch) => match ch {
                    'w' | 's' => {
                        let mov = if ch == 'w' { -0.6 } else { 0.6 };
                        camera.translate(0.0, 0.0, mov);
                        opengl! {
                            gl::UniformMatrix4fv(
                                u_proj, 1, gl::FALSE,
                                camera.get_proj().as_ref() as *const _
                            );
                        }
                    },
                    'a' | 'd' => {
                        let mov = if ch == 'a' { -0.6 } else { 0.6 };
                        camera.translate(mov, 0.0, 0.0);
                        opengl! {
                            gl::UniformMatrix4fv(
                                u_proj, 1, gl::FALSE,
                                camera.get_proj().as_ref() as *const _
                            );
                        }
                    },
                    _ => (),
                },
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        surface.resize(
                            &context,
                            size.width.try_into().unwrap(),
                            size.height.try_into().unwrap(),
                        );
                        camera.resized((size.width as i32 as f32) / (size.height as i32 as f32));
                        opengl! {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                            gl::UniformMatrix4fv(
                                u_proj, 1, gl::FALSE,
                                camera.get_proj().as_ref() as *const _
                            );
                        }
                        window.request_redraw();
                    }
                },
                WindowEvent::CloseRequested =>  {
                    control_flow.set_exit();
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if rotating && prev_x >= 0.0 {
                        let dx = position.x - prev_x;
                        let dy = position.y - prev_y;
                        let size = window.inner_size();

                        camera.rotate(
                            (dy / size.height as f64) as f32,
                            (dx / size.width as f64) as f32,
                            0.0,
                        );
                        opengl!(
                            gl::UniformMatrix4fv(
                                u_proj, 1, gl::FALSE,
                                camera.get_proj().as_ref() as *const _
                            );
                        );
                    }
                    if rotating {
                        prev_x = position.x;
                        prev_y = position.y;
                    }
                },
                WindowEvent::MouseInput { button, state, .. } => {
                    use winit::event::{ElementState, MouseButton};

                    if button == MouseButton::Left {
                        rotating = state == ElementState::Pressed;

                        if !rotating {
                            prev_x = -1.0;
                            prev_y = -1.0;
                        }
                    } 
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                teapot.rotate_y(0.5);
                opengl! { 
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::UniformMatrix4fv(u_model, 1, gl::FALSE, teapot.model_to_world.as_ref() as *const _);
                };
                teapot.draw();
                surface.swap_buffers(&context).expect("I want to swap!");
                window.request_redraw();
            },
            _ => (),
        }
    });
}

mod camera {
    use glam::{Mat4, Vec3, Vec4, Quat};

    pub struct Camera {
        /// Defines position of the camera in the world coordinate space.
        pub camera_to_world: Mat4,
        projection: Mat4,
        fov: f32,
        near: f32,
        far: f32,
    }

    impl Camera {
        pub fn new(fov: f32, width: u32, height: u32) -> Self {
            Self {
                camera_to_world: Mat4::IDENTITY,
                fov,
                near: 0.1,
                far: 100.0,
                projection: Mat4::perspective_rh_gl(
                    fov.to_radians(),
                    (width as i32 as f32) / (height as i32 as f32),
                    0.1,
                    100.0,
                ),
            }
        }

        pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
            self.camera_to_world *= Mat4::from_translation(Vec3::new(x, y, z));
            self
        }

        pub fn rotate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
            self.camera_to_world *= Mat4::from_quat(Quat::from_vec4(
                Vec4::new(
                    x,
                    y,
                    z,
                    1.0,
                ),
            ));
            self
        }

        pub fn resized(&mut self, ratio: f32) -> &mut Self {
            self.projection = Mat4::perspective_rh_gl(
                self.fov.to_radians(),
                ratio,
                self.near,
                self.far,
            );
            self
        }

        pub fn get_proj(&self) -> Mat4 {
            // Projection * World to Camera (inverse of the camera to world).
            self.projection * self.camera_to_world.inverse()
        }
    }
}

