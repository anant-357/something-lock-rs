use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, InstanceDescriptor, InstanceFlags, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration,
};

pub struct GSurfaceWrapper {
    surface: Option<Surface<'static>>,
    config: Option<SurfaceConfiguration>,
}

impl GSurfaceWrapper {
    pub fn new() -> Self {
        Self {
            surface: None,
            config: None,
        }
    }
    pub fn init(
        &mut self,
        gcontext: &GContext,
        surface_target: Surface<'static>,
        size: (u32, u32),
    ) {
        self.surface = Some(surface_target);
        let surface = self.surface.as_ref().unwrap();

        let config = surface
            .get_default_config(&gcontext.adapter(), size.0, size.1)
            .expect("Surface is not supported by the adapter");
        surface.configure(&gcontext.device(), &config);
        self.config = Some(config);
    }

    pub fn resize(&mut self, gcontext: &GContext, size: (u32, u32)) {
        let config = self.config.as_mut().unwrap();
        config.width = size.0;
        config.height = size.1;
        let surface = self.surface.as_ref().unwrap();
        surface.configure(&gcontext.device(), config);
    }

    fn get(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    fn config(&self) -> &SurfaceConfiguration {
        self.config.as_ref().unwrap()
    }
}

pub struct GContext {
    pub instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
}

impl GContext {
    pub fn adapter(&self) -> &Adapter {
        self.adapter.as_ref().unwrap()
    }

    pub fn device(&self) -> &Device {
        self.device.as_ref().unwrap()
    }

    pub fn queue(&self) -> &Queue {
        self.queue.as_ref().unwrap()
    }
    
    pub fn new() -> Self {
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version = wgpu::util::gles_minor_version_from_env().unwrap_or_default();
        let instance = Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: InstanceFlags::VALIDATION,
            dx12_shader_compiler,
            gles_minor_version,
        });
        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
        }
    }

    pub async fn init(&mut self, compatible_surface: Option<&Surface<'_>>) {
        let adapter = self.instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface,
            })
            .await
            .expect("No suitable GPU adapters found on the system!");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .expect("Unable to find a suitable GPU adapter");

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    pub fn render_color(&self, surface: &GSurfaceWrapper, color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get().unwrap().get_current_texture().unwrap();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =            device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok({})
    }
}
