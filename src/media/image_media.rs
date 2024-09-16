use std::{path::Path, process};

use image::{imageops::resize, RgbaImage};

#[derive(Debug)]
pub struct Image {
    pub blur: u32,
    pub buffer: RgbaImage,
}

impl Image {
    pub fn init(p: &Path, blur: u32) -> Self {
        Image {
            blur,
            buffer: match image::open(p) {
                Ok(image) => {
                    image.to_rgba8()
                    // if let Some(r) = image.as_rgba8() {
                    //     r.clone()
                    // } else if let Some(r) = image.as_rgb8() {
                    //     let r2: RgbaImage = r.convert();
                    //     r2
                    // } else {
                    //     tracing::error!("Unable to convert to rgba8");
                    //     process::exit(1);
                    // }
                }
                Err(e) => {
                    tracing::error!("Unable to open given path, {}", e);
                    process::exit(1);
                }
            },
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let new_buffer = {
            let buf = self.buffer.clone();
            if width != buf.width() || height != buf.height() {
                tracing::trace!(
                    "Resize Needed, initial size: ({}, {}), needed: ({}, {})",
                    buf.width(),
                    buf.height(),
                    width,
                    height
                );
                let new_buf = resize(&buf, width, height, image::imageops::FilterType::Nearest);
                tracing::trace!("Resized to : {}, {}", new_buf.width(), new_buf.height());
                new_buf
            } else {
                tracing::trace!("Resize Not Needed");
                buf
            }
        };
        tracing::trace!(
            "Final buffer Size ({}, {})",
            new_buffer.width(),
            new_buffer.height()
        );
        self.buffer = new_buffer;
    }
}
