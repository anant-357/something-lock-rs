use std::{path::PathBuf, ptr::NonNull};

use futures::executor;
use raw_window_handle::{WaylandDisplayHandle, WaylandWindowHandle};
use smithay_client_toolkit::{
    reexports::client::{Connection, Proxy, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
};

use crate::{graphics::surface::LockSurfaceWrapper, media::Media, AppData};

impl SessionLockHandler for AppData {
    fn locked(&mut self, conn: &Connection, qh: &QueueHandle<Self>, session_lock: SessionLock) {
        tracing::trace!("Locked");
        for output in self.wayland.output_state.outputs() {
            let output_info = self.wayland.output_state.info(&output).unwrap();
            let size = output_info.logical_size.unwrap();
            let surface = self.wayland.compositor_state.create_surface(qh);
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);

            let raw_display_handle =
                raw_window_handle::RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                    NonNull::new(conn.backend().display_ptr() as *mut _).unwrap(),
                ));
            let raw_window_handle =
                raw_window_handle::RawWindowHandle::Wayland(WaylandWindowHandle::new(
                    NonNull::new(lock_surface.wl_surface().id().as_ptr() as *mut _).unwrap(),
                ));

            let mut gsurface = LockSurfaceWrapper::new(lock_surface, Media::from_config(&self.xdg));
            let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle,
                raw_display_handle,
            };

            let vsurface = unsafe {
                self.graphics_context
                    .instance
                    .create_surface_unsafe(surface_target)
                    .unwrap()
            };
            executor::block_on(self.graphics_context.init(Some(&vsurface)));
            gsurface.init(
                &self.graphics_context,
                vsurface,
                (size.0 as u32, size.1 as u32),
            );
            self.lock_data.add_surface(gsurface);
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
        let surface = self.lock_data.session_lock_surfaces.get_mut(0).unwrap();
        surface.resize(&self.graphics_context, (width, height));
        match self.lock_data.media {
            Media::Solid(color) => {
                self.graphics_context.render_color(surface, color).unwrap();
            }
            Media::Image(ref mut im) => {
                im.resize(width, height);
                self.graphics_context
                    .create_texture_from_image_for_surface(surface, im);
                self.graphics_context
                    .render_texture_for_image(surface)
                    .unwrap();
            }
            Media::Shader(ref path) => {
                self.graphics_context
                    .create_texture_from_shader_for_surface(surface, &PathBuf::from(path));
                self.graphics_context
                    .render_texture_for_shader(surface)
                    .unwrap();
            }
            _ => {
                tracing::trace!("Screenshot, Video not supported yet!");
            }
        }

        session_lock_surface.wl_surface().commit();
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
