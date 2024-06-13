use smithay_client_toolkit::{
    reexports::client::{Connection, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
};

use crate::app_data::AppData;

impl SessionLockHandler for AppData {
    fn locked(&mut self, _conn: &Connection, qh: &QueueHandle<Self>, session_lock: SessionLock) {
        println!("Locked");

        for output in self.output_state.outputs() {
            let surface = self.compositor_state.create_surface(&qh);
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);
            self.loop_handle.insert_idle(|app_data| {
                app_data.lock_data.add_surface(lock_surface);
            });
        }
    }

    fn finished(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock: SessionLock,
    ) {
        println!("Finished");
        self.exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        let (_width, _height) = configure.new_size;
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
