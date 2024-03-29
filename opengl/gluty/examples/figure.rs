//! Let's start it easy. How about a triangle?!
//!
//! https://docs.gl/

use gluty::{assets, opengl, Glindow, Program};
use std::mem::size_of;

fn main() {
    let glin = Glindow::new();
    let (vert, frag) = assets!("./figure.vert", "./figure.frag");
    let mut program = Program::default()
        .shader(vert.get(), gl::VERTEX_SHADER)
        .shader(frag.get(), gl::FRAGMENT_SHADER)
        .link();

    // Vertex((Attribute Position: f32 f32 f32), (Attribute Normal: f32 f32 f32))
    //
    // VertexAttribPointer(
    //   0      - first attribute
    //   3      - 3 elements in the first attribute
    //   FLOAT  - those 3 elements are floats (4 bytes each)
    //   FALSE  - should be normalized? No.
    //   24     - Stride: How many bytes should OpenGL move to the next vertex - size of vertex.
    //   0      - How many bytes should I move form the start of the Vertex to get Attribute data.
    // )
    //
    // VertexAttribPointer(
    //   1
    //   3
    //   FLOAT
    //   FALSE
    //   24
    //   12
    // )
    //
    // Data for ARRAY_BUFFER.
    // Mapped by VertexAttribPointer.
    #[rustfmt::skip]
    let positions: &[f32] = &[
        -0.5,  0.5,     // 0
         0.5,  0.5,     // 1
         0.5, -0.5,     // 2
        -0.5, -0.5,     // 3
         0.0,  0.75,    // 4
         0.75, 0.0,     // 5
         0.0, -0.75,    // 6
        -0.75, 0.0,     // 7
    ];

    // Data for ELEMENT_ARRAY_BUFFER.
    #[rustfmt::skip]
    let indices: &[u32] = &[
        0, 2, 1,
        0, 3, 2,
        1, 4, 0,
        1, 2, 5,
        2, 3, 6,
        3, 0, 7,
    ];

    let mut vbo: u32 = 0; // Vertices
    let mut vao: u32 = 0; // Attributes
    let mut ibo: u32 = 0; // Indices
    let mut vbo2: u32 = 0;

    // Data for Instanced Draw.
    // Vertex attributes are considered instanced if their divisor is greater than 0.
    // This can be set using VertexAttribDivisor(uint, uint);
    let instance_data: &[f32] = &[
        1.9, 1.8, 1.7, 1.6, 1.5, 1.4, 1.3, 1.2, 1.1, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2,
    ];

    opengl! {
        gl::ClearColor(1.0, 0.0, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * size_of::<f32>()) as isize,
            positions.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // https://learnopengl.com/Getting-started/Hello-Triangle
        // > Core OpenGL requires that we use a VAO so it knows what to do with our vertex inputs.
        // > If we fail to bind a VAO, OpenGL will most likely refuse to draw anything.
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (2 * size_of::<f32>()) as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::GenBuffers(1, &mut vbo2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (instance_data.len() * size_of::<f32>()) as isize,
            instance_data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE, (1 * size_of::<f32>()) as i32, std::ptr::null());
        gl::VertexAttribDivisor(1, 1);
        gl::EnableVertexAttribArray(1);

        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

    }

    program.use_program();
    program.validate();
    Program::check_errors(&program).expect("Program should have no errors");

    opengl! {
        // gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        gl::DrawElementsInstanced(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _, instance_data.len() as i32);
    }

    glin.run_until_close();
}
