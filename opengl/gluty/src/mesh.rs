use super::{opengl, Attributes, Buffer, Material};
use glam::{Mat4, Vec3};

#[derive(Debug)]
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
    pub material: Option<Material>,
}

impl Mesh {
    pub fn new<F>(vbo_data: &[f32], ebo_data: &[u32], add_attrs: F) -> Self
    where
        F: FnOnce(&mut Attributes),
    {
        let mut vao: u32 = 0;
        let mut attrs = Attributes::new();

        opengl! {
            // Create and bind Vertex Array.
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        // Bind vertex buffer.
        let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.bind().data(vbo_data);
        // Bind element buffer.
        let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        ebo.bind().data(ebo_data);

        add_attrs(&mut attrs);
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
            material: None,
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

    pub fn rotate_y(&mut self, y_deg: f32) -> &mut Self {
        self.model_to_world *= Mat4::from_rotation_y(y_deg.to_radians());
        self
    }

    pub fn rotate_x(&mut self, x_deg: f32) -> &mut Self {
        self.model_to_world *= Mat4::from_rotation_x(x_deg.to_radians());
        self
    }

    pub fn rotate_z(&mut self, z_deg: f32) -> &mut Self {
        self.model_to_world *= Mat4::from_rotation_z(z_deg.to_radians());
        self
    }

    pub fn bind_vao(&self) {
        opengl!(gl::BindVertexArray(self.vao));
    }

    pub fn unbind_vao(&self) {
        opengl!(gl::BindVertexArray(0));
    }

    pub fn draw_as(&self, mode: gl::types::GLenum) -> &Self {
        self.bind_vao();
        opengl! {
            gl::DrawElements(mode, self.indices, gl::UNSIGNED_INT, std::ptr::null());
        }
        self.unbind_vao();
        self
    }

    pub fn draw(&self) -> &Self {
        self.draw_as(gl::TRIANGLES)
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        opengl! {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
