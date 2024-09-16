use wgpu::{
    Adapter, BindGroup, Buffer, Device, DeviceDescriptor, Instance, InstanceDescriptor,
    InstanceFlags, Queue, RenderPipeline, RequestAdapterOptions, Surface, Texture,
};

mod image;
mod shader;
pub mod surface;
use crate::graphics::surface::LockSurfaceWrapper;

pub struct Graphics {
    pub instance: Instance,
    pipeline: Option<RenderPipeline>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    texture: Option<Texture>,
    texture_bind_group: Option<BindGroup>,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
}

impl Graphics {
    pub fn adapter(&self) -> &Adapter {
        self.adapter.as_ref().unwrap()
    }

    pub fn device(&self) -> &Device {
        self.device.as_ref().unwrap()
    }

    pub fn queue(&self) -> &Queue {
        self.queue.as_ref().unwrap()
    }

    fn vertex_buffer(&self) -> &Buffer {
        self.vertex_buffer.as_ref().unwrap()
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
            pipeline: None,
            adapter: None,
            device: None,
            queue: None,
            texture: None,
            texture_bind_group: None,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub async fn init(&mut self, compatible_surface: Option<&Surface<'_>>) {
        let adapter = self
            .instance
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

    pub fn render_color(
        &self,
        surface: &LockSurfaceWrapper,
        color: wgpu::Color,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get_surface().get_current_texture().unwrap();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        Ok(())
    }
}
