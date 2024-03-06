//! Displaying texture on rectangle.
//! The image displayed must be present at gluty/examples/resources/opossum.jpg.

use gluty::{assets, opengl, Glindow, Program};
use std::mem::size_of;

fn main() {
    let glin = Glindow::new();
    let (vert, frag, img) = assets!("./texture.vert", "./texture.frag", "./opossum.jpg");

    let program = Program::default()
        .shader(vert.get(), gl::VERTEX_SHADER)
        .shader(frag.get(), gl::FRAGMENT_SHADER)
        .link();

    let image = img
        .try_to_img()
        .expect("image missing")
        .flipv()
        .into_rgba32f();
    let (i_width, i_height) = image.dimensions();

    let mut texture_id: u32 = 0;
    #[rustfmt::skip]
    let positions: &[f32] = &[
        -0.5,  0.5, 0.0, 1.0,     // 0
         0.5,  0.5, 1.0, 1.0,     // 1
         0.5, -0.5, 1.0, 0.0,     // 2
        -0.5, -0.5, 0.0, 0.0,     // 3
    ];
    #[rustfmt::skip]
    let indices: &[u32] = &[
        0, 2, 1,
        0, 3, 2,
    ];
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    program.use_program();

    opengl! {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Generate and bind VAO.
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generate vertex buffer.
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * size_of::<f32>()) as isize,
            positions.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        // Enable vertex position attribute.
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (4 * size_of::<f32>()) as i32,
            0 as *const _,
        );
        // Enable vertex texture coordinate attribute.
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (4 * size_of::<f32>()) as i32,
            (2 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        // Generate Index Buffer.
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Generate texture.
        gl::GenTextures(1, &mut texture_id);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA32F as i32,
            i_width as i32,
            i_height as i32,
            0,
            gl::RGBA,
            gl::FLOAT,
            image.as_raw().as_ptr() as *const _
        );

        // Set uTexture uniform value to bound texure slot.
        gl::Uniform1i(program.get_uniform("uTexture\0"), 0);
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
        use winit::event::{Event, WindowEvent};
        use glutin::prelude::*;

        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    opengl! {
                        gl::DeleteVertexArrays(1, &vao);
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteBuffers(1, &ebo);
                        gl::DeleteTextures(1, &texture_id);
                    }
                },
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        surface.resize(
                            &context,
                            size.width.try_into().unwrap(),
                            size.height.try_into().unwrap(),
                        );
                        opengl!(gl::Viewport(
                            0,
                            0,
                            size.width as i32,
                            size.height as i32,
                        ));
                        window.request_redraw();
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
                };
                surface.swap_buffers(&context).expect("I want to swap!");
            },
            _ => (),
        }
    });
}
