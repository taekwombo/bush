use super::opengl;

/// Contains all necessary data for the following function calls.
///
/// https://docs.gl/gl4/glVertexAttribPointer
/// https://docs.gl/gl4/glEnableVertexAttribArray
struct Attribute {
    index: u32,
    elem_count: u32,
    elem_size: u32,
    gl_type: u32,
}

pub struct Attributes {
    attrs: Vec<Attribute>,
}

impl Attributes {
    fn get_stride(&self) -> u32 {
        self.attrs.iter().fold(0, |acc, attr| {
            acc + attr.elem_size * attr.elem_count
        })
    }

    pub fn new() -> Self {
        Self { attrs: Vec::new() }
    }

    pub fn add<T>(&mut self, index: u32, elem_count: u32, gl_type: u32) -> &mut Self {
        use std::mem::size_of;

        self.attrs.push(Attribute {
            index,
            elem_count,
            elem_size: size_of::<T>() as u32,
            gl_type,
        });

        self
    }

    pub fn bind(&self) -> &Self {
        let mut offset = 0;
        let stride = self.get_stride();

        // Check if Vertex Array object is bound.
        // Not needed since 4.5 thanks to `glEnableVertexArrayAttrib`.
        debug_assert!(0 != unsafe {
            let mut bound_vao: i32 = 0;
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut bound_vao);
            bound_vao
        });

        for (index, attr) in self.attrs.iter().enumerate() {
            opengl! {
                gl::VertexAttribPointer(
                    attr.index,
                    attr.elem_size as i32,
                    attr.gl_type,
                    gl::FALSE,
                    stride as i32,
                    offset as *const _,
                );
                gl::EnableVertexAttribArray(index as u32);
            }

            offset += attr.elem_count * attr.elem_size;
        }

        self
    }
}

