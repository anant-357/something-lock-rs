use std::{borrow::BorrowMut, u32};

use drm::buffer::DrmFourcc;
use smithay_client_toolkit::{
    dmabuf::{DmabufFormat, DmabufParams},
    reexports::{
        client::{protocol::wl_shm, Connection, QueueHandle},
        protocols::wp::linux_dmabuf::zv1::client::zwp_linux_buffer_params_v1::Flags,
    },
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
    shm::raw::RawPool,
};

use crate::{app_data::AppData, media::MediaType};

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
        qh: &QueueHandle<Self>,
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        tracing::trace!("Starting SessionLockSurface configure");
        let (width, height) = configure.new_size;
        let mut pool = RawPool::new(width as usize * height as usize * 4, &self.shm).unwrap();
        let canvas = pool.mmap();
        tracing::trace!("Created pool and canvas!");
        match self.media.base {
            MediaType::Image(ref mut i) => {
                let mut image = i.buffer.clone();
                if width != image.width() || height != image.height() {
                    image = image::imageops::resize(
                        &image,
                        width,
                        height,
                        image::imageops::FilterType::Nearest,
                    );
                    i.set_buffer(image.clone());
                    tracing::trace!("Resized image!");
                }
                {
                    for (pixel, argb) in image.pixels().zip(canvas.chunks_exact_mut(4)) {
                        argb[3] = pixel.0[3];
                        argb[2] = pixel.0[0];
                        argb[1] = pixel.0[1];
                        argb[0] = pixel.0[2];
                    }
                }

                tracing::trace!("Converted pixels!");
            }
            _ => {
                canvas
                    .chunks_exact_mut(4)
                    .enumerate()
                    .for_each(|(index, chunk)| {
                        let x = (index % width as usize) as u32;
                        let y = (index / width as usize) as u32;

                        let a = 0xFF;
                        let r =
                            u32::min(((width - x) * 0xFF) / width, ((height - y) * 0xFF) / height);
                        let g = u32::min((x * 0xFF) / width, ((height - y) * 0xFF) / height);
                        let b = u32::min(((width - x) * 0xFF) / width, (y * 0xFF) / height);
                        let color = (a << 24) + (r << 16) + (g << 8) + b;

                        let array: &mut [u8; 4] = chunk.try_into().unwrap();
                        *array = color.to_le_bytes();
                    });
            }
        };

        let buffer = pool.create_buffer(
            0,
            width as i32,
            height as i32,
            width as i32 * 4,
            wl_shm::Format::Argb8888,
            (),
            qh,
        );

        session_lock_surface
            .wl_surface()
            .attach(Some(&buffer), 0, 0);
        session_lock_surface.wl_surface().commit();
        tracing::trace!("Buffer Attatched!");

        buffer.destroy();
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
