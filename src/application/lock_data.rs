use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::session_lock::{SessionLock, SessionLockState, SessionLockSurface};

use crate::pam::auth;
use crate::AppData;

pub struct LockData {
    session_lock: Option<SessionLock>,
    session_lock_state: SessionLockState,
    lock_surfaces: Vec<SessionLockSurface>,
    retries: usize,
    pub password_buffer: String,
}

impl LockData {
    pub fn from_state(session_lock_state: SessionLockState) -> Self {
        LockData {
            session_lock: None,
            retries: 0,
            password_buffer: String::new(),
            session_lock_state,
            lock_surfaces: Vec::new(),
        }
    }

    pub fn add_surface(&mut self, lock_surface: SessionLockSurface) {
        self.lock_surfaces.push(lock_surface);
    }

    pub fn lock(&mut self, qh: &QueueHandle<AppData>) {
        tracing::trace!("Starting Locking process");
        self.session_lock = Some(
            self.session_lock_state
                .lock(&qh)
                .expect("ext-session-lock not supported!"),
        )
    }

    pub fn unlock_with_auth(&mut self) -> Result<(), &str> {
        match auth(
            String::from("system-auth"),
            whoami::username(),
            self.password_buffer.clone(),
        ) {
            Ok(_) => {
                self.unlock();
                return Ok(());
            }
            Err(e) => {
                tracing::trace!("Error while authenticating!: {:#?}", e);
                self.password_buffer = String::new();
                self.retries = self.retries + 1;
                return Err("Authentication Error!");
            }
        };
    }

    pub fn unlock(&mut self) {
        self.session_lock.take().unwrap().unlock();
    }
}
