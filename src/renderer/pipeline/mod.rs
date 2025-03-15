pub mod compute;
pub mod voxel;
pub mod common;

use wgpu::BindGroupLayout;
use wgpu::Device;
use wgpu::TextureFormat;
use wgpu::ComputePipeline;
use wgpu::RenderPipeline;

pub use compute::create_compute_pipeline;
pub use voxel::create_voxel_pipeline;

/// Создаёт все пайплайны (compute и voxel)
pub fn create_pipelines(
    device: &Device,
    surface_format: TextureFormat,
    voxel_compute_bind_group_layout: &BindGroupLayout,
    voxel_render_bind_group_layout: &BindGroupLayout,
    camera_bind_group_layout: &BindGroupLayout,
) -> (ComputePipeline, RenderPipeline) {
    let compute_pipeline = create_compute_pipeline(device, voxel_compute_bind_group_layout);
    let voxel_pipeline = create_voxel_pipeline(device, surface_format, voxel_render_bind_group_layout, camera_bind_group_layout);
    (compute_pipeline, voxel_pipeline)
}
