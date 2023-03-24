use gluty::{gl, Program};

mod input;
mod light;
mod so_project;

pub use input::*;
pub use light::*;
pub use so_project::*;

#[allow(clippy::result_unit_err)]
pub fn create_program(dir: Option<&'static str>) -> Result<Program, ()> {
    let mut program = Program::create();

    if let Some(dir) = dir {
        program
            .attach_shader_source(format!("{}.vert", dir), gl::VERTEX_SHADER)
            .and_then(|p| p.attach_shader_source(format!("{}.frag", dir), gl::FRAGMENT_SHADER))
            .and_then(|p| p.link())?;
    }

    Ok(program)
}

pub fn get_model_path() -> String {
    let path_arg = std::env::args().nth(1);

    path_arg.map_or_else(|| String::from("./resources/teapot.obj"), |v| v)
}
