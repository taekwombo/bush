use super::opengl;
use image::{io::Reader, ImageBuffer, Rgba};

type Image = ImageBuffer<Rgba<f32>, Vec<f32>>;

#[derive(Debug)]
pub struct Texture {
    pub gl_id: u32,
    pub gl_type: u32,
    pub slot: u32,
    pub width: u32,
    pub height: u32,
}

pub trait Lte6 {}
pub struct Check<const N: usize> {}
impl Lte6 for Check<1> {}
impl Lte6 for Check<2> {}
impl Lte6 for Check<3> {}
impl Lte6 for Check<4> {}
impl Lte6 for Check<5> {}
impl Lte6 for Check<6> {}

impl Texture {
    #[allow(clippy::result_unit_err)]
    pub fn load_files<const N: usize>(paths: [&str; N], flip: bool) -> Result<[Image; N], ()>
    where
        Check<N>: Lte6,
    {
        use std::mem::MaybeUninit;
        use std::thread::spawn;

        let mut handles: [MaybeUninit<_>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for i in 0..N {
            let path = paths[i].to_owned();
            handles[i] = MaybeUninit::new(spawn(move || {
                println!("Loading {path}");
                Texture::load_file(path, flip)
            }));
        }

        let mut images: [Image; N] = unsafe { std::mem::zeroed() };

        for (i, handle) in handles.into_iter().enumerate() {
            images[i] = unsafe { handle.assume_init() }.join().unwrap()?;
        }

        Ok(images)
    }

    #[allow(clippy::result_unit_err)]
    pub fn load_file<S>(path: S, flipv: bool) -> Result<Image, ()>
    where
        S: AsRef<std::path::Path>,
    {
        let img = Reader::open(path.as_ref())
            .map_err(|_| ())?
            .decode()
            .map_err(|_| ())?;

        Ok((if flipv { img.flipv() } else { img }).into_rgba32f())
    }

    #[allow(clippy::result_unit_err)]
    pub fn create_from_file<S: AsRef<std::path::Path>>(
        path: &S,
        gl_type: u32,
        slot: u32,
    ) -> Result<Self, ()> {
        let Ok(image) = Texture::load_file(path, true) else {
            eprintln!("Yikes, make sure that {} contains a texture (relative to PWD).", path.as_ref().to_string_lossy());
            return Err(());
        };
        let (width, height) = image.dimensions();

        let tex = Texture::new(gl_type, slot, width, height);

        tex.bind();
        tex.data(image.as_raw(), None);
        tex.unbind();

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

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn parameter(&self, param: u32, value: i32) {
        gl::TexParameteri(self.gl_type, param, value);
    }

    pub fn data(&self, data: &[f32], ty: Option<u32>) {
        opengl! {
            gl::TexImage2D(
                ty.unwrap_or(self.gl_type),
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
