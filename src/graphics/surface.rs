use smithay_client_toolkit::session_lock::SessionLockSurface;
use wgpu::{Surface, SurfaceConfiguration, TextureUsages};

use super::Graphics;

pub struct LockSurfaceWrapper {
    surface: Option<Surface<'static>>,
    config: Option<SurfaceConfiguration>,
    size: (u32, u32),
    _lock_surface: SessionLockSurface
}

impl LockSurfaceWrapper {
    pub fn new(lock_surface: SessionLockSurface) -> Self {
        Self {
            surface: None,
            config: None,
            size: (0, 0),
            _lock_surface: lock_surface
        }
    }
    pub fn init(
        &mut self,
        gcontext: &Graphics,
        surface_target: Surface<'static>,
        size: (u32, u32),
    ) {
        self.surface = Some(surface_target);
        let surface = self.surface.as_ref().unwrap();
        let capabilities = surface.get_capabilities(gcontext.adapter());
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(gcontext.device(), &config);
        self.config = Some(config);
        self.size = size;
    }

    pub fn resize(&mut self, gcontext: &Graphics, size: (u32, u32)) {
        let config = self.config.as_mut().unwrap();
        config.width = size.0;
        config.height = size.1;
        let surface = self.surface.as_ref().unwrap();
        surface.configure(gcontext.device(), config);
        self.size = size;
    }

    pub fn get_surface(&self) -> &Surface {
        self.surface.as_ref().unwrap()
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}
