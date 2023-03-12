use super::opengl;

pub struct Buffer {
    pub gl_id: u32,
    gl_type: u32,
    gl_usage: u32,
}

impl Buffer {
    pub fn new(gl_type: u32, gl_usage: u32) -> Self {
        let mut gl_id: u32 = 0;

        opengl! {
            gl::GenBuffers(1, &mut gl_id);
        }

        Self {
            gl_id,
            gl_type,
            gl_usage,
        }
    }

    pub fn data<T>(&self, data: &[T]) -> &Self {
        use std::mem::size_of;

        opengl! {
            gl::BufferData(
                self.gl_type,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                self.gl_usage,
            );
        }

        self
    }

    pub fn bind(&self) -> &Self {
        opengl! {
            gl::BindBuffer(self.gl_type, self.gl_id);
        }
        self
    }

    pub fn unbind(&self) -> &Self {
        opengl! {
            gl::BindBuffer(self.gl_type, 0);
        }
        self
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        opengl! {
            gl::DeleteBuffers(1, &self.gl_id);
        }
    }
}
