pub mod init;
pub mod render;
pub mod input;
pub mod update;

use std::sync::Arc;
use wgpu::{BindGroup, Buffer, CommandEncoder, ComputePipeline, Device, Queue, RenderPipeline, Surface, TextureFormat};
use winit::window::Window;
use crate::renderer::camera::Camera;
use crate::renderer::voxel::VoxelGrid;
use winit::dpi::PhysicalSize;

pub struct State {
    pub window: Arc<Window>,
    pub device: Device,
    pub queue: Queue,
    pub size: PhysicalSize<u32>,
    pub surface: Surface<'static>,
    pub surface_format: TextureFormat,
    pub compute_pipeline: ComputePipeline,
    pub voxel_pipeline: RenderPipeline,
    pub voxel_grid: VoxelGrid,
    pub voxel_buffer: Buffer,
    pub voxel_compute_bind_group: BindGroup,
    pub voxel_render_bind_group: BindGroup,
    pub camera: Camera,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        init::initialize(window).await
    }

    pub fn render(&mut self) {
        render::render(self);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) {
        input::process_input(self, event);
    }

    pub fn update(&mut self) {
        update::update(self);
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn run_compute_pass(&self, encoder: &mut CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.voxel_compute_bind_group, &[]);
        compute_pass.dispatch_workgroups(2, 2, 2); // 🟢 Запускаем compute shader

        drop(compute_pass);
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }
}
