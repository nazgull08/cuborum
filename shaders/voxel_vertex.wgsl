struct Voxel {
    voxel_type: u32,
    color: u32,
};

@group(1) @binding(0)
var<storage, read> voxels: array<Voxel>;

@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// Функция для декодирования `u32 -> vec4<f32>`
fn unpack_color(packed: u32) -> vec4<f32> {
    let r = f32((packed >> 24) & 0xFF) / 255.0;
    let g = f32((packed >> 16) & 0xFF) / 255.0;
    let b = f32((packed >> 8) & 0xFF) / 255.0;
    let a = f32(packed & 0xFF) / 255.0;
    return vec4<f32>(r, g, b, a);
}

@vertex
fn main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let voxel = voxels[vertex_index];

    let grid_size = 16.0;
    let pos = vec3<f32>(
        f32(vertex_index % 16) / grid_size,
        f32((vertex_index / 16) % 16) / grid_size,
        f32(vertex_index / 256) / grid_size
    );

    var out: VertexOutput;
    out.position = view_proj * vec4<f32>(pos * 2.0 - 1.0, 1.0);
    out.color = unpack_color(voxel.color);
    return out;
}
