//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=6

use gluty::asset::Asset;
use gluty::*;
use ig::*;
use winit::event::*;

#[cfg(feature = "swap_textures")]
struct Textures {
    prev: Texture,
    next: Texture,
}

#[cfg(feature = "swap_textures")]
impl Textures {
    fn swap(&mut self, old: &mut Texture) {
        self.next.bind();
        std::mem::swap(&mut self.next, old);
        std::mem::swap(&mut self.next, &mut self.prev);
    }
}

fn textures<T: AsRef<[u8]>>(sources: &[Asset<T>], slot: u32) -> Result<Texture, ()> {
    let images = sources
        .iter()
        .map(|it| it.try_to_img().unwrap())
        .collect::<Vec<_>>();
    let cubemap_texture_types = &[
        gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ];

    let ((width, height), ..) = Texture::get_image_info(images.first().unwrap());
    let texture = Texture::new(gl::TEXTURE_CUBE_MAP, slot, width, height);

    texture.bind();
    opengl!(texture.parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32));

    for (idx, img) in images.iter().enumerate() {
        let (_, dataf, datat) = Texture::get_image_info(img);
        texture.data_with_type(
            cubemap_texture_types[idx],
            gl::RGBA,
            dataf,
            datat,
            img.as_bytes(),
        );
    }

    opengl! {
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
    }

    texture.unbind();

    Ok(texture)
}

fn load_cube_mesh() -> Mesh {
    let mut obj = Obj::new();
    // This object can contain only vertex positions - they're going to be used as texture
    // coordinates.
    obj.parse_obj(&assets!("./cube.obj"));

    let opts = BuildOptions::vertices_only();
    if !obj.cmp_opts(&opts) {
        println!("At least vertices should be available in the model.");
        std::process::exit(1);
    }

    let (vbo, ebo) = obj.build(&opts);

    Mesh::new(&vbo, &ebo, |attrs| {
        attrs.add::<f32>(0, 3, gl::FLOAT);
    })
}

fn load_sphere_mesh() -> Mesh {
    let mut obj = Obj::new();
    obj.parse_obj(&assets!("./sphere.obj"));
    let opts = BuildOptions::default();
    if !obj.cmp_opts(&opts) {
        println!("Model with v, vn and vt attributes required.");
        std::process::exit(1);
    }

    let (vbo, ebo) = obj.build(&opts);
    Mesh::new(&vbo, &ebo, |attrs| {
        attrs.add::<f32>(0, 3, gl::FLOAT);
        attrs.add::<f32>(1, 3, gl::FLOAT);
    })
}

uniforms!(EnvMapUniforms; u_model_t, u_view_t, u_proj_t, u_texture);

struct Cube {
    model: Mesh,
    texture: Texture,
    program: Program,
    uniforms: EnvMapUniforms,
}

impl Cube {
    fn new(texture: Texture) -> Self {
        let (vert, frag) = assets!("./shaders/p6/cube.vert", "./shaders/p6/cube.frag",);
        let program = Program::default()
            .shader(frag.get(), gl::FRAGMENT_SHADER)
            .shader(vert.get(), gl::VERTEX_SHADER)
            .link();

        let model = load_cube_mesh();

        Self {
            uniforms: EnvMapUniforms::new(&program),
            texture,
            program,
            model,
        }
    }

    fn update_uniforms(&self, camera: &FlyCamera) {
        self.program.use_program();

        opengl! {
            gl::UniformMatrix4fv(self.uniforms.u_view_t, 1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.uniforms.u_proj_t, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
            gl::UniformMatrix4fv(self.uniforms.u_model_t, 1, gl::FALSE, self.model.model_to_world.as_ref() as *const _);
            gl::Uniform1i(self.uniforms.u_texture, self.texture.slot as i32);
        }
    }
}

struct Sphere {
    model: Mesh,
    program: Program,
    uniforms: EnvMapUniforms,
}

impl Sphere {
    fn load_program() -> Result<Program, ()> {
        let (vert, frag) = assets!("./shaders/p6/sphere.vert", "./shaders/p6/sphere.frag",);
        let program = Program::default()
            .shader(frag.get(), gl::FRAGMENT_SHADER)
            .shader(vert.get(), gl::VERTEX_SHADER)
            .link();

        match Program::check_errors(&program) {
            Ok(()) => Ok(program),
            Err(_) => Err(()),
        }
    }

    fn new() -> Self {
        let program = Self::load_program().expect("Sphere program to compile");

        Self {
            model: load_sphere_mesh(),
            uniforms: EnvMapUniforms::new(&program),
            program,
        }
    }

