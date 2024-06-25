use std::{path::PathBuf, process};

use fast_image_resize::{IntoImageView, PixelType, ResizeOptions, Resizer};
use image::{imageops::blur, DynamicImage, RgbaImage};
use tracing::error;

pub struct Image {
    pub buffer: image::DynamicImage,
    pub pixel_type: PixelType,
}

impl Image {
    pub fn from_path(p: &PathBuf, blur_size: u32) -> Self {
        let image = match image::open(p.as_path()) {
            Ok(image) => image,
            Err(e) => match p.to_str() {
                Some("screenshot") => {
                    let conn = libwayshot::WayshotConnection::new().unwrap();
                    let im = conn
                        .screenshot(
                            libwayshot::CaptureRegion {
                                x_coordinate: 0,
                                y_coordinate: 0,
                                width: 1920,
                                height: 1080,
                            },
                            false,
                        )
                        .unwrap();
                    let rim = RgbaImage::from_vec(im.width(), im.height(), im.into_vec()).unwrap();
                    DynamicImage::from(rim)
                }
                _ => {
                    error!("Unable to open given path, {}", e);
                    process::exit(1);
                }
            },
        };

        let mut new =
            fast_image_resize::images::Image::new(1920, 1080, image.pixel_type().unwrap());
        Resizer::new()
            .resize(
                &image,
                &mut new,
                &ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Nearest),
            )
            .unwrap();
        let image_buffer = RgbaImage::from_raw(1920, 1080, new.into_vec()).unwrap();
        let rim = blur(&image_buffer, blur_size as f32);
        Image {
            buffer: DynamicImage::from(rim),
            pixel_type: image.pixel_type().unwrap(),
        }
    }

    pub fn set_buffer(&mut self, buf: DynamicImage) {
        self.buffer = buf;
    }
}
