use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub fn create_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices = [
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [ 0.5, -0.5, 0.0] },
        Vertex { position: [ 0.5,  0.5, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [ 0.5,  0.5, 0.0] },
        Vertex { position: [-0.5,  0.5, 0.0] },
    ];
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
