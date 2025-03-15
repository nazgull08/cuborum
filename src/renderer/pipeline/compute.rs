use wgpu::{BindGroupLayout, ComputePipeline, Device};
use crate::renderer::pipeline::common::load_shader;

/// Создаёт Compute Pipeline для работы с вокселями
pub fn create_compute_pipeline(device: &Device, voxel_bind_group_layout: &BindGroupLayout) -> ComputePipeline {
    let compute_shader = load_shader(device, "shaders/compute_voxel.wgsl", "Compute Shader");

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[voxel_bind_group_layout], // ✅ Используем биндинг вокселей
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
