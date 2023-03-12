use glam::Mat4;

pub enum ProjectionKind {
    Orthographic([f32; 6]),
    Perspective([f32; 4]),
}

impl ProjectionKind {
    fn to_matrix(&self) -> Mat4 {
        match self {
            Self::Orthographic([l, r, b, t, n, f]) => Mat4::orthographic_rh_gl(
                *l, *r, *b, *t, *n, *f
            ),
            Self::Perspective([fov, aspect, n, f]) => Mat4::perspective_rh_gl(
                *fov, *aspect, *n, *f
            ),
        }
    }
}

pub struct Projection {
    kind: ProjectionKind,
    pub matrix: Mat4,
}

impl Projection {
    pub fn is_orthographic(&self) -> bool {
        matches!(self.kind, ProjectionKind::Orthographic(_))
    }
    pub fn orthographic(bounds: [f32; 6]) -> Self {
        let kind = ProjectionKind::Orthographic(bounds);
        let matrix = kind.to_matrix();

        Self { kind, matrix }
    }

    pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        let kind = ProjectionKind::Perspective([fov_deg.to_radians(), aspect, near, far]);
        let matrix = kind.to_matrix();

        Self { kind, matrix }
    }

    pub fn resize(&mut self, aspect: f32) -> &mut Self {
        if let ProjectionKind::Perspective(v) = &mut self.kind {
            v[1] = aspect;
            self.matrix = self.kind.to_matrix();
        }

        self
    }

    pub fn replace(&mut self, proj: Projection) -> &mut Self {
        self.kind = proj.kind;
        self.matrix = proj.matrix;
        self
    }
}
