use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3], 
    pub color: [f32; 4],   
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3, // Координаты
        },
        wgpu::VertexAttribute {
            offset: 12,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x4, // Цвет
        },
    ];
}

pub fn create_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices = [
        Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0, 1.0] }, // Красный
        Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0, 1.0] }, // Зелёный
        Vertex { position: [ 0.5,  0.5, 0.0], color: [0.0, 0.0, 1.0, 1.0] }, // Синий
        Vertex { position: [-0.5,  0.5, 0.0], color: [1.0, 1.0, 0.0, 1.0] }, // Жёлтый
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn create_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let indices: [u16; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    })
}
