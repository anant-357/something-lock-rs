pub mod image_media;
pub mod video_media;

use image_media::Image;

pub enum Media {
    Image(Image),
    Shader(String),
    Screenshot,
    Solid(wgpu::Color),
    None,
}
use std::path::PathBuf;

use ini::Ini;
use wgpu::Color;

const SOLID_RED_DEFAULT_STR: &str = "255";
const SOLID_GREEN_DEFAULT_STR: &str = "255";
const SOLID_BLUE_DEFAULT_STR: &str = "255";
const SOLID_ALPHA_DEFAULT_STR: &str = "255";

const SOLID_RED_DEFAULT: u32 = 255;
const SOLID_GREEN_DEFAULT: u32 = 255;
const SOLID_BLUE_DEFAULT: u32 = 255;
const SOLID_ALPHA_DEFAULT: u32 = 255;

const IMAGE_BLUR_SIZE_DEFAULT: u32 = 4;
const IMAGE_BLUR_SIZE_DEFAULT_STR: &str = "4";

impl Media {
    pub fn from_config(base: &xdg::BaseDirectories) -> Self {
        let config_file = base
            .place_config_file("conf.ini")
            .expect("cannot create config directory");
        tracing::trace!("Config file path: {:?}", config_file);
        match Ini::load_from_file(config_file.clone()) {
            Ok(conf) => {
                let main_section = conf.section(Some("main")).unwrap();
                match main_section.get("type") {
                    Some(s) => match s {
                        "image" => {
                            let image_section = conf.section(Some("image")).unwrap();
                            Media::Image(Image::init(
                                &PathBuf::from(image_section.get("path").unwrap()),
                                image_section
                                    .get("blur")
                                    .unwrap_or(IMAGE_BLUR_SIZE_DEFAULT_STR)
                                    .parse::<u32>()
                                    .unwrap_or(IMAGE_BLUR_SIZE_DEFAULT),
                            ))
                        }
                        "screenshot" => {
                            let _screenshot_section = conf.section(Some("screenshot")).unwrap();
                            Media::Screenshot
                        }
                        "solid" => {
                            let solid_section = conf.section(Some("solid")).unwrap();
                            let r = solid_section
                                .get("red")
                                .unwrap_or(SOLID_RED_DEFAULT_STR)
                                .parse::<u32>()
                                .unwrap_or(SOLID_RED_DEFAULT);
                            let g = solid_section
                                .get("green")
                                .unwrap_or(SOLID_GREEN_DEFAULT_STR)
                                .parse::<u32>()
                                .unwrap_or(SOLID_GREEN_DEFAULT);
                            let b = solid_section
                                .get("blue")
                                .unwrap_or(SOLID_BLUE_DEFAULT_STR)
                                .parse::<u32>()
                                .unwrap_or(SOLID_BLUE_DEFAULT);
                            let a = solid_section
                                .get("alpha")
                                .unwrap_or(SOLID_ALPHA_DEFAULT_STR)
                                .parse::<u32>()
                                .unwrap_or(SOLID_ALPHA_DEFAULT);
                            tracing::trace!(
                                "Solid Type: Color is (rgba) ({}, {}, {}, {})",
                                r / 255,
                                g / 255,
                                b / 255,
                                a
                            );
                            Media::Solid(Color {
                                r: (r / 255) as f64,
                                g: (g / 255) as f64,
                                b: (b / 255) as f64,
                                a: (a / 255) as f64,
                            })
                        }
                        "shader" => {
                            let shader_section = conf.section(Some("shader")).unwrap();
                            Media::Shader(shader_section.get("path").unwrap().to_string())
                        }
                        _ => {
                            tracing::error!(
                                "Type defined in config not one of 'image', 'video', 'shader'"
                            );
                            std::process::exit(1);
                        }
                    },
                    None => {
                        tracing::error!("Type not defined in config");
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => {
                tracing::warn!("No config found at {:?}, using defaults", config_file);
                Media::Solid(Color {
                    r: (SOLID_RED_DEFAULT / 255) as f64,
                    g: (SOLID_GREEN_DEFAULT / 255) as f64,
                    b: (SOLID_BLUE_DEFAULT / 255) as f64,
                    a: (SOLID_ALPHA_DEFAULT / 255) as f64,
                })
            }
        }
    }
}
