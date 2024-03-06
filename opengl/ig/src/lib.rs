use gluty::winit::dpi::PhysicalSize;

mod input;
mod light;
mod so_project;

pub use input::*;
pub use light::*;
pub use so_project::*;

pub fn size_u_to_f32(size: &PhysicalSize<u32>) -> PhysicalSize<f32> {
    debug_assert!(size.width <= std::i32::MAX as u32);
    debug_assert!(size.height <= std::i32::MAX as u32);

    let width = size.width as f32;
    let height = size.height as f32;
    PhysicalSize::new(width, height)
}
