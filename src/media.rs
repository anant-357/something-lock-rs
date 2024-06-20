use std::path::PathBuf;

use gstreamer::Pipeline;

use crate::conf::{Config, MediaType as CMediaType};

pub struct Image {
    pub base: image::DynamicImage,
    pub buffer: image::RgbaImage,
}

impl Image {
    pub fn from_path(p: &PathBuf) -> Self {
        let image = image::open(p.as_path()).expect("Unable to open image!");

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

pub struct Video {
    base: Pipeline,
}

pub enum MediaType {
    Image(Image),
    Video(Video),
    NA,
}

pub struct Media {
    pub base: MediaType,
}

impl Media {
    pub fn from_config(config: &Config) -> Self {
        match config.media_type {
            CMediaType::Image => Media {
                base: MediaType::Image(Image::from_path(&config.media_path)),
            },
            _ => Media {
                base: MediaType::NA,
            },
        }
    }
}
