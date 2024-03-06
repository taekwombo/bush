use super::opengl;
use gl::types::GLenum;

#[derive(Debug)]
pub struct Texture {
    pub gl_id: u32,
    pub gl_type: u32,
    pub slot: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn get_image_info(image: &image::DynamicImage) -> ((u32, u32), GLenum, GLenum) {
        use image::DynamicImage;

        match image {
            DynamicImage::ImageRgb8(img) => (img.dimensions(), gl::RGB, gl::UNSIGNED_BYTE),
            DynamicImage::ImageRgba8(img) => (img.dimensions(), gl::RGBA, gl::UNSIGNED_BYTE),
            DynamicImage::ImageRgb16(img) => (img.dimensions(), gl::RGB, gl::UNSIGNED_SHORT),
            DynamicImage::ImageRgba16(img) => (img.dimensions(), gl::RGBA, gl::UNSIGNED_SHORT),
            DynamicImage::ImageRgb32F(img) => (img.dimensions(), gl::RGB, gl::FLOAT),
            DynamicImage::ImageRgba32F(img) => (img.dimensions(), gl::RGBA, gl::FLOAT),
            _ => unimplemented!(),
        }
    }

    pub fn from_image(
        gl_type: GLenum,
        slot: u32,
        image: &image::DynamicImage,
        texture_format: GLenum,
    ) -> Self {
        let ((width, height), dataf, datat) = Texture::get_image_info(image);
        let tex = Texture::new(gl_type, slot, width, height);
        tex.bind()
            .data(texture_format, dataf, datat, image.as_bytes())
            .unbind();
        tex
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

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn parameter(&self, param: u32, value: i32) {
        gl::TexParameteri(self.gl_type, param, value);
    }

    pub fn data<T>(
        &self,
        tex_format: GLenum,
        data_format: GLenum,
        data_type: GLenum,
        data: &[T],
    ) -> &Self {
        self.data_with_type(
            self.gl_type,
            tex_format,
            data_format,
            data_type,
            data,
        )
    }

    pub fn data_with_type<T>(
        &self,
        tex_type: GLenum,
        tex_format: GLenum,
        data_format: GLenum,
        data_type: GLenum,
        data: &[T],
    ) -> &Self {
        opengl! {
            gl::TexImage2D(
                tex_type,
                0,
                tex_format as i32,
                self.width as i32,
                self.height as i32,
                0,
                data_format,
                data_type,
                data.as_ptr() as *const _,
            )
        }
        self
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