    fn update_uniforms(&self, camera: &FlyCamera, texture: &Texture) {
        self.program.use_program();

        opengl! {
            gl::UniformMatrix4fv(self.uniforms.u_view_t, 1, gl::FALSE, camera.get_view().as_ref() as *const _);
            gl::UniformMatrix4fv(self.uniforms.u_proj_t, 1, gl::FALSE, camera.get_proj().as_ref() as *const _);
            gl::UniformMatrix4fv(self.uniforms.u_model_t, 1, gl::FALSE, self.model.model_to_world.as_ref() as *const _);
            gl::Uniform1i(self.uniforms.u_texture, texture.slot as i32);
        }
    }
}

fn main() {
    #[cfg(feature = "swap_textures")]
    let cubemap_mountain = assets!([
        "./cubemap/negx.jpg",
        "./cubemap/negy.jpg",
        "./cubemap/negz.jpg",
        "./cubemap/posx.jpg",
        "./cubemap/posy.jpg",
        "./cubemap/posz.jpg",
    ]);

    #[cfg(feature = "swap_textures")]
    let cubemap_sea = assets!([
        "./cubemap/left.jpg",
        "./cubemap/bottom.jpg",
        "./cubemap/back.jpg",
        "./cubemap/right.jpg",
        "./cubemap/top.jpg",
        "./cubemap/front.jpg",
    ]);

    let cubemap_city = assets!([
        "./cubemap/cubemap_negx.png",
        "./cubemap/cubemap_negy.png",
        "./cubemap/cubemap_negz.png",
        "./cubemap/cubemap_posx.png",
        "./cubemap/cubemap_posy.png",
        "./cubemap/cubemap_posz.png",
    ]);

    let glin = Glindow::new();
    let size = glin.window.inner_size();
    println!("Loading textures");
    let cube_texture = textures(&cubemap_city, 0).expect("Textures loaded succesfully");
    let mut input_state = InputState::new(size);

    let mut cube = Cube::new(cube_texture);
    let mut camera = FlyCamera::new(|| {
        use camera_consts::*;
        let size = size_u_to_f32(&size);
        Projection::perspective(FOV, size.width / size.height, NEAR, 10000.0)
    });
    let mut sphere = Sphere::new();
    #[cfg(feature = "swap_textures")]
    let mut txt_store = Textures {
        prev: textures(CUBEMAP_MOUNTAIN, 0).expect("ok"),
        next: textures(CUBEMAP_CITY, 0).expect("ok"),
    };

    cube.texture.bind();
    cube.model.scale(200.0, 200.0, 200.0);
    cube.update_uniforms(&camera);
    sphere.model.translate(0.0, 0.0, -80.0);
    sphere.update_uniforms(&camera, &cube.texture);

    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    #[cfg(debug_assertions)]
    {
        cube.model.bind_vao();
        Program::check_errors(&cube.program).expect("cube program is invalid");
        sphere.model.bind_vao();
        Program::check_errors(&sphere.program).expect("Sphere program is valid");
        opengl!(gl::BindVertexArray(0));
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

                cube.program.use_program();
                cube.model.draw();
                sphere.program.use_program();
                sphere.model.draw();

                surface.swap_buffers(&context).expect("I want to swap!");
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::MouseInput { state, button, .. } => {
                    input_state.mouse_click(&state, &button);
                }
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    if input.state == ElementState::Released {
                        return;
                    }

                    let Some(kc) = input.virtual_keycode else {
                        return;
                    };

                    match kc {
                        #[cfg(feature = "swap_textures")]
                        VirtualKeyCode::T => {
                            println!("Changing texture");
                            txt_store.swap(&mut cube.texture);
                            window.request_redraw();
                        }
                        VirtualKeyCode::R => {
                            println!("Reloading shaders.");
                            if let Ok(program) = Sphere::load_program() {
                                sphere.program = program;
                                sphere.update_uniforms(&camera, &cube.texture);
                                window.request_redraw();
                            }
                        }
                        _ => (),
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if input_state.mouse.is_none() {
                        return;
                    }

                    let Some((delta_x, delta_y)) = input_state.cursor_move(&position) else {
                        return;
                    };

                    match input_state.mouse.unwrap() {
                        MouseButton::Right => {
                            camera.accelerate_z(delta_y).accelerate_x(delta_x).update();
                        }
                        MouseButton::Left => {
                            camera.rotate(delta_y, delta_x).update();
                        }
                        _ => return,
                    };

                    cube.update_uniforms(&camera);
                    sphere.update_uniforms(&camera, &cube.texture);
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        }
    });
}
