mod app;
mod config;
mod graphics;
mod handlers;
mod lock;
mod media;
mod pam;

use app::AppData;
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
    let xdg_dirs = xdg::BaseDirectories::with_prefix("something_lock").unwrap();
    AppData::connect(xdg_dirs);
}
