use wgpu::{BindGroupLayout, Device, RenderPipeline, TextureFormat};
use crate::renderer::pipeline::common::load_shader;

/// Создаёт рендерный пайплайн для вокселей
pub fn create_voxel_pipeline(
    device: &Device,
    surface_format: TextureFormat,
    voxel_render_bind_group_layout: &BindGroupLayout,
    camera_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let vertex_shader = load_shader(device, "shaders/voxel_vertex.wgsl", "Voxel Vertex Shader");
    let fragment_shader = load_shader(device, "shaders/voxel_fragment.wgsl", "Voxel Fragment Shader");

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Voxel Render Pipeline Layout"),
        bind_group_layouts: &[camera_bind_group_layout, voxel_render_bind_group_layout], // ✅ Камера и воксели
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Voxel Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
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
        fragment: Some(wgpu::FragmentState {
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
