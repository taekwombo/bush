use gluty::gl;
use gluty::glam::{Mat4, Vec4};
use gluty::{opengl, Mesh, Program};

const VERTEX_SOURCE: &[u8] = b"
#version 410 core

uniform mat4 u_proj_t;
uniform mat4 u_view_t;
uniform mat4 u_model_t;

layout(location = 0) in vec3 a_position;

void main() {
    gl_Position = u_proj_t * u_view_t * u_model_t * vec4(a_position, 1.0);
}
";

const FRAGMENT_SOURCE: &[u8] = b"
#version 410 core

layout(location = 0) out vec4 color;

void main() {
    color = vec4(1.0);
}
";

#[rustfmt::skip]
const CUBE_VERTICES: &[f32] = &[
    -1.0,  1.0,  1.0, // 0  3─────2
     1.0,  1.0,  1.0, // 1  │╲    │╲
     1.0,  1.0, -1.0, // 2  │ 0───┼─1
    -1.0,  1.0, -1.0, // 3  │ │   │ │
    -1.0, -1.0,  1.0, // 4  7─┼───6 │
     1.0, -1.0,  1.0, // 5   ╲│    ╲│
     1.0, -1.0, -1.0, // 6    4─────5
    -1.0, -1.0, -1.0, // 7
];

const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 2, 3, 0, // Top
    4, 5, 6, 6, 7, 4, // Bottom
    4, 5, 1, 1, 0, 4, // Front
    7, 6, 2, 2, 3, 7, // Back
    7, 4, 0, 0, 3, 7, // Left
    6, 5, 1, 1, 2, 6, // Right
];

/// Cube illuminating the scene.
pub struct Light {
    pub mesh: Mesh,
    /// Program drawing single colored cube.
    pub program: Program,
    /// Light color.
    pub color: Vec4,
    /// Light position in world coordinate space.
    pub position: Mat4,
    pub u_model: i32,
    pub u_view: i32,
    pub u_proj: i32,
}

impl Light {
    pub fn new() -> Self {
        let mesh = Mesh::new(CUBE_VERTICES, CUBE_INDICES, |attrs| {
            attrs.add::<f32>(0, 3, gl::FLOAT);
        });

        let mut program = Program::create();
        program
            .attach_shader_source_str(VERTEX_SOURCE, gl::VERTEX_SHADER)
            .and_then(|p| p.attach_shader_source_str(FRAGMENT_SOURCE, gl::FRAGMENT_SHADER))
            .and_then(|p| p.link())
            .unwrap();

        Self {
            color: Vec4::ONE,
            position: Mat4::IDENTITY,
            u_model: program.get_uniform("u_model_t\0"),
            u_view: program.get_uniform("u_view_t\0"),
            u_proj: program.get_uniform("u_proj_t\0"),
            program,
            mesh,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.position.w_axis.x += x;
        self.position.w_axis.y += y;
        self.position.w_axis.z += z;
        self
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.position.x_axis.x = x;
        self.position.y_axis.y = y;
        self.position.z_axis.z = z;
        self
    }

    pub fn upload_uniforms(&self, u_view: &Mat4, u_proj: &Mat4) -> &Self {
        self.program.use_program();

        opengl! {
            gl::UniformMatrix4fv(self.u_model, 1, gl::FALSE, self.position.as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_view, 1, gl::FALSE, u_view.as_ref() as *const _);
            gl::UniformMatrix4fv(self.u_proj, 1, gl::FALSE, u_proj.as_ref() as *const _);
        }

        self
    }

    pub fn draw(&mut self) -> &mut Self {
        self.program.use_program();
        self.mesh.draw();

        opengl! {
            gl::UseProgram(0);
        }

        self
    }

    pub fn get_color(&self) -> Vec4 {
        self.color
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new()
    }
}
