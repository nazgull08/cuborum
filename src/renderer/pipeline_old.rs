use wgpu::{BindGroupLayout, ComputePipeline, Device, FragmentState, RenderPipeline, ShaderModule, SurfaceConfiguration, VertexState};
use wgpu::util::DeviceExt;
use std::fs;

use crate::renderer::vertex::Vertex; // Импортируем новую структуру Vertex

fn load_shader(device: &Device, path: &str, label: &str) -> ShaderModule {
    let shader_src = fs::read_to_string(path).expect(&format!("Failed to read shader file: {}", path));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(shader_src.into()),
    })
}

pub fn create_pipeline(
    device: &Device,
    surface_format: wgpu::TextureFormat,
    camera_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let vertex_shader = load_shader(device, "shaders/vertex.wgsl", "Vertex Shader");
    let fragment_shader = load_shader(device, "shaders/fragment.wgsl", "Fragment Shader");

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[camera_bind_group_layout], // ✅ Добавляем поддержку вокселей
        push_constant_ranges: &[],
    });

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // Теперь stride = Vertex
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &Vertex::ATTRIBS, // Используем атрибуты из Vertex
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &vertex_shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(FragmentState {
            module: &fragment_shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    })
}

pub fn create_compute_pipeline(device: &Device, voxel_bind_group_layout: &BindGroupLayout) -> ComputePipeline {
    let compute_shader = load_shader(device, "shaders/compute_voxel.wgsl", "Compute Shader");

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[voxel_bind_group_layout], // ✅ Теперь используем voxel_bind_group_layout
        push_constant_ranges: &[],
    });

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &compute_shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None
    })
}

pub fn create_voxel_pipeline(
    device: &Device,
    surface_format: wgpu::TextureFormat,
    voxel_bind_group_layout: &BindGroupLayout,
    camera_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let vertex_shader = load_shader(device, "shaders/voxel_vertex.wgsl", "Voxel Vertex Shader");
    let fragment_shader = load_shader(device, "shaders/voxel_fragment.wgsl", "Voxel Fragment Shader");

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Voxel Render Pipeline Layout"),
        bind_group_layouts: &[camera_bind_group_layout, voxel_bind_group_layout], // ✅ Камера и воксели
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Voxel Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &vertex_shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            buffers: &[], // Пока без вершинного буфера
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList, // 🔥 Начнём с точек
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, // Воксели – это точки, куллинг не нужен
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(FragmentState {
            module: &fragment_shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    })
}
