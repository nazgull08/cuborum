use std::sync::Arc;
use wgpu::{Backends, Device, Queue, Instance, InstanceDescriptor, InstanceFlags, PowerPreference, RequestAdapterOptions};
use winit::window::Window;
use crate::renderer::pipeline::{create_pipelines};
use crate::renderer::camera::Camera;
use crate::renderer::voxel::{Voxel, VoxelGrid};
use wgpu::util::DeviceExt;
use wgpu::{Surface, TextureFormat, BindGroup, Buffer};

pub async fn initialize(window: Arc<Window>) -> crate::renderer::state::State {
    let instance = Instance::new(&InstanceDescriptor {
        backends: Backends::VULKAN | Backends::GL,
        flags: InstanceFlags::default(),
        backend_options: Default::default(),
    });

    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: true,
    }).await.expect("Failed to find a suitable GPU!");

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Cuborum Device"),
            required_features: wgpu::Features::VERTEX_WRITABLE_STORAGE, // 🔥 Добавляем эту фичу
            required_limits: adapter.limits().using_resolution(wgpu::Limits::downlevel_defaults()),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ).await.expect("Failed to create device!");

    let surface = instance.create_surface(window.clone()).unwrap();
    let inner_size = window.inner_size().clone();
    let capabilities = surface.get_capabilities(&adapter);
    let surface_format = capabilities.formats[0];

    // === Создаём Layout для камеры ===
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

    let binding_size = std::num::NonZeroU64::new(8).unwrap();
    println!("DEBUG===================================");
    println!("binding_size: {:?}", binding_size);

    // === Создаём Layout для вокселей ===
    let voxel_compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Voxel Compute Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: Some(binding_size),
            },
            count: None,
        }],
    });

    let voxel_render_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Voxel Render Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: Some(binding_size),
            },
            count: None,
        }],
    });


    let mut voxel_grid = VoxelGrid::new(16);
    voxel_grid.fill_with_test_pattern();

    let voxel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Voxel Buffer"),
        contents: bytemuck::cast_slice(&voxel_grid.data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX, // ✅ Добавили `VERTEX`
    });

    // === Создаём BindGroup для вокселей ===
    let voxel_compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Voxel Compute Bind Group"),
        layout: &voxel_compute_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: voxel_buffer.as_entire_binding(),
        }],
    });

    let voxel_render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Voxel Render Bind Group"),
        layout: &voxel_render_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: voxel_buffer.as_entire_binding(),
        }],
    });

    // === Создаём пайплайны ===
    let (compute_pipeline, voxel_pipeline) = create_pipelines(
        &device, 
        surface_format, 
        &voxel_compute_bind_group_layout,
        &voxel_render_bind_group_layout,
        &camera_bind_group_layout
    );

    let camera = Camera::new(1.0);
    let camera_matrix = camera.view_proj_matrix();
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(camera_matrix.as_ref()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // === Создаём BindGroup для камеры ===
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
    });

    crate::renderer::state::State {
        window,
        device,
        queue,
        size: inner_size,
        surface,
        surface_format,
        compute_pipeline,
        voxel_pipeline,
        voxel_grid,
        voxel_buffer,
        voxel_compute_bind_group,
        voxel_render_bind_group,
        camera,
        camera_buffer,
        camera_bind_group,
    }
}
