use image::GenericImageView;
use smithay_client_toolkit::{
    reexports::client::{protocol::wl_shm, Connection, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
    shm::raw::RawPool,
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
        qh: &QueueHandle<Self>,
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        let (width, height) = configure.new_size;
        let mut pool = RawPool::new(width as usize * height as usize * 4, &self.shm).unwrap();
        let canvas = pool.mmap();
        if self.image.is_some() {
            let image_rgba8 = self.image.clone().unwrap().to_rgba8();
            {
                let image = image::imageops::resize(
                    &image_rgba8,
                    width,
                    height,
                    image::imageops::FilterType::Nearest,
                );
                for (pixel, argb) in image.pixels().zip(canvas.chunks_exact_mut(4)) {
                    argb[3] = pixel.0[3];
                    argb[2] = pixel.0[0];
                    argb[1] = pixel.0[1];
                    argb[0] = pixel.0[2];
                }
            }
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

            buffer.destroy();
        } else {
            canvas
                .chunks_exact_mut(4)
                .enumerate()
                .for_each(|(index, chunk)| {
                    let x = (index % width as usize) as u32;
                    let y = (index / width as usize) as u32;

                    let a = 0xFF;
                    let r = u32::min(((width - x) * 0xFF) / width, ((height - y) * 0xFF) / height);
                    let g = u32::min((x * 0xFF) / width, ((height - y) * 0xFF) / height);
                    let b = u32::min(((width - x) * 0xFF) / width, (y * 0xFF) / height);
                    let color = (a << 24) + (r << 16) + (g << 8) + b;

                    let array: &mut [u8; 4] = chunk.try_into().unwrap();
                    *array = color.to_le_bytes();
                });
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

            buffer.destroy();
        }
    }
}
smithay_client_toolkit::delegate_session_lock!(AppData);
