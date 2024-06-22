use std::path::PathBuf;

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
