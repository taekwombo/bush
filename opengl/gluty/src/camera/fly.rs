use super::Projection;
use glam::{Mat4, Vec3};

/// Follows https://learnopengl.com/Getting-started/Camera
pub struct FlyCamera {
    /// Position of the camera in world space.
    position: Vec3,
    /// Vector opposite to the Z+ of the camera coordinate space.
    pub front: Vec3,
    /// X vector of camera coordinate space.
    pub right: Vec3,
    /// Rotation along Y axis in degrees.
    /// Looking left or right.
    yaw: f32,
    /// Rotation along X axis in degrees.
    /// Looking up or down.
    pitch: f32,
    /// Computed view matrix (already inversed).
    view_matrix: Mat4,
    // Camera velocity while moving. Units per second.
    velocity: f32,
    pub projection: Projection,
}

impl FlyCamera {
    pub fn new<F>(proj_create: F) -> Self
    where
        F: FnOnce() -> Projection,
    {
        let position = Vec3::ZERO;
        let front = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::Y;
        let view_matrix = Mat4::look_at_rh(position, front, up);

        Self {
            position,
            front,
            right: Vec3::X,
            view_matrix,
            yaw: -90.0,
            pitch: 0.0,
            projection: proj_create(),
            velocity: 5.0,
        }
    }

    pub fn accelerate_x(&mut self, change: f32) -> &mut Self {
        self.position += self.right * (change * self.velocity);
        self
    }

    pub fn accelerate_z(&mut self, change: f32) -> &mut Self {
        self.position += self.front * (change * self.velocity);
        self
    }

    pub fn goto(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
        self
    }

    pub fn rotate(&mut self, x_deg: f32, y_deg: f32) -> &mut Self {
        self.pitch += x_deg;
        self.yaw += y_deg;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.pitch = self.pitch.clamp(-89.0, 89.0);
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        self.front = Vec3::new(
            f32::cos(yaw) * f32::cos(pitch),
            f32::sin(pitch),
            f32::sin(yaw) * f32::cos(pitch),
        )
        .normalize();

        self.right = self.front.cross(Vec3::Y);
        // Just make sure up vector actually points up
        // when calling this function for the first time.
        let up = self.right.cross(self.front);

        // look_at_rh(
        //   position (looking from this position),
        //   center (position of thing camera is looking at),
        //   up
        // )
        // look_to_rh(
        //   position (looking from this position),
        //   front    (looking in this direction),
        //   up
        // )

        // While normally view transform is: Translation * Rotation and
        // it needs to be inversed to apply to vertex position.
        // This one is already inversed and ready to apply to vertex position.
        self.view_matrix = Mat4::look_to_rh(self.position, self.front, up);

        self
    }

    pub fn get_view(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn get_proj(&self) -> &Mat4 {
        &self.projection.matrix
    }

    pub fn get_view_proj(&self) -> Mat4 {
        self.projection.matrix * self.view_matrix
    }
}
