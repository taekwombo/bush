//! Creates window, displays green+blue (aqua?) screen.

use gluty::Glindow;

fn main() {
    let glin = Glindow::new();

    unsafe {
        gl::ClearColor(0.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    glin.run_until_close();
}
