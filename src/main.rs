mod application;
mod conf;
mod pam;

use application::AppData;
use conf::Config;
use std::io;
use tracing::Level;

fn initialize_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(Level::TRACE)
        .with_writer(io::stderr)
        .init();
}

fn main() {
    initialize_tracing();
    let config = Config::from_config_file();
    AppData::connect(config);
}
