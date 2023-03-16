use super::Obj;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub specular_component: f32,
    pub ambient_texture: String,
    pub diffuse_texture: String,
    pub specular_texture: String,
}

impl Default for Material {
    fn default() -> Self {
        let empty = String::from("");
        let white = [0.0; 3];

        Self {
            name: empty.clone(),
            ambient_color: white,
            diffuse_color: white,
            specular_color: white,
            specular_component: 10.0,
            ambient_texture: empty.clone(),
            diffuse_texture: empty.clone(),
            specular_texture: empty,
        }
    }
}

/// Load first material from .mtl file.
pub fn load_mtl(path: PathBuf) -> Material {
    use std::fs::read;

    let mut material = Material::default();

    let file = read(path).expect("MTL file must exist.");
    let file = String::from_utf8_lossy(&file);

    for line in file.lines().filter(|v| !v.is_empty()) {
        let line = line.trim_start();

        if let Some(suffix) = line.strip_prefix("Ka ") {
            material.ambient_color = Obj::parse_vector(suffix, false);
        } else if let Some(suffix) = line.strip_prefix("Kd ") {
            material.diffuse_color = Obj::parse_vector(suffix, false);
        } else if let Some(suffix) = line.strip_prefix("Ks ") {
            material.specular_color = Obj::parse_vector(suffix, false);
        } else if let Some(suffix) = line.strip_prefix("Ns ") {
            material.specular_component = suffix.parse().expect("Ns must be a float.");
        } else if let Some(suffix) = line.strip_prefix("map_Ka ") {
            material.ambient_texture = suffix.to_owned();
        } else if let Some(suffix) = line.strip_prefix("map_Kd ") {
            material.diffuse_texture = suffix.to_owned();
        } else if let Some(suffix) = line.strip_prefix("map_Ks ") {
            material.specular_texture = suffix.to_owned();
        } else if let Some(suffix) = line.strip_prefix("newmtl ") {
            material.name = suffix.to_owned();
        }
    }

    material
}
