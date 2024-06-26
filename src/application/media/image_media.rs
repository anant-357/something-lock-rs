use std::{path::PathBuf, process};

use photon_rs::{
    conv::{box_blur, gaussian_blur},
    transform, PhotonImage,
};
use tracing::error;

pub struct Image {
    pub buffer: PhotonImage,
}

impl Image {
    pub fn from_path(p: &PathBuf, blur_type: String, blur_size: u32) -> Self {
        let mut image = match photon_rs::native::open_image(p.to_str().unwrap()) {
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
                    PhotonImage::new(im.clone().into_vec(), im.width(), im.height())
                }
                _ => {
                    error!("Unable to open given path, {}", e);
                    process::exit(1);
                }
            },
        };

        tracing::info!("starting resize");
        image = transform::resize(&image, 1920, 1080, transform::SamplingFilter::Nearest);
        tracing::info!("ending resize");
        tracing::info!("starting blur");
        match blur_type.as_str() {
            "box" => box_blur(&mut image),
            "gaussian" => gaussian_blur(&mut image, blur_size as i32),
            _ => (),
        }
        tracing::info!("ending blur");
        Image { buffer: image }
    }

    pub fn set_buffer(&mut self, buf: PhotonImage) {
        self.buffer = buf;
    }
}
