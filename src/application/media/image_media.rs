use std::{path::PathBuf, process};

use image::DynamicImage;
use tracing::error;

pub struct Image {
    pub base: image::DynamicImage,
    pub buffer: image::RgbaImage,
}

impl Image {
    pub fn from_path(p: &PathBuf) -> Self {
        let image = match image::open(p.as_path()) {
            Ok(image) => image,
            Err(e) => match p.to_str() {
                Some("screenshot") => DynamicImage::new(100, 100, image::ColorType::Rgba8),
                _ => {
                    error!("Unable to open given path, {}", e);
                    process::exit(1);
                }
            },
        };

        let image_buffer =
            image::imageops::resize(&image, 1920, 1080, image::imageops::FilterType::Nearest);
        Image {
            base: image,
            buffer: image_buffer,
        }
    }

    pub fn set_buffer(&mut self, buf: image::RgbaImage) {
        self.buffer = buf;
    }
}
