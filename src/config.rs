use std::path::PathBuf;

use ini::Ini;
use wgpu::Color;

pub enum MediaType {
    Shader,
    Image,
    Solid,
    Video,
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Solid
    }
}

const SOLID_RED_DEFAULT_STR: &str = "255";
const SOLID_GREEN_DEFAULT_STR: &str = "255";
const SOLID_BLUE_DEFAULT_STR: &str = "255";
const SOLID_ALPHA_DEFAULT_STR: &str = "255";

const SOLID_RED_DEFAULT: u32 = 255;
const SOLID_GREEN_DEFAULT: u32 = 255;
const SOLID_BLUE_DEFAULT: u32 = 255;
const SOLID_ALPHA_DEFAULT: u32 = 255;

const IMAGE_BLUR_TYPE_DEFAULT: &str = "box";
const IMAGE_BLUR_SIZE_DEFAULT: u32 = 0;
const IMAGE_BLUR_SIZE_DEFAULT_STR: &str = "0";

pub struct Config {
    pub media_type: Option<MediaType>,
    pub media_path: Option<PathBuf>,
    pub color: Option<Color>,
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
    pub fn from_config_file(base: &xdg::BaseDirectories) -> Self {
        let config_file = base
            .place_config_file("conf.ini")
            .expect("cannot create config directory");
        tracing::trace!("Config file path: {:?}", config_file);
        let mut config = Config::empty();
        match Ini::load_from_file(config_file.clone()) {
            Ok(conf) => {
                let main_section = conf.section(Some("main")).unwrap();
                match main_section.get("type") {
                    Some(s) => match s {
                        "image" => {
                            let image_section = conf.section(Some("image")).unwrap();
                            config.media_type = Some(MediaType::Image);
                            config.media_path =
                                Some(PathBuf::from(image_section.get("path").unwrap()));
                            config.blur_size = Some(
                                image_section
                                    .get("blur_size")
                                    .unwrap_or(IMAGE_BLUR_SIZE_DEFAULT_STR)
                                    .parse::<u32>()
                                    .unwrap_or(IMAGE_BLUR_SIZE_DEFAULT),
                            );

                            config.blur_type = Some(
                                image_section
                                    .get("blur_type")
                                    .unwrap_or(IMAGE_BLUR_TYPE_DEFAULT)
                                    .to_string(),
                            );
                        }
                        "solid" => {
                            let solid_section = conf.section(Some("solid")).unwrap();
                            config.media_type = Some(MediaType::Solid);
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
                            config.color = Some(Color {
                                r: (r / 255) as f64,
                                g: (g / 255) as f64,
                                b: (b / 255) as f64,
                                a: (a / 255) as f64,
                            });
                        }
                        "video" => {}
                        "shader" => {
                            let shader_section = conf.section(Some("shader")).unwrap();
                            config.media_type = Some(MediaType::Shader);
                            config.media_path =
                                Some(PathBuf::from(shader_section.get("path").unwrap()));
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
                };
            }
            Err(_) => {
                tracing::warn!("No config found at {:?}, using defaults", config_file);
                config.media_type = Some(MediaType::default());
                config.color = Some(wgpu::Color {
                    r: (SOLID_RED_DEFAULT/255) as f64,
                    g: (SOLID_GREEN_DEFAULT/255) as f64,
                    b: (SOLID_BLUE_DEFAULT/255) as f64,
                    a: (SOLID_ALPHA_DEFAULT/255) as f64,                    
                });
            }
        }
        config
    }
}
