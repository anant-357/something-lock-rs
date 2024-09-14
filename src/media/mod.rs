mod image_media;
mod video_media;

use std::{path::PathBuf, str::FromStr};

use crate::config::{Config, MediaType as CMediaType};
use image_media::Image;
use wgpu::Color;
use video_media::Video;

pub enum Media {
    Image(Image),
    Solid(Color),
    _Video(Video),
    Shader(PathBuf),
    NA,
}

impl Media {
    pub fn from_config(config: &Config) -> Self {
        match config.media_type {
            Some(CMediaType::Image) => {
                tracing::trace!("Init Image");
                Media::Image(Image::init(
                config.media_path.as_ref().unwrap(),
                config.blur_type.clone().unwrap_or("".to_string()),
                config.blur_size.unwrap_or(0),
            ))
            },
            Some(CMediaType::Solid) => Media::Solid(config.color.unwrap()),
            Some(CMediaType::Shader) => Media::Shader(PathBuf::from_str(config.media_path.as_ref().unwrap().to_str().unwrap()).unwrap()),
            _ => Media::NA,
        }
    }
}
