use std::sync::Arc;
use tracing::info;
use wgpu::{
    Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    InstanceFlags, Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureFormat, TextureUsages, BindGroup, BindGroupLayout,
};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;
use crate::renderer::vertex::{Vertex, create_vertex_buffer, create_index_buffer};
use crate::renderer::pipeline::create_pipeline;
use crate::renderer::camera::Camera;
use wgpu::util::DeviceExt;

pub struct State {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: Surface<'static>,
    surface_format: TextureFormat,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_indices: u32, 
    camera: Camera, 
    camera_buffer: wgpu::Buffer, 
    camera_bind_group: BindGroup, 
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
        let index_buffer = create_index_buffer(&device);
        let num_indices = 6; // Количество индексов для квадрата

        // ✅ Создание камеры
        let camera = Camera::new(size.width as f32 / size.height as f32);
        let camera_matrix = camera.projection_matrix() * camera.view_matrix();

        // ✅ Буфер камеры
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(camera_matrix.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // ✅ Bind Group Layout для камеры
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // ✅ Bind Group для камеры
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let state = Self {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            camera,
            camera_buffer,
            camera_bind_group,
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
        let camera_matrix = self.camera.projection_matrix() * self.camera.view_matrix();
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(camera_matrix.as_slice()));

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

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
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

    pub fn input(&mut self, event: &WindowEvent) {
        info!("Event received: {:?}", event); // ✅ Логируем все события

        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. },
                ..
            } => {
                info!("Key Pressed: {:?}", key); // ✅ Логируем нажатую клавишу
                match key.as_ref() {
                    Key::Character("w") => {
                        self.camera.process_keyboard("forward");
                        info!("Moving forward");
                    }
                    Key::Character("s") => {
                        self.camera.process_keyboard("backward");
                        info!("Moving backward");
                    }
                    Key::Character("a") => {
                        self.camera.process_keyboard("left");
                        info!("Moving left");
                    }
                    Key::Character("d") => {
                        self.camera.process_keyboard("right");
                        info!("Moving right");
                    }
                    Key::Named(NamedKey::Escape) => {
                        info!("Exiting...");
                        std::process::exit(0);
                    }
                    _ => (),
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                info!("Cursor moved: x={}, y={}", position.x, position.y); // ✅ Логируем движение мыши
                let delta_x = position.x as f32 - (self.size.width as f32 / 2.0);
                let delta_y = (self.size.height as f32 / 2.0) - position.y as f32;

                self.camera.process_mouse(delta_x, delta_y);
                info!("Camera rotated: delta_x={}, delta_y={}", delta_x, delta_y);
            }

            _ => (),
        }
    }
}
