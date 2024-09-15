use std::{
    path::{Path, PathBuf},
    process,
};

use image::{buffer::ConvertBuffer, imageops::resize, RgbaImage};

#[derive(Copy, Clone, Debug)]
pub enum Blur {
    Box,
    Gaussian,
}

#[derive(Debug)]
pub struct Image {
    path: PathBuf,
    blur_type: Option<Blur>,
    blur_size: u32,
    pub buffer: Option<RgbaImage>,
}

impl Image {
    pub fn init(p: &Path, blur_type: String, blur_size: u32) -> Self {
        tracing::trace!("Actual Init");
        Image {
            path: p.to_path_buf(),
            //            buffer: None,
            blur_type: match blur_type.as_str() {
                "box" => Some(Blur::Box),
                "gaussian" => Some(Blur::Gaussian),
                _ => None,
            },
            blur_size,
            buffer: None,
        }
    }

    // pub fn blur(&mut self) {
    //     tracing::trace!("Starting blur");
    //     let mut binding = self.buffer.clone();
    //     let blurred = binding.as_mut().unwrap();
    //     match self.blur_type.clone() {
    //         Some(bt) => match bt {
    //             Blur::Box => {
    //                 box_blur(blurred);
    //             }
    //             Blur::Gaussian => {
    //                 gaussian_blur(blurred, self.blur_size as i32);
    //             }
    //         },
    //         None => {}
    //     };
    //     self.buffer = Some(blurred.clone());
    // }

    pub fn resize(&mut self, width: u32, height: u32) {
        tracing::trace!("Starting resize");
        let mut resize_needed: bool = false;
        let new_buffer = match self.buffer.clone() {
            Some(buf) => {
                tracing::trace!("Buffer found");
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
            }
            None => {
                tracing::trace!("Buffer not found");
                resize_needed = true;
                match image::open(self.path.clone()) {
                    Ok(image) => {
                        if let Some(r) = image.as_rgba8() {
                            r.clone()
                        } else if let Some(r) = image.as_rgb8() {
                            let r2: RgbaImage = r.convert();
                            r2
                        } else {
                            tracing::error!("Unable to convert to rgba8");
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        // Some("screenshot") => {
                        //     let conn = libwayshot::WayshotConnection::new().unwrap();
                        //     let im = conn
                        //         .screenshot(
                        //             libwayshot::CaptureRegion {
                        //                 x_coordinate: 0,
                        //                 y_coordinate: 0,
                        //                 width: width as i32,
                        //                 height: height as i32,
                        //             },
                        //             false,
                        //         )
                        //         .unwrap();
                        //     PhotonImage::new(im.clone().into_vec(), im.width(), im.height())
                        // }

                        tracing::error!("Unable to open given path, {}", e);
                        process::exit(1);
                    }
                }
            }
        };
        tracing::trace!(
            "Final buffer Size ({}, {})",
            new_buffer.width(),
            new_buffer.height()
        );
        self.buffer = Some(new_buffer);
        if resize_needed {
            self.resize(width, height);
        }
    }
}
