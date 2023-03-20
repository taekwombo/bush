use super::opengl;

pub struct Texture {
    pub gl_id: u32,
    gl_type: u32,
    slot: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    #[allow(clippy::result_unit_err)]
    pub fn create<S: AsRef<std::path::Path>>(path: S, gl_type: u32, slot: u32) -> Result<Self, ()> {
        use image::io::Reader;

        let image = Reader::open(path.as_ref())
            .map_err(|_| ())?
            .decode()
            .map_err(|_| ())?
            .flipv()
            .into_rgba32f();

        let (width, height) = image.dimensions();

        let mut gl_id = 0;

        opengl! {
            gl::GenTextures(1, &mut gl_id);

            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl_type, gl_id);
            gl::TexParameteri(gl_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexImage2D(
                gl_type,
                0,
                gl::RGBA32F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                image.as_raw().as_ptr() as *const _,
            );
            gl::BindTexture(gl_type, 0);
        }

        Ok(Self {
            gl_id,
            gl_type,
            slot: gl::TEXTURE0 + slot,
            width,
            height,
        })
    }

    pub fn bind(&self) -> &Self {
        opengl! {
            gl::ActiveTexture(self.slot);
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
