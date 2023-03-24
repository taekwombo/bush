//! Displaying texture on rectangle with aspect projection.
//!
//! The image displayed must be present at gluty/examples/resources/opossum.jpg.

use glam::f32::{Mat4, Vec3};
use gluty::{opengl, Attributes, Buffer, Glindow, Program, Texture};

fn to_f(val: u32) -> f32 {
    //! Good enough u32 -> f32 conversion for this example.
    let im: i32 = val.try_into().expect("value fits in i32");
    im as f32
}

struct State {
    /// Current zoom of the image.
    /// Ranges from 0.5 to 4.
    zoom: f32,
    /// X axis translation.
    /// Ranges from -1 to 1.
    tx: f32,
    /// Y axis translation.
    /// Ranges from -1 to 1.
    ty: f32,
    /// Image projection matrix.
    /// It has mostly one job: preserve ratio of the image.
    image: Mat4,
    /// Original image projection matrix without translation and scale
    /// transformations applied.
    aspect_image: Mat4,
    /// Used to rebuild image projection matrix in case of resize event.
    image_width: u32,
    /// Used to rebuild image projection matrix in case of resize event.
    image_height: u32,
    /// ID of the u_projection uniform where image matrix is stored.
    gl_uniform: i32,
}

/// Store current translation, zoom, projection matrix.
impl State {
    fn new(
        image_width: u32,
        image_height: u32,
        screen_width: u32,
        screen_height: u32,
        gl_uniform: i32,
    ) -> Self {
        let aspect_matrix = Mat4::from_scale(Self::get_aspect(
            image_width,
            image_height,
            screen_width,
            screen_height,
        ));
        Self {
            tx: 0.0,
            ty: 0.0,
            zoom: 1.0,
            image: aspect_matrix,
            aspect_image: aspect_matrix,
            image_width,
            image_height,
            gl_uniform,
        }
    }

    /// Get scale vector so that the image can preserve its aspect ratio.
    fn get_aspect(img_w: u32, img_h: u32, screen_w: u32, screen_h: u32) -> Vec3 {
        let img_aspect = to_f(img_w) / to_f(img_h);
        let screen_aspect = to_f(screen_w) / to_f(screen_h);

        if img_aspect > screen_aspect {
            Vec3::new(1.0, screen_aspect / img_aspect, 1.0)
        } else {
            Vec3::new(img_aspect / screen_aspect, 1.0, 1.0)
        }
    }

    /// Update u_projection uniform on the GPU.
    fn update_uniform(&self) -> &Self {
        unsafe {
            gl::UniformMatrix4fv(
                self.gl_uniform,
                1,
                gl::FALSE,
                self.image.as_ref() as *const _,
            );
        }

        self
    }

    /// Handle window resize.
    fn window_resized(&mut self, screen_width: u32, screen_height: u32) -> &mut Self {
        // Create completely new aspect ratio matrix.
        self.image = Mat4::from_scale(Self::get_aspect(
            self.image_width,
            self.image_height,
            screen_width,
            screen_height,
        ));
        self.aspect_image = self.image;
        self.apply_transformations();
        self.update_uniform();
        self
    }

    fn apply_transformations(&mut self) -> &mut Self {
        // Apply zoom.
        self.image.x_axis.x = self.aspect_image.x_axis.x * self.zoom;
        self.image.y_axis.y = self.aspect_image.y_axis.y * self.zoom;
        // Apply translation.
        self.image.x_axis.w = self.tx;
        self.image.y_axis.w = self.ty;
        // Aaand done.
        self
    }

    fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        self.tx = self.tx + x;
        self.ty = self.ty + y;
        self.clamp_translate();
        self.apply_transformations();
        self.update_uniform();
        self
    }

    /// Make sure tx and ty always stays in the range.
    /// Of course, take into account zoom level so it is possible to zoom in
    /// at the edge of the image.
    fn clamp_translate(&mut self) -> &mut Self {
        let min = -1.0 * self.zoom;
        let max = 1.0 * self.zoom;
        self.tx = self.tx.clamp(min, max);
        self.ty = self.ty.clamp(min, max);
        self
    }

    fn zoom(&mut self, delta: f32) -> &mut Self {
        self.zoom = (self.zoom * (1.0 + delta)).clamp(0.5, 4.0);
        self.clamp_translate();
        self.apply_transformations();
        self.update_uniform();
        self
    }
}

fn main() {
    //! Goal of this example:
    //! - Draw rectangle filled with provided texture.
    //! - Create model projection matrix that will project rectangle to preserve aspect ratio of the
    //!   original texture.
    //! - Allow zoom and scroll of the image.

    let glin = Glindow::new();
    let mut program = Program::create();
    let texture = Texture::create_from_file("./examples/resources/opossum.jpg", gl::TEXTURE_2D, 0)
        .expect("Texture created.");
    program
        .attach_shader_source("./examples/shaders/text_aspect.vert", gl::VERTEX_SHADER)
        .and_then(|p| {
            p.attach_shader_source("./examples/shaders/text_aspect.frag", gl::FRAGMENT_SHADER)
        })
        .and_then(|p| p.link())
        .expect("Program created.")
        .use_program();

    let mut state = {
        let size = glin.window.inner_size();
        State::new(
            texture.width,
            texture.height,
            size.width,
            size.height,
            program.get_uniform("u_projection\0"),
        )
    };
    let mut vao: u32 = 0;
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    let mut attrs = Attributes::new();

    // General setup, binding static data.
    opengl! {
        // Vertex Arrays.
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    // Vertex Buffer.
    #[rustfmt::skip]
    vbo.bind().data::<f32>(&[
        // Position  Texture
        1.0,  1.0,   1.0,  1.0, // Top Right
        1.0, -1.0,   1.0,  0.0, // Bottom Right
       -1.0, -1.0,   0.0,  0.0, // Bottom Left
       -1.0,  1.0,   0.0,  1.0, // Top Left
    ]);

    attrs
        // Vertex position attribute.
        .add::<f32>(0, 2, gl::FLOAT)
        // Texture coordinate attribute.
        .add::<f32>(1, 2, gl::FLOAT)
        .bind();

    // Index buffer for DrawElements.
    #[rustfmt::skip]
    ebo.bind().data::<u32>(&[
        0, 2, 1,
        0, 3, 2,
    ]);

    // Set texture as active.
    texture.bind();

    opengl! {
        gl::Uniform1i(program.get_uniform("uTexture\0"), 0);
    }

    // Set u_projection uniform value.
    state.update_uniform();

    // Start event loop.
    #[allow(unused_variables)]
    let Glindow {
        window,
        event_loop,
        display,
        surface,
        context,
    } = glin;

    event_loop.run(move |event, _, control_flow| {
        use glutin::prelude::*;
        use winit::event::{Event, MouseScrollDelta, WindowEvent};

        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    // Delete the only resource without Drop impl.
                    opengl! {
                        gl::DeleteVertexArrays(1, &vao);
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        surface.resize(
                            &context,
                            size.width.try_into().unwrap(),
                            size.height.try_into().unwrap(),
                        );
                        state.window_resized(size.width, size.height);
                        opengl! {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }
                        window.request_redraw();
                    }
                }
                WindowEvent::TouchpadMagnify { delta, .. } => {
                    state.zoom(delta as f32);
                    window.request_redraw();
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, __) => {
                            // Do noting.
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            let size = window.inner_size();
                            let w: f64 = size.width.into();
                            let h: f64 = size.height.into();

                            state.translate((pos.x / w) as f32, -1.0 * (pos.y / h) as f32);
                            window.request_redraw();
                        }
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                }
                surface.swap_buffers(&context).expect("I want to swap!");
            }
            _ => (),
        }
    });
}
