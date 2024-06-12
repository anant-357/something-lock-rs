use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::session_lock::{SessionLock, SessionLockState, SessionLockSurface};

use crate::AppData;

pub struct LockData {
    session_lock: Option<SessionLock>,
    lock_surfaces: Vec<SessionLockSurface>,
    session_lock_state: SessionLockState,
}

impl LockData {
    pub fn from_state(session_lock_state: SessionLockState) -> Self {
        LockData {
            session_lock: None,
            lock_surfaces: Vec::new(),
            session_lock_state,
        }
    }

    pub fn lock(&mut self, qh: &QueueHandle<AppData>) {
        self.session_lock = Some(
            self.session_lock_state
                .lock(&qh)
                .expect("ext-session-lock not supported!"),
        )
    }

    pub fn add_surface(&mut self, surface: SessionLockSurface) {
        self.lock_surfaces.push(surface);
    }

    pub fn unlock(&mut self) {
        self.session_lock.take().unwrap().unlock();
    }
}
