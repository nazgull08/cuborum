#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Voxel {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
