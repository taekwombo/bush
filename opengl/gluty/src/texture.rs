use super::opengl;

#[derive(Debug)]
pub struct Texture {
    pub gl_id: u32,
    gl_type: u32,
    pub slot: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    #[allow(clippy::result_unit_err)]
    pub fn create_from_file<S: AsRef<std::path::Path>>(
        path: S,
        gl_type: u32,
        slot: u32,
    ) -> Result<Self, ()> {
        use image::io::Reader;

        let image = Reader::open(path.as_ref())
            .map_err(|_| ())?
            .decode()
            .map_err(|_| ())?
            .flipv()
            .into_rgba32f();
        let (width, height) = image.dimensions();

        let tex = Texture::new(gl_type, slot, width, height);

        opengl! {
            tex.bind();
            tex.data(image.as_raw());
            tex.unbind();
        }

        Ok(tex)
    }

    pub fn new(gl_type: u32, slot: u32, width: u32, height: u32) -> Self {
        let mut tex = Self {
            gl_type,
            gl_id: 0,
            slot,
            width,
            height,
        };

        opengl! {
            gl::GenTextures(1, &mut tex.gl_id);
            tex.bind();
            tex.parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            tex.parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            tex.parameter(gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            tex.parameter(gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            tex.unbind();
        }

        tex
    }

    unsafe fn parameter(&self, param: u32, value: i32) {
        gl::TexParameteri(self.gl_type, param, value);
    }

    unsafe fn data(&self, data: &[f32]) {
        gl::TexImage2D(
            self.gl_type,
            0,
            gl::RGBA32F as i32,
            self.width as i32,
            self.height as i32,
            0,
            gl::RGBA,
            gl::FLOAT,
            data.as_ptr() as *const _,
        );
    }

    pub fn bind(&self) -> &Self {
        opengl! {
            gl::ActiveTexture(gl::TEXTURE0 + self.slot);
            gl::BindTexture(self.gl_type, self.gl_id);
        }

        self
    }

    pub fn unbind(&self) -> &Self {
        opengl! {
            gl::BindTexture(self.gl_type, 0);
        }

        self
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        opengl! {
            gl::DeleteTextures(1, &self.gl_id);
        }
    }
}
