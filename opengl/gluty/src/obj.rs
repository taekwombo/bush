use std::collections::{hash_map::Entry, HashMap};

mod material;

pub use material::*;

fn to_isize_expect(value: Option<&str>) -> isize {
    match value {
        None => -1,
        Some(s) if s.is_empty() => -1,
        Some(s) => s.parse().expect("Face index to be isize."),
    }
}

fn to_u32_expect(value: isize) -> u32 {
    u32::try_from(value).expect("Value must fit into u32.")
}

#[derive(Clone, Copy)]
struct Vertex {
    position: isize,
    texture: isize,
    normal: isize,
}

impl Vertex {
    #[inline]
    fn as_key(&self, tex: bool) -> (u32, u32, u32) {
        let p = to_u32_expect(self.position);
        let n = to_u32_expect(self.normal);
        let t = if tex { to_u32_expect(self.texture) } else { 0 };

        (p, n, t)
    }
}

#[derive(Default)]
pub struct BuildOptions {
    tex: bool,
}

impl BuildOptions {
    pub fn with_tex() -> Self {
        Self { tex: true }
    }
}

pub struct Obj {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    texture_coords: Vec<[f32; 3]>,
    triangles: Vec<[Vertex; 3]>,
    pub material: Material,
}

impl Default for Obj {
    fn default() -> Self {
        Self::new()
    }
}

impl Obj {
    pub fn new() -> Self {
        Self {
            positions: Default::default(),
            normals: Default::default(),
            texture_coords: Default::default(),
            triangles: Default::default(),
            material: Material::default(),
        }
    }

    pub fn load_vvn(path: &str) -> (Vec<f32>, Vec<u32>) {
        Obj::new().parse(path).build(Default::default())
    }

    pub fn build(&self, opt: BuildOptions) -> (Vec<f32>, Vec<u32>) {
        let vertex_size = if opt.tex { 9 } else { 6 };

        let mut vbo_data = Vec::new();
        let mut ebo_data = Vec::new();
        // Store position-normal pairs for re-use.
        let mut vertex_map: HashMap<(u32, u32, u32), u32> = HashMap::new();
        let mut vertex_index: u32 = 0;

        for triangle in &self.triangles {
            for vertex in triangle {
                let key = vertex.as_key(opt.tex);

                if let Entry::Vacant(entry) = vertex_map.entry(key) {
                    debug_assert!(vertex_index as usize == (vbo_data.len() / vertex_size));
                    entry.insert(vertex_index);
                    // Push position data.
                    vbo_data.extend_from_slice(&self.positions[key.0 as usize]);
                    // Push normal data.
                    vbo_data.extend_from_slice(&self.normals[key.1 as usize]);
                    // Push texture data.
                    if opt.tex {
                        vbo_data.extend_from_slice(&self.texture_coords[key.2 as usize]);
                    }
                    ebo_data.push(vertex_index);
                    vertex_index += 1;
                } else {
                    let index = vertex_map.get(&key).unwrap();
                    ebo_data.push(*index);
                }
            }
        }

        (vbo_data, ebo_data)
    }

    pub fn parse(&mut self, path: &str) -> &mut Self {
        use std::fs::read;
        use std::path::PathBuf;

        let file = read(path).expect("Model file must exist.");
        let file = String::from_utf8_lossy(&file);

        for line in file.lines() {
            if let Some(suffix) = line.strip_prefix("v ") {
                self.parse_vertex_position(suffix);
            } else if let Some(suffix) = line.strip_prefix("vn ") {
                self.parse_vertex_normal(suffix);
            } else if let Some(suffix) = line.strip_prefix("vt ") {
                self.parse_vertex_texture_coord(suffix);
            } else if let Some(suffix) = line.strip_prefix("f ") {
                self.parse_face(suffix);
            } else if let Some(suffix) = line.strip_prefix("mtllib ") {
                let mut path = PathBuf::from(path);
                path.pop();
                path.push(suffix);

                if path.is_file() {
                    self.material = load_mtl(path);
                }
            }
        }

        self
    }

    fn parse_vertex_position(&mut self, line: &str) {
        self.positions.push(Obj::parse_vector(line, false));
    }

    fn parse_vertex_normal(&mut self, line: &str) {
        self.normals.push(Obj::parse_vector(line, false));
    }

    fn parse_vertex_texture_coord(&mut self, line: &str) {
        self.texture_coords.push(Obj::parse_vector(line, true));
    }

    fn parse_face(&mut self, line: &str) {
        let mut vertices: [Vertex; 4] = [Vertex {
            position: -1,
            normal: -1,
            texture: -1,
        }; 4];
        let count = Obj::parse_face_line(line, &mut vertices);

        self.triangles.push([vertices[0], vertices[1], vertices[2]]);

        if count == 4 {
            self.triangles.push([vertices[2], vertices[3], vertices[0]]);
        }
    }

    /// Expects: "<f32> <f32> <f32>\n" string slice.
    /// Parses floats and appends them to the vec parameter.
    fn parse_vector(line: &str, third_optional: bool) -> [f32; 3] {
        let mut split = line.split(' ').filter(|v| !v.is_empty());
        let x = split
            .next()
            .expect("Component exists.")
            .parse::<f32>()
            .expect("Component value is f32.");
        let y = split
            .next()
            .expect("Component exists.")
            .parse::<f32>()
            .expect("Component value is f32.");
        let z = split
            .next()
            .map(|v| v.parse::<f32>().expect("Component value is f32."));

        [
            x,
            y,
            if third_optional {
                z.unwrap_or(0.0)
            } else {
                z.expect("Component exists.")
            },
        ]
    }

    /// Expects (<isize>/<isize>?/<isize>){3,4}
    fn parse_face_line(line: &str, vertices: &mut [Vertex; 4]) -> usize {
        let mut vertex_count = 0;

        for chunk in line.split(' ').filter(|v| !v.is_empty()) {
            debug_assert!(vertex_count <= 4);

            let mut split = chunk.split('/');
            let position = to_isize_expect(split.next()) - 1;
            let texture = to_isize_expect(split.next()) - 1;
            let normal = to_isize_expect(split.next()) - 1;

            // Require at least vertex position and vertex normal values.
            debug_assert!(position >= 0);
            debug_assert!(normal >= 0);

            vertices[vertex_count] = Vertex {
                position,
                texture,
                normal,
            };
            vertex_count += 1;
        }

        vertex_count
    }
}
