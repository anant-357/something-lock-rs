use std::{fs::read_to_string, path::Path};

use wgpu::{
    util::DeviceExt, Adapter, Buffer, Device, DeviceDescriptor, Instance, InstanceDescriptor,
    InstanceFlags, Queue, RenderPipeline, RequestAdapterOptions, Surface
};

mod vertex;
pub mod surface;
use vertex::Vertex;
use crate::graphics::surface::LockSurfaceWrapper;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
     Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];

pub struct Graphics {
    pub instance: Instance,
    pipeline: Option<RenderPipeline>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
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
            vertex_buffer: None,
            index_buffer: None
        }
    }

    pub fn create_texture_from_shader_for_surface(&mut self, surface: &LockSurfaceWrapper, shader_path: &Path) {
        let device = self.device();
        let adapter = self.adapter();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX
        });

        let data = read_to_string(shader_path).expect("Failed to read shader");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&data)),
        });


        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_surface().get_capabilities(adapter);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_capabilities.formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.pipeline = Some(render_pipeline);
    }

    pub fn create_texture_from_image_for_surface(
        &self,
        surface: &LockSurfaceWrapper,
        image: image::RgbaImage,
    ) {
        let surface_size = surface.size();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();

        let texture_size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let input_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("input texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            input_texture.as_image_copy(),
            bytemuck::cast_slice(image.as_raw()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * surface_size.0),
                rows_per_image: None,
            },
            texture_size,
        );
    }

    pub fn render_texture(&self, surface: &LockSurfaceWrapper) -> Result<(), wgpu::SurfaceError> {
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(self.pipeline.as_ref().unwrap());
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(self.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
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
