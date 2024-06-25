use fast_image_resize::{IntoImageView, ResizeOptions, Resizer};
use image::DynamicImage;
use smithay_client_toolkit::{
    reexports::client::{protocol::wl_shm, Connection, QueueHandle},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockSurface, SessionLockSurfaceConfigure,
    },
    shm::raw::RawPool,
};

use crate::application::{media::Media, AppData};

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
        match self.media {
            Media::Image(ref mut i) => {
                let image = i.buffer.clone();
                if width != image.width() || height != image.height() {
                    let mut new =
                        fast_image_resize::images::Image::new(width, height, i.pixel_type);
                    Resizer::new()
                        .resize(
                            &image,
                            &mut new,
                            &ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Nearest),
                        )
                        .unwrap();
                    i.set_buffer(image.clone());
                    tracing::trace!("Resized image!");
                }
                {
                    for (pixel, argb) in image.to_rgba8().pixels().zip(canvas.chunks_exact_mut(4)) {
                        argb[3] = pixel.0[3];
                        argb[2] = pixel.0[0];
                        argb[1] = pixel.0[1];
                        argb[0] = pixel.0[2];
                    }
                }

                tracing::trace!("Converted pixels!");
            }
            Media::Solid(color) => {
                let a = (color.alpha()) as u32;
                let r = (color.red()) as u32;
                let g = (color.green()) as u32;
                let b = (color.blue()) as u32;
                tracing::trace!("Rendering Solid Color: ({},{},{},{})", r, g, b, a);
                canvas
                    .chunks_exact_mut(4)
                    .enumerate()
                    .for_each(|(_index, chunk)| {
                        let color: u32 = (a << 24) + (r << 16) + (g << 8) + b;

                        let array: &mut [u8; 4] = chunk.try_into().unwrap();
                        *array = color.to_le_bytes();
                    });
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
