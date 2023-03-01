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
            acc + attr.elem_size * attr.elem_count as u32
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

    pub fn bind(&self) -> () {
        let mut offset = 0;
        let stride = self.get_stride();

        for (index, attr) in self.attrs.iter().enumerate() {
            unsafe {
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
    }
}

pub struct Buffer {
    pub gl_id: u32,
    gl_type: u32,
}

impl Buffer {
    pub fn new(gl_type: u32) -> Self {
        let mut gl_id: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut gl_id);
        }

        Self {
            gl_id,
            gl_type,
        }
    }

    pub unsafe fn data<T>(&self, data: &[T]) -> &Self {
        use std::mem::size_of;

        gl::BufferData(
            self.gl_type,
            (data.len() * size_of::<T>()) as isize,
            data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        self
    }

    pub unsafe fn bind(&self) -> &Self {
        gl::BindBuffer(self.gl_type, self.gl_id);
        self
    }

    pub unsafe fn unbind(&self) -> &Self {
        gl::BindBuffer(self.gl_type, 0);
        self
    }
}

impl Drop for Buffer {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id);
        }
    }
}

pub struct Texture {
    pub gl_id: u32,
    gl_type: u32,
    slot: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
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

        unsafe {
            gl::GenTextures(1, &mut gl_id);
            gl::BindTexture(gl_type, gl_id);
            gl::TexParameteri(gl_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl_type, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
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

    pub unsafe fn bind(&self) -> &Self {
        gl::ActiveTexture(self.slot);
        gl::BindTexture(self.gl_type, self.gl_id);

        self
    }

    pub unsafe fn unbind(&self) -> &Self {
        gl::BindTexture(self.gl_type, 0);

        self
    }
}

impl Drop for Texture {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id);
        }
    }
}
