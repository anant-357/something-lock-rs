mod app_data;
mod handlers;
mod lock_data;
mod lock_screen;
mod pam;

use app_data::AppData;
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
    AppData::connect();
}
