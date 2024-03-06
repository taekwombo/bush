pub use macros::asset_path;
use std::fmt;

/// Helper macro used for asset loading.
///
/// ### Examples
/// Loading single file:
///
/// ```no_run
/// use gluty::assets;
///
/// let asset = assets!("./path/to/asset.jpg");
/// ```
///
/// Loading multiple files into tuple:
///
/// ```no_run
/// use gluty::assets;
///
/// let (texture, shader) = assets!(
///     "./path/to/texture.ext",
///     "./path/to/shader.ext",
/// );
/// ```
///
/// Loading multiple files into array:
/// ```no_run
/// use gluty::assets;
///
/// let [texture, shader] = assets!([
///     "./path/to/texture.ext",
///     "./path/to/shader.ext",
/// ]);
/// ```
///
/// Loading multiple files into struct:
///
/// ```no_run
/// use gluty::assets;
///
/// let assets = assets!({
///     texture: "./path/to/texture.ext",
///     vertex: "./path/to/shader.vert",
///     fragment: "./path/to/shader.frag",
/// });
/// ```
#[macro_export]
macro_rules! assets {
    ($path:literal) => {{
        #[cfg(test)] {
            $crate::asset::Asset {
                path: $path,
                value: &[],
            }
        }
        #[cfg(not(test))] {
            #[cfg(debug_assertions)] {
                ::std::println!(
                    "Loading asset: {} -- {}", 
                    $path,
                    $crate::asset::asset_path!($path),
                );
                let path = $crate::asset::asset_path!($path);
                $crate::asset::Asset::new(
                    path,
                    ::std::fs::read(path).expect("Asset not found"),
                )
            }
            #[cfg(not(debug_assertions))] {
                $crate::asset::Asset::new(
                    $path,
                    include_bytes!($crate::asset::asset_path!($path)) as &[u8],
                )
            }
        }
    }};

    ($($path:literal),*$(,)?) => {{
        ($($crate::assets!($path)),*)
    }};

    ([ $($path:literal),*$(,)? ]) => {{
        [$(
            $crate::assets!($path),
        )*]
    }};

    ({ $($name:ident: $path:literal),*$(,)? }) => {{
        struct __Assets {
            $($name: $crate::asset::Asset),*
        }

        impl __Assets {
            pub fn reload(&mut self) {
                $(
                    self.$name.reload();
                )*
            }
        }

        __Assets {
            $($name: $crate::assets!($path),)*
        }
    }};
}

pub struct Asset<T> {
    pub path: String,
    pub value: T,
}

impl<T: AsRef<[u8]>> Asset<T> {
    pub fn get(&self) -> &[u8] {
        self.value.as_ref()
    }

    pub fn new(path: &str, value: T) -> Self {
        Self {
            path: path.to_owned(),
            value,
        }
    }

    pub fn try_to_img(&self) -> image::ImageResult<image::DynamicImage> {
        use image::codecs::jpeg::JpegDecoder;
        use image::codecs::png::PngDecoder;
        use image::{DynamicImage, ImageFormat};
        use std::io::BufReader;

        let reader = BufReader::new(self.value.as_ref());
        match image::guess_format(self.value.as_ref())? {
            ImageFormat::Jpeg => JpegDecoder::new(reader).and_then(DynamicImage::from_decoder),
            ImageFormat::Png => PngDecoder::new(reader).and_then(DynamicImage::from_decoder),
            _ => unimplemented!(),
        }
    }
}

impl Asset<Vec<u8>> {
    pub fn reload(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.value = ::std::fs::read(&self.path).unwrap();
        }
    }

    pub fn from_file(path: &str) -> Self {
        use ::std::fs;

        let Ok(file) = fs::read(path) else {
            panic!("Failed to read asset file {:?}", path);
        };

        Self {
            path: path.to_owned(),
            value: file,
        }
    }
}

impl<T> fmt::Debug for Asset<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Asset").field("path", &self.path).finish()
    }
}
