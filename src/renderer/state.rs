use std::sync::Arc;
use wgpu::{
    Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    InstanceFlags, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::window::Window;
use crate::renderer::vertex::{Vertex, create_vertex_buffer};
use crate::renderer::pipeline::create_pipeline;

pub struct State {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: Surface<'static>,
    surface_format: TextureFormat,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl State {
    
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::VULKAN | Backends::GL,
            flags: InstanceFlags::default(),
            backend_options: Default::default(),
        });

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: true,
        })
        .await
        .expect("Failed to find a suitable GPU!");

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: Some("Cuborum Device"),
                required_features: Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults(),
                memory_hints: MemoryHints::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device!");

        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];

        let pipeline = create_pipeline(&device, surface_format);
        let vertex_buffer = create_vertex_buffer(&device);

        let state = Self {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            pipeline,
            vertex_buffer,
        };

        state.configure_surface();
        state
    }

    fn configure_surface(&self) {
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().expect("Failed to acquire next swapchain texture");
        let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        drop(render_pass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }
}
