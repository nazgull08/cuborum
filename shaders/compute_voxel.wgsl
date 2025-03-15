struct Voxel {
    voxel_type: u32,
    color: u32,
};

@group(0) @binding(0)
var<storage, read_write> voxels: array<Voxel>;

@compute @workgroup_size(8, 8, 8)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x + id.y * 16 + id.z * 16 * 16;

    if voxels[index].voxel_type == 1 {
        voxels[index].color = (255u << 24) | (0u << 16) | (0u << 8) | 255u; // Красный
    }
}
