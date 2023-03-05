//! Displaying .obj model with lighting and camera.

use gluty::{Glindow, Program, Attributes, opengl};

fn main() {
    // https://users.cs.utah.edu/~natevm/newell_teaset/
    let (vertices, indices) = model::load("./examples/resources/teapot_newell.obj");
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

    let mut teapot = mesh::Mesh::new(
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

mod mesh {
    use gluty::{Attributes, Buffer, opengl};
    use glam::{Mat4, Vec3};

    pub struct Mesh {
        vao: u32,
        #[allow(dead_code)]
        vbo: Buffer,
        #[allow(dead_code)]
        ebo: Buffer,
        indices: i32,
        /// VBO attributes.
        #[allow(dead_code)]
        attrs: Attributes,
        /// Defines position of the model in the world coordinate system.
        pub model_to_world: Mat4,
    }

    impl Mesh {
        pub fn new(vbo_data: &[f32], ebo_data: &[u32], attrs: Attributes) -> Self {
            let mut vao: u32 = 0;
            let vbo: Buffer;
            let ebo: Buffer;

            opengl! {
                // Create and bind Vertex Array.
                gl::GenVertexArrays(1, &mut vao);
                gl::BindVertexArray(vao);
            }

            // Bind vertex buffer.
            vbo = Buffer::new(gl::ARRAY_BUFFER);
            vbo.bind().data(vbo_data);
            // Bind element buffer.
            ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            ebo.bind().data(ebo_data);

            // Enable vertex attributes.
            attrs.bind();

            opengl! {
                // Cleanup starting from Vertex Array.
                gl::BindVertexArray(0);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }

            Self {
                vao,
                vbo, 
                ebo,
                attrs,
                indices: ebo_data.len() as i32,
                model_to_world: Mat4::IDENTITY,
            }
        }

        pub fn scale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
            self.model_to_world *= Mat4::from_scale(Vec3::new(x, y, z));
            self
        }

        pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
            self.model_to_world *= Mat4::from_translation(Vec3::new(x, y, z));
            self
        }

        pub fn rotate_y(&mut self, y: f32) -> &mut Self {
            self.model_to_world *= Mat4::from_rotation_y(
                y.to_radians(),
            );
            self
        }

        pub fn draw(&self) -> &Self {
            opengl! {
                gl::BindVertexArray(self.vao);
                gl::DrawElements(gl::TRIANGLES, self.indices, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0);
            }
            self
        }
    }

    impl Drop for Mesh {
        fn drop(&mut self) {
            opengl! {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
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

mod model {
    /// Expects: "<f32> <f32> <f32>\n" string slice.
    /// Parses floats and appends them to the vec parameter.
    fn parse_vector(line: &str) -> [f32; 3] {
        let mut split = line.split(' ').skip_while(|v| v.len() == 0);
        [
            split
                .next().expect("Component exists.")
                .parse::<f32>().expect("Component value is f32."),
            split
                .next().expect("Component exists.")
                .parse::<f32>().expect("Component value is f32."),
            split
                .next().expect("Component exists.")
                .parse::<f32>().expect("Component value is f32."),
        ]
    }

    fn parse_face(line: &str) -> ([(u32, u32); 4], usize) {
        // Let's have a space for at most 4 vertices in one face.
        let mut loaded: [(u32, u32); 4] = [(0, 0); 4];
        let mut index = 0;

        for value in line.split(' ') {
            if value.len() == 0 {
                continue;
            }

            if index > 4 {
                println!("{}", line);
                unimplemented!("Faces with more than 4 vertices are not supported.");
            }

            let mut split = value.split('/');

            let vertex = split.next().unwrap()
                .parse::<i32>().expect("Vertex index to be u32.");
            let normal = split.last().unwrap()
                .parse::<i32>().expect("Vertex normal index to be u32.");

            debug_assert!(vertex >= 0);
            debug_assert!(normal >= 0);

            // Indices are 1 based.
            let vertex = vertex as u32 - 1;
            let normal = normal as u32 - 1;

            loaded[index] = (vertex, normal);
            index += 1;
        }

        debug_assert!(index == 4 || index == 3);
        (loaded, index)
    }

    fn append_vertex(
        loaded_vertices: &[[f32; 3]],
        loaded_vertex_normals: &[[f32; 3]],
        vbo_data: &mut Vec<f32>,
        ebo_data: &mut Vec<u32>,
        vertex_map: &mut std::collections::HashMap<(u32, u32), usize>,
        pair: &(u32, u32),
    ) {
        if vertex_map.contains_key(pair) {
            let index = vertex_map.get(pair).unwrap();
            ebo_data.push(u32::try_from(*index).expect("Index must fit into u32."));
        } else {
            // 6 floats are stored per vertex.
            let index = vbo_data.len() / 6;
            vertex_map.insert(*pair, index);
            ebo_data.push(u32::try_from(index).expect("Index must fit into u32."));
            vbo_data.extend_from_slice(
                &loaded_vertices[pair.0 as usize]
            );
            vbo_data.extend_from_slice(
                &loaded_vertex_normals[pair.1 as usize]
            );
        }
    }

    /// Loads some parts of .obj file.
    /// Just enough to render teapot.
    pub fn load(path: &'static str) -> (Vec<f32>, Vec<u32>) {
        use std::fs::read;
        use std::collections::HashMap;

        let mut ebo_data: Vec<u32> = Vec::new();
        let mut vbo_data: Vec<f32> = Vec::new();

        let mut loaded_vertices: Vec<[f32; 3]> = Vec::new();
        let mut loaded_vertex_normals: Vec<[f32; 3]> = Vec::new();
        let mut vertex_map: HashMap<(u32, u32), usize> = HashMap::new();

        let file = read(path).expect("Model file must exist.");
        let file = String::from_utf8_lossy(&file);

        for line in file.lines() {
            if line.starts_with("v ") {
                loaded_vertices.push(parse_vector(&line[2..]));
            } else if line.starts_with("vn ") {
                loaded_vertex_normals.push(parse_vector(&line[3..]));
            } else if line.starts_with("f ") {
                let (vn, len) = parse_face(&line[2..]);
                for pair in &vn[0..3] {
                    append_vertex(
                        &mut loaded_vertices,
                        &mut loaded_vertex_normals,
                        &mut vbo_data,
                        &mut ebo_data,
                        &mut vertex_map,
                        pair
                    );
                }

                // If the face was a rectangle, add second part of it.
                if len == 4 {
                    for pair in &[vn[2], vn[3], vn[0]] {
                        append_vertex(
                            &mut loaded_vertices,
                            &mut loaded_vertex_normals,
                            &mut vbo_data,
                            &mut ebo_data,
                            &mut vertex_map,
                            pair,
                        );
                    }
                }
            }
        }

        (vbo_data, ebo_data)
    }
}
