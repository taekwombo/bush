//! Providing uniforms to shaders.
//!
//! Animation advances only when WindowEvent::CursorMoved is triggered.
//!
//! Supported GLSL: 4.10
//! https://registry.khronos.org/OpenGL/specs/gl/glspec41.core.pdf

use gluty::{Glindow, Program, gl_call};
use std::mem::size_of;
use winit::event::{Event, WindowEvent};
use glutin::prelude::*;

fn main() {
    #[allow(unused_variables)]
    let Glindow { window, event_loop, display, surface, context } = Glindow::new();

    surface.set_swap_interval(
        &context,
        glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(5).unwrap()),
    ).expect("Set interval OK.");

    let mut frame: u32 = 0;
    let mut program = Program::create();
    program
        .attach_shader_source("./examples/shaders/uniform.vert", gl::VERTEX_SHADER)
        .and_then(|p| p.attach_shader_source("./examples/shaders/uniform.frag", gl::FRAGMENT_SHADER))
        .and_then(|p| p.link())
        .expect("Program created.");

    let positions: &[f32] = &[
        -0.5,  0.5,     // 0
         0.5,  0.5,     // 1
         0.5, -0.5,     // 2
        -0.5, -0.5,     // 3
    ];
    let indices: &[u32] = &[
        0, 2, 1,
        0, 3, 2,
    ];
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;
    #[allow(unused_assignments)]
    let mut u_col: i32 = 0;
    #[allow(unused_assignments)]
    let mut u_frame: i32 = 0;

    unsafe {
        gl::ClearColor(0.0, 0.4, 0.7, 1.0);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * size_of::<f32>()) as isize,
            positions.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (2 * size_of::<f32>()) as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        program.use_program();
        #[cfg(debug_assertions)]
        program.validate().expect("Program to be valid");

        u_col = gl::GetUniformLocation(program.gl_id, "uColor\0".as_ptr() as *const _);
        u_frame = gl::GetUniformLocation(program.gl_id, "uFrame\0".as_ptr() as *const _);

        debug_assert!(u_col >= 0, "uColor uniform must be used in shader.");
        debug_assert!(u_frame >= 0, "uFrame uniform must be used in shader.");

        // Set "uColor" uniform to white.
        gl::Uniform4f(u_col, 1.0, 1.0, 1.0, 1.0);
    }

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
                frame = frame.wrapping_add(1);
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::Uniform1ui(u_frame, frame);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        indices.len() as i32,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
                surface.swap_buffers(&context).expect("swap_buffers OK.");
            },
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { .. },
                ..
            } => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                unsafe {
                    gl_call!(gl::DeleteBuffers(1, &vao));
                    gl_call!(gl::DeleteBuffers(1, &vbo));
                    gl_call!(gl::DeleteBuffers(1, &ebo));
                }
                control_flow.set_exit();
            },
            _ => (),
        }
    });
}