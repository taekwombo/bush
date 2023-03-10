//! https://graphics.cs.utah.edu/courses/cs6610/spring2021/?prj=2

use gluty::{gl, Glindow, opengl};
use ig::*;

fn main () {
    let glin = Glindow::new();
    let mut project = Project::new(glin.window.inner_size());

    project.camera.goto(0.0, 0.0, 60.0).update();
    project.prog.use_program();
    project
        .add_uniform("u_model_t\0", Box::new(|proj| {
            proj.mesh.model_to_world
        }))
        .add_uniform("u_view_t\0", Box::new(|proj| {
            proj.camera.get_view()
        }))
        .add_uniform("u_proj_t\0", Box::new(|proj| {
            proj.camera.get_proj()
        }))
        .upload_uniforms();


    opengl! {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }

    #[allow(unused_variables)]
    let Glindow { window, event_loop, display, surface, context } = glin;

    event_loop.run(move |event, _, control_flow| {
        use gluty::winit::event::{Event, WindowEvent};
        use gluty::glutin::prelude::*;

        control_flow.set_wait();

        match event {
            Event::RedrawRequested(_) => {
                opengl! {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                project.mesh.draw();
                surface.swap_buffers(&context).expect("I want to swap!");
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, is_synthetic: false, .. } => {
                    if let Some(kc) = input.virtual_keycode {
                        project.handle_key_code(&input.state, &kc);
                        window.request_redraw();
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    project.handle_mouse_btn(&state, &button);
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if project.handle_cursor_move(&position) {
                        window.request_redraw();
                    }
                },
                _ => (),
            },
            _ => (),
        };
    });
}
