use std::path::Path;
use gl::types::GLenum;
use super::opengl;

fn load_shader_from_path<P>(path: &P, shader_type: GLenum) -> Result<u32, &'static str>
where P: AsRef<Path>
{
    use std::fs::read;

    let shader_source = read(path.as_ref()).expect("Shader source to exist.");
    compile_shader(&shader_source, shader_type)
}

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

        let mut info_len: i32 = 0;
        opengl!(gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_len));

        debug_assert!(info_len > 0);
        let mut info_log: Vec<u8> = Vec::with_capacity(info_len as usize);
        let info_ptr = info_log.as_mut_ptr();
        let mut written: i32 = 0;

        opengl!(gl::GetShaderInfoLog(shader, info_len, &mut written, info_ptr as *mut _));
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
    shader_cnt: u8,
}

impl Program {
    pub fn create() -> Self {
        Self {
            gl_id: opengl!(gl::CreateProgram()),
            shaders: [0; 8],
            shader_cnt: 0,
        }
    }

    /// Note: name must end with \0 character.
    pub fn get_uniform(&self, name: &str) -> i32 {
        debug_assert!(name.get((name.len() - 1)..).unwrap() == "\0");
        opengl!(
            gl::GetUniformLocation(self.gl_id, name.as_ptr() as *const _)
        )
    }

    pub fn attach_shader(&mut self, shader: u32) -> Result<&mut Self, ()> {
        opengl!(gl::AttachShader(self.gl_id, shader));
        self.shaders[self.shader_cnt as usize] = shader;
        self.shader_cnt += 1;

        Ok(self)
    }

    pub fn attach_shader_source_str(&mut self, source: &[u8], shader_type: GLenum) -> Result<&mut Self, ()> {
        let shader = match compile_shader(source, shader_type) {
            Err(err) => {
                eprintln!("[shader_source_str]: {}", err);
                return Err(());
            },
            Ok(shader) => shader,
        };

        self.attach_shader(shader)
    }

    pub fn attach_shader_source<P: AsRef<Path>>(&mut self, path: P, shader_type: GLenum) -> Result<&mut Self, ()> {
        let shader = match load_shader_from_path(&path, shader_type) {
            Err(err) => {
                eprintln!("[{}]: {}", path.as_ref().to_string_lossy(), err);
                return Err(());
            },
            Ok(shader) => shader,
        };

        self.attach_shader(shader)
    }

    pub fn link(&self) -> Result<&Self, ()> {
        opengl!(gl::LinkProgram(self.gl_id));
        self.check_program_iv(gl::LINK_STATUS)
    }

    fn log_program_info(&self) {
        static mut LOGS: [u8; 512] = [0; 512];

        let mut info_len: i32 = 0;
        opengl!(gl::GetProgramiv(self.gl_id, gl::INFO_LOG_LENGTH, &mut info_len));

        println!("Info len: {}", info_len);
        debug_assert!(info_len > 0);

        let info_ptr = unsafe { LOGS.as_mut_ptr() };
        let mut written: i32 = 0;

        opengl!(gl::GetProgramInfoLog(self.gl_id, 512, &mut written, info_ptr as *mut _));
        eprintln!("{}", unsafe { String::from_utf8_lossy(&LOGS) });
    }

    fn check_program_iv(&self, ivtype: gl::types::GLenum) -> Result<&Self, ()> {
        let mut status = 0;
        opengl!(gl::GetProgramiv(self.gl_id, ivtype, &mut status));

        if status == gl::FALSE as i32 {
            #[cfg(debug_assertions)]
            {
                eprintln!("Program validation failed.");
                self.log_program_info();
            }
            return Err(());
        }

        Ok(self)
    }

    pub fn validate(&self) -> Result<&Self, ()> {
        opengl!(gl::ValidateProgram(self.gl_id));
        self.check_program_iv(gl::VALIDATE_STATUS)
    }

    #[inline]
    pub fn use_program(&self) -> &Self {
        opengl!(gl::UseProgram(self.gl_id));
        self
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        for i in 0..self.shader_cnt {
            opengl!(gl::DeleteShader(self.shaders[i as usize]));
        }
        opengl!(gl::DeleteProgram(self.gl_id));
    }
}
