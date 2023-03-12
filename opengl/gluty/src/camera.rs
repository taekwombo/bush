use glam::{Mat4, Vec3};

mod fly;
mod projection;

pub use fly::FlyCamera;
pub use projection::Projection;

/// Basic camera, doesn't follow any particular implementation.
/// Somehow works, then it's good enough.
pub struct Camera {
    /// Defines position of the camera in the world coordinate space.
    /// Contains only scale and translation transformations.
    camera_to_world: Mat4,
    /// X and Y axis rotation.
    rotation: (f32, f32),
    /// Projection matrix.
    projection: Mat4,
    /// FOV for projection matrix.
    fov: f32,
    /// Near plane for projection matrix.
    near: f32,
    /// Far plane for projection matrix.
    far: f32,
}

impl Camera {
    pub fn new(fov: f32, width: u32, height: u32) -> Self {
        Self {
            camera_to_world: Mat4::IDENTITY,
            rotation: (0.0, 0.0),
            fov,
            near: 0.1,
            far: 100.0,
            projection: Mat4::perspective_rh_gl(
                fov.to_radians(),
                (width as i32 as f32) / (height as i32 as f32),
                0.1,
                100.0,
            ),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.camera_to_world *= Mat4::from_translation(Vec3::new(x, y, z));
        self
    }

    pub fn rotate_x(&mut self, x: f32) -> &mut Self {
        self.rotation.0 += x;
        self
    }

    pub fn rotate_y(&mut self, y: f32) -> &mut Self {
        self.rotation.1 += y;
        self
    }

    pub fn resized(&mut self, ratio: f32) -> &mut Self {
        self.projection =
            Mat4::perspective_rh_gl(self.fov.to_radians(), ratio, self.near, self.far);
        self
    }

    pub fn get_proj(&self) -> Mat4 {
        // View-Projection matrix = Projection * Inverse(Camera_World).
        let rotation =
            Mat4::from_rotation_x(self.rotation.0) * Mat4::from_rotation_y(self.rotation.1);
        self.projection * (self.camera_to_world * rotation).inverse()
    }
}
