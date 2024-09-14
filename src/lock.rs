use smithay_client_toolkit::session_lock::SessionLock;

use crate::graphics::surface::LockSurfaceWrapper;
use crate::pam::auth;

pub struct LockState {
    session_lock: SessionLock,
    retries: usize,
    pub session_lock_surfaces: Vec<LockSurfaceWrapper>,
    pub password_buffer: String,
}

impl LockState {
    pub fn from_lock(session_lock: SessionLock) -> Self {
        Self {
            session_lock,
            retries: 0,
            password_buffer: String::new(),
            session_lock_surfaces: Vec::new(),
        }
    }

    pub fn add_surface(&mut self, session_lock_surface: LockSurfaceWrapper) {
        self.session_lock_surfaces.push(session_lock_surface);
    }

    pub fn unlock_with_auth(&mut self) -> Result<(), &str> {
        match auth(
            String::from("system-auth"),
            whoami::username(),
            self.password_buffer.clone(),
        ) {
            Ok(_) => {
                self.session_lock.unlock();
                Ok(())
            }
            Err(_) => {
                self.password_buffer.clear();
                self.retries += self.retries;
                Err("Authentication Error!")
            }
        }
    }
}
