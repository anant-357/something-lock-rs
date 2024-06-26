use std::path::PathBuf;

use ini::Ini;
use tiny_skia::ColorU8;

pub enum MediaType {
    Animation,
    Image,
    Solid,
    Video,
}

pub struct Config {
    pub media_type: Option<MediaType>,
    pub media_path: Option<PathBuf>,
    pub color: Option<ColorU8>,
    pub blur_size: Option<u32>,
    pub blur_type: Option<String>,
}
impl Config {
    pub fn empty() -> Self {
        Config {
            media_type: None,
            media_path: None,
            color: None,
            blur_size: None,
            blur_type: None,
        }
    }
    pub fn from_config_file() -> Self {
        let mut config = Config::empty();
        let conf = Ini::load_from_file("conf.ini").unwrap();
        let main_section = conf.section(Some("main")).unwrap();
        match main_section.get("type") {
            Some(s) => match s {
                "image" => {
                    let image_section = conf.section(Some("image")).unwrap();
                    config.media_type = Some(MediaType::Image);
                    config.media_path = Some(PathBuf::from(image_section.get("path").unwrap()));
                    config.blur_size = Some(
                        image_section
                            .get("blur_size")
                            .unwrap_or("0")
                            .parse::<u32>()
                            .unwrap_or(0),
                    );

                    config.blur_type =
                        Some(image_section.get("blur_type").unwrap_or("box").to_string());
                }
                "solid" => {
                    let solid_section = conf.section(Some("solid")).unwrap();
                    config.media_type = Some(MediaType::Solid);
                    let red = solid_section.get("red").unwrap().parse::<u8>().unwrap();
                    let green = solid_section.get("green").unwrap().parse::<u8>().unwrap();
                    let blue = solid_section.get("blue").unwrap().parse::<u8>().unwrap();
                    let alpha = solid_section.get("alpha").unwrap().parse::<u8>().unwrap();
                    tracing::trace!(
                        "Solid Type: Color is (rgba) ({}, {}, {}, {})",
                        red,
                        green,
                        blue,
                        alpha
                    );
                    config.color = Some(ColorU8::from_rgba(red, green, blue, alpha));
                }
                "video" => {}
                "animation" => {}
                _ => {
                    tracing::error!(
                        "Type defined in config not one of 'image', 'video', 'animation'"
                    );
                    std::process::exit(1);
                }
            },
            None => {
                tracing::error!("Type not defined in config");
                std::process::exit(1);
            }
        };
        config
    }
}
