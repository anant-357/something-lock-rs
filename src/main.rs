use lock_connection::LockConnection;
use std::io;
use tracing::Level;
mod lock_connection;
mod lock_state;
mod lockgtk;
mod output;
//mod xdg_shell;

fn initialize_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(Level::TRACE)
        .with_writer(io::stderr)
        .init();
}

fn main() {
    initialize_tracing();
    let conn = LockConnection::connect();
    conn.lock();
}
