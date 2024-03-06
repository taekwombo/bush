use crate::asset::Asset;
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

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: isize,
    texture: isize,
    normal: isize,
}

impl Vertex {
    #[inline]
    fn as_key(&self) -> (u32, u32, u32) {
        let p = to_u32_expect(self.position);
        let n = if self.normal < 0 {
            0
        } else {
            to_u32_expect(self.normal)
        };
        let t = if self.texture < 0 {
            0
        } else {
            to_u32_expect(self.texture)
        };

        (p, n, t)
    }
}

/// Describes required attributes.
/// Vertex position (required)
/// Vertex normal (optional, enabled by default)
/// Texture coordinate (optional)
pub struct BuildOptions {
    normal: bool,
    tex: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            normal: true,
            tex: false,
        }
    }
}

impl BuildOptions {
    pub fn vertices_only() -> Self {
        Self {
            normal: false,
            tex: false,
        }
    }

    pub fn with_tex() -> Self {
        Self {
            normal: true,
            tex: true,
        }
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

    pub fn cmp_opts(&self, opts: &BuildOptions) -> bool {
        let normal = if opts.normal {
            !self.normals.is_empty()
        } else {
            true
        };
        let tex = if opts.tex {
            !self.texture_coords.is_empty()
        } else {
            true
        };

        normal && tex
    }

    pub fn build(&self, opt: &BuildOptions) -> (Vec<f32>, Vec<u32>) {
        let has_normal = opt.normal;
        let has_texture = opt.tex;
        #[cfg(debug_assertions)]
        {
            if has_normal {
                assert!(!self.normals.is_empty());
            }
            if has_texture {
                assert!(!self.texture_coords.is_empty());
            }
        }
        let vertex_size = match (has_normal, has_texture) {
            (true, true) => 9,
            (true, false) | (false, true) => 6,
            (false, false) => 3,
        };

        let mut vbo_data = Vec::new();
        let mut ebo_data = Vec::new();
        // Store position-normal pairs for re-use.
        let mut vertex_map: HashMap<(u32, u32, u32), u32> = HashMap::new();
        let mut vertex_index: u32 = 0;

        for triangle in &self.triangles {
            for vertex in triangle {
                let key = vertex.as_key();

                if let Entry::Vacant(entry) = vertex_map.entry(key) {
                    debug_assert!(vertex_index as usize == (vbo_data.len() / vertex_size));
                    entry.insert(vertex_index);
                    // Push position data.
                    vbo_data.extend_from_slice(&self.positions[key.0 as usize]);
                    // Push normal data.
                    if has_normal {
                        vbo_data.extend_from_slice(&self.normals[key.1 as usize]);
                    }
                    // Push texture data.
                    if has_texture {
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

    pub fn parse_mtl<T>(&mut self, mtl: &Asset<T>, resources: &[Asset<T>]) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.material = load_mtl(mtl, resources);
        self
    }

    pub fn parse_obj<T>(&mut self, asset: &Asset<T>) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        let file = std::str::from_utf8(asset.value.as_ref()).expect("Asset must be utf-8 encoded");

        for line in file.lines() {
            if let Some(suffix) = line.strip_prefix("v ") {
                self.parse_vertex_position(suffix);
            } else if let Some(suffix) = line.strip_prefix("vn ") {
                self.parse_vertex_normal(suffix);
            } else if let Some(suffix) = line.strip_prefix("vt ") {
                self.parse_vertex_texture_coord(suffix);
            } else if let Some(suffix) = line.strip_prefix("f ") {
                self.parse_face(suffix);
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

            // Require vertex positions.
            debug_assert!(position >= 0);

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
