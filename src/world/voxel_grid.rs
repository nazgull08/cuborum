pub struct VoxelGrid {
    pub voxels: Vec<Voxel>,
}

impl VoxelGrid {
    pub fn new() -> Self {
        Self { voxels: Vec::new() }
    }

    pub fn add_voxel(&mut self, voxel: Voxel) {
        self.voxels.push(voxel);
    }
}
