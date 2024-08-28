use std::ptr::NonNull;

use futures::executor;
use raw_window_handle::{WaylandDisplayHandle, WaylandWindowHandle};
use smithay_client_toolkit::{
    reexports::client::{Connection, Proxy, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
};

use crate::{graphics::{GContext, GSurfaceWrapper}, media::Media, AppData};

impl SessionLockHandler for AppData {
    fn locked(&mut self, conn: &Connection, qh: &QueueHandle<Self>, session_lock: SessionLock) {
        tracing::trace!("Locked");
        for output in self.states.output_state.outputs() {
            let output_info = self.states.output_state.info(&output).unwrap();
            let size = output_info.logical_size.unwrap();
            let surface = self.states.compositor_state.create_surface(&qh);
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);

            let raw_display_handle =
                raw_window_handle::RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                    NonNull::new(conn.backend().display_ptr() as *mut _).unwrap(),
                ));
            let raw_window_handle =
                raw_window_handle::RawWindowHandle::Wayland(WaylandWindowHandle::new(
                    NonNull::new(lock_surface.wl_surface().id().as_ptr() as *mut _).unwrap(),
                ));
            let mut gsurface = GSurfaceWrapper::new();
            let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle,
                raw_display_handle,
            };
            let mut gcontext = GContext::new();

            let vsurface = unsafe {
                gcontext
                    .instance
                    .create_surface_unsafe(surface_target)
                    .unwrap()
            };
            executor::block_on(gcontext.init(Some(&vsurface)));
            gsurface.init(&gcontext, vsurface, (size.0 as u32, size.1 as u32));
            self.graphics_context = Some(gcontext);
            self.gsurfaces.push(gsurface);
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
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        tracing::trace!("Configure Event");
        let (width, height) = configure.new_size;
        let gcontext = self.graphics_context.as_ref().unwrap();
        let surface = self.gsurfaces.get_mut(0).unwrap();
        surface.resize(gcontext, (width, height));
        match self.media {
            Media::Solid(color) => {
                gcontext.render_color(&surface,color ).unwrap();
            }
                _ => {tracing::trace!("Image, Video not supported yet!");}
        }
       
        session_lock_surface.wl_surface().commit();
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
