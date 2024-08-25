use futures::executor;
use smithay_client_toolkit::{
    reexports::client::{ Connection, Proxy, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    }
};
use std::ptr::NonNull;
use raw_window_handle::{WaylandDisplayHandle, WaylandWindowHandle};

use crate::application::{graphics::Graphics,  AppData};

impl SessionLockHandler for AppData {
    fn locked(&mut self, conn: &Connection, qh: &QueueHandle<Self>, session_lock: SessionLock) {
        println!("Locked");

        for output in self.output_state.outputs() {
            let surface = self.compositor_state.create_surface(&qh);
            //let window = self.xdg_state.create_window(surface, smithay_client_toolkit::shell::xdg::window::WindowDecorations::None, qh);
            //window.commit();
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);
            let raw_display_handle =
                    raw_window_handle::RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                        NonNull::new(conn.backend().display_ptr() as *mut _).unwrap(),
                    ));
                let raw_window_handle =
                    raw_window_handle::RawWindowHandle::Wayland(WaylandWindowHandle::new(
                        NonNull::new(lock_surface.wl_surface().id().as_ptr() as *mut _).unwrap(),
                    ));

                self.graphics_state = Some(executor::block_on(Graphics::new(
                    wgpu::SurfaceTargetUnsafe::RawHandle {
                        raw_window_handle,
                        raw_display_handle,
                    },
                    1920,
                    1080,
                )));

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
        qh: &QueueHandle<Self>,
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        let (width, height) = configure.new_size;
        self.graphics_state.as_mut().unwrap().resize(width, height);
        self.graphics_state.as_ref().unwrap().render().unwrap();
        session_lock_surface.wl_surface().commit();
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
