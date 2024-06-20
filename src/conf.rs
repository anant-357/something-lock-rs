use std::path::{Path, PathBuf};

use ini::Ini;

pub enum MediaType {
    Image,
    Video,
    Animation,
}

pub struct Config {
    pub media_type: MediaType,
    pub media_path: PathBuf,
}
impl Config {
    pub fn from_config_file() -> Self {
        let conf = Ini::load_from_file("conf.ini").unwrap();
        let main_section = conf.section(Some("main")).unwrap();
        let media_type: MediaType = match main_section.get("type") {
            Some(s) => match s {
                "image" => {
                    let t = MediaType::Image;
                    t
                }
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
        let media_path: PathBuf = match main_section.get("path") {
            Some(s) => PathBuf::from(s),
            None => {
                tracing::error!("Path not defined in config");
                std::process::exit(1);
            }
        };
        Config {
            media_type,
            media_path,
        }
    }
}
