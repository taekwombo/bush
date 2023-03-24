use super::super::Texture;
use super::Obj;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub specular_component: f32,
    pub ambient_texture: Option<Texture>,
    pub diffuse_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
}

impl Default for Material {
    fn default() -> Self {
        let white = [0.0; 3];

        Self {
            name: String::from(""),
            ambient_color: white,
            diffuse_color: white,
            specular_color: white,
            specular_component: 10.0,
            ambient_texture: None,
            diffuse_texture: None,
            specular_texture: None,
        }
    }
}

/// Load first material from .mtl file.
pub fn load_mtl(path: PathBuf) -> Material {
    use std::fs::read;

    let mut material = Material::default();

    let file = read(&path).expect("MTL file must exist.");
    let file = String::from_utf8_lossy(&file);

    let mut tex_path = path;

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
            tex_path.set_file_name(suffix);
            if let Ok(tex) = Texture::create_from_file(&tex_path, gl::TEXTURE_2D, 0) {
                material.ambient_texture = Some(tex);
            }
        } else if let Some(suffix) = line.strip_prefix("map_Kd ") {
            tex_path.set_file_name(suffix);
            if let Ok(tex) = Texture::create_from_file(&tex_path, gl::TEXTURE_2D, 1) {
                material.diffuse_texture = Some(tex);
            }
        } else if let Some(suffix) = line.strip_prefix("map_Ks ") {
            tex_path.set_file_name(suffix);
            if let Ok(tex) = Texture::create_from_file(&tex_path, gl::TEXTURE_2D, 2) {
                material.specular_texture = Some(tex);
            }
        } else if let Some(suffix) = line.strip_prefix("newmtl ") {
            material.name = suffix.to_owned();
        }
    }

    material
}
