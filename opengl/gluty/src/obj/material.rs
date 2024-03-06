use super::Obj;
use crate::asset::Asset;
use crate::Texture;

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
pub fn load_mtl<T>(mtl: &Asset<T>, resources: &[Asset<T>]) -> Material
where
    T: AsRef<[u8]>,
{
    let mut material = Material::default();

    let file = String::from_utf8_lossy(mtl.value.as_ref());

    let find_asset = |suffix: &str| -> Option<&Asset<T>> {
        resources.iter().find(|it| it.path.ends_with(suffix))
    };

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
            if let Some(asset) = find_asset(suffix) {
                if let Ok(img) = asset.try_to_img() {
                    material.ambient_texture =
                        Some(Texture::from_image(gl::TEXTURE_2D, 0, &img, gl::RGBA));
                }
            }
        } else if let Some(suffix) = line.strip_prefix("map_Kd ") {
            if let Some(asset) = find_asset(suffix) {
                if let Ok(img) = asset.try_to_img() {
                    material.diffuse_texture =
                        Some(Texture::from_image(gl::TEXTURE_2D, 1, &img, gl::RGBA));
                }
            }
        } else if let Some(suffix) = line.strip_prefix("map_Ks ") {
            if let Some(asset) = find_asset(suffix) {
                if let Ok(img) = asset.try_to_img() {
                    material.specular_texture =
                        Some(Texture::from_image(gl::TEXTURE_2D, 2, &img, gl::RGBA));
                }
            }
        } else if let Some(suffix) = line.strip_prefix("newmtl ") {
            material.name = suffix.to_owned();
        }
    }

    material
}
