//! Module with any useful things needed to create a couple of OpenGL examples
//! without damaging C and V buttons.

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{
    ContextAttributesBuilder, GlProfile, NotCurrentGlContextSurfaceAccessor, PossiblyCurrentContext,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub use gl;
pub use glam;
pub use glutin;
pub use image;
pub use winit;

mod attributes;
mod buffer;
mod camera;
mod mesh;
mod obj;
mod program;
mod texture;

pub use attributes::*;
pub use buffer::*;
pub use camera::*;
pub use mesh::*;
pub use obj::*;
pub use program::*;
pub use texture::*;

/// All the things needed to display something.
pub struct Glindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: Display,
    pub context: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
}

impl Default for Glindow {
    fn default() -> Self {
        Self::new()
    }
}

impl Glindow {
    /// This function will cause your program to panic if something goes wrong.
    /// No error handling whatsoever.
    pub fn new() -> Self {
        // https://github.com/rust-windowing/glutin/blob/966cf95334adf1c70050bd36e5961872675cf915/glutin_examples/src/lib.rs
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new();
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (window, gl_config) = display_builder
            .build(&event_loop, template, |mut configs| {
                configs
                    .next()
                    .expect("Failed to pick config - out of None.")
            })
            .expect("Failed to create display.");
        let window = window.expect("Failed to obtain window.");
        let display = gl_config.display();
        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(None);
        let context = unsafe {
            display
                .create_context(&gl_config, &context_attributes)
                .expect("Failed to create context.")
        };
        let win_attrs = window.build_surface_attributes(Default::default());
        let surface = unsafe {
            display
                .create_window_surface(&gl_config, &win_attrs)
                .expect("Failed to create surface.")
        };
        let context = context
            .make_current(&surface)
            .expect("Failed to make current context.");

        #[cfg(debug_assertions)]
        {
            println!("GL-Winit setup.");
            println!("Window Scale Factor: {}", window.scale_factor());
            println!("Window Size: {:#?}", window.inner_size());
        }

        let result = Self {
            event_loop,
            window,
            display,
            context,
            surface,
        };

        result.load_gl();

        result
    }

    pub fn load_gl(&self) {
        #[cfg(debug_assertions)]
        println!("Loading GL!");

        gl::load_with(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            self.display.get_proc_address(symbol.as_c_str()).cast()
        });

        #[cfg(debug_assertions)]
        {
            print_gl("Version", gl::VERSION);
            print_gl("Renderer", gl::RENDERER);
            print_gl("Shading Language Version", gl::SHADING_LANGUAGE_VERSION);
        }
    }

    /// Swaps surface buffers and runs EventLoop with listener that does nothing except
    /// for WindowEvent::CloseRequested and WindowEvent::Resized events.
    ///
    /// Any drawing must happen before this function call.
    pub fn run_until_close(self) -> ! {
        #[allow(unused_variables)]
        let Self {
            window,
            surface,
            context,
            event_loop,
            display,
        } = self;

        if !surface.is_single_buffered() {
            surface.swap_buffers(&context).expect("I want to swap!");
        }

        event_loop.run(move |event, _target, control_flow| {
            // https://docs.rs/winit/latest/winit/index.html
            control_flow.set_wait();

            // Do nothing but close the window.
            #[allow(clippy::single_match)]
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::Resized(size) => {
                        #[cfg(debug_assertions)]
                        println!("New size: {:?}", size);

                        if size.width != 0 && size.height != 0 {
                            surface.resize(
                                &context,
                                size.width.try_into().unwrap(),
                                size.height.try_into().unwrap(),
                            );
                            unsafe {
                                gl::Viewport(
                                    0,
                                    0,
                                    size.width.try_into().unwrap(),
                                    size.height.try_into().unwrap(),
                                )
                            };
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

#[cfg(debug_assertions)]
fn print_gl(prefix: &str, about: gl::types::GLenum) {
    use std::ffi::CStr;

    unsafe {
        let cstr = gl::GetString(about);
        if !cstr.is_null() {
            println!(
                "{}: {}",
                prefix,
                CStr::from_ptr(cstr.cast()).to_string_lossy()
            );
        }
    }
}

#[macro_export]
macro_rules! opengl {
    ($($st:stmt;)+) => {
        $(
            opengl!($st);
        )+
    };

    ($st:stmt) => {
        unsafe {
            $crate::with_get_error(
                || { $st },
                stringify!($st),
                file!(),
            )
        }
    };
}

#[inline(always)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn with_get_error<R, F: FnOnce() -> R>(
    work: F,
    source: &'static str,
    file: &'static str,
) -> R {
    #[cfg(debug_assertions)]
    while gl::GetError() != gl::NO_ERROR {}

    let res = work();

    #[cfg(debug_assertions)]
    {
        let mut errored = false;
        let mut error: gl::types::GLenum = gl::GetError();
        while error != gl::NO_ERROR {
            errored = true;
            eprintln!("\x1b[91mCheck error: {}\x1b[0m", error);
            error = gl::GetError();
        }
        if errored {
            eprintln!("\x1b[91mAt: {}\x1b[0m\n{}", file, source);
            panic!("GL Assertion failed.");
        }
    }

    res
}

#[macro_export]
macro_rules! uniforms {
    ($name:ident; $($uniform:ident),+) => {
        struct $name {
            $(
                $uniform: i32,
            )+
        }

        impl $name {
            fn new(program: &$crate::Program) -> Self {
                let s = Self {
                    $(
                        $uniform: program.get_uniform(concat!(stringify!($uniform), "\0")),
                    )+
                };
                s.assert();
                s
            }

            fn assert(&self) {
                $(
                    if self.$uniform < 0 {
                        panic!("Uniform {} was not found in the shader", stringify!($uniform));
                    }
                )+
            }
        }
    }
}
