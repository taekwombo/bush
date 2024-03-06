use super::opengl;
use gl::types::GLenum;

fn compile_shader(shader_source: &[u8], shader_type: GLenum) -> Result<u32, &'static str> {
    let src_len: i32 = i32::try_from(shader_source.len()).expect("Shader length must fit in i32.");
    let src_ptr = shader_source.as_ptr() as *const _;

    let shader = opengl!(gl::CreateShader(shader_type));

    opengl! {
        gl::ShaderSource(shader, 1, &src_ptr, &src_len);
        gl::CompileShader(shader);
    }

    let mut status: i32 = 0;
    opengl!(gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status));

    #[cfg(debug_assertions)]
    if status == gl::FALSE as i32 {
        eprintln!("Failed to compile shader.");
        dbg!(String::from_utf8_lossy(shader_source));

        let mut info_len: i32 = 0;
        opengl!(gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_len));

        debug_assert!(info_len > 0);
        let mut info_log: Vec<u8> = Vec::with_capacity(info_len as usize);
        let info_ptr = info_log.as_mut_ptr();
        let mut written: i32 = 0;

        opengl!(gl::GetShaderInfoLog(
            shader,
            info_len,
            &mut written,
            info_ptr as *mut _
        ));
        unsafe {
            info_log.set_len(written as usize);
        }
        eprintln!("{}", String::from_utf8_lossy(&info_log));

        opengl!(gl::DeleteShader(shader));

        return Err("Failed to compile shader.");
    }

    Ok(shader)
}

#[derive(Debug)]
pub struct Program {
    pub gl_id: u32,
    shaders: [u32; 8],
    shader_count: u8,
    error: Option<String>,
}

impl Program {
    /// Note: name must end with \0 character.
    pub fn get_uniform(&self, name: &str) -> i32 {
        debug_assert!(name.get((name.len() - 1)..).unwrap() == "\0");
        opengl!(gl::GetUniformLocation(
            self.gl_id,
            name.as_ptr() as *const _
        ))
    }

    #[inline]
    pub fn use_program(&self) -> &Self {
        opengl!(gl::UseProgram(self.gl_id));
        self
    }

    pub fn shader(mut self, source: &[u8], shader_type: GLenum) -> Self {
        if self.error.is_some() {
            return self;
        }

        let shader = match compile_shader(source, shader_type) {
            Ok(sh) => sh,
            Err(err) => {
                self.error = Some(err.to_owned());
                return self;
            }
        };

        self.attach_shader(shader);
        self
    }

    pub fn link(mut self) -> Self {
        if self.error.is_some() {
            return self;
        }

        opengl!(gl::LinkProgram(self.gl_id));
        self.check_program_iv(gl::LINK_STATUS);
        self
    }

    pub fn validate(&mut self) -> &mut Self {
        if self.error.is_some() {
            return self;
        }

        opengl!(gl::ValidateProgram(self.gl_id));
        self.check_program_iv(gl::VALIDATE_STATUS)
    }

    fn attach_shader(&mut self, shader: u32) -> &mut Self {
        opengl!(gl::AttachShader(self.gl_id, shader));
        self.shaders[self.shader_count as usize] = shader;
        self.shader_count += 1;
        self
    }

    pub fn check_errors(program: &Program) -> Result<(), &str> {
        match &program.error {
            Some(err) => {
                eprintln!("Program {} raised errors: {}", program.gl_id, err);
                Err(err)
            }
            None => Ok(()),
        }
    }

    fn check_program_iv(&mut self, ivtype: gl::types::GLenum) -> &mut Self {
        let mut status = 0;
        opengl!(gl::GetProgramiv(self.gl_id, ivtype, &mut status));

        if status == gl::FALSE as i32 {
            eprintln!("Program validation failed.");
            self.error = Some(self.get_program_info());
        }

        self
    }

    fn get_program_info(&self) -> String {
        static mut LOGS: [u8; 512] = [0; 512];

        let mut info_len: i32 = 0;
        opengl!(gl::GetProgramiv(
            self.gl_id,
            gl::INFO_LOG_LENGTH,
            &mut info_len
        ));

        println!("Info length for program {}: {}", self.gl_id, info_len);
        debug_assert!(info_len > 0);

        let info_ptr = unsafe { LOGS.as_mut_ptr() };
        let mut written: i32 = 0;

        opengl!(gl::GetProgramInfoLog(
            self.gl_id,
            512,
            &mut written,
            info_ptr as *mut _
        ));

        format!("{}", unsafe { String::from_utf8_lossy(&LOGS) })
    }
}

impl Default for Program {
    fn default() -> Self {
        Self {
            gl_id: opengl!(gl::CreateProgram()),
            shaders: [0; 8],
            shader_count: 0,
            error: None,
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        for i in 0..self.shader_count {
            opengl!(gl::DeleteShader(self.shaders[i as usize]));
        }
        opengl!(gl::DeleteProgram(self.gl_id));
    }
}
