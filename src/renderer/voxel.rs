#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Voxel {
    pub voxel_type: u32,   // 4 байта 
    pub color: u32, 
}

impl Voxel {
    pub fn empty() -> Self {
        Self {
            voxel_type: 0,
            color: 0, 
        }
    }

    pub fn new(voxel_type: u32, r: u8, g: u8, b: u8, a: u8) -> Self {
        let packed_color = (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32);
        Self {
            voxel_type,
            color: packed_color,
        }
    }

    pub fn unpack_color(&self) -> [f32; 4] {
        let r = ((self.color >> 24) & 0xFF) as f32 / 255.0;
        let g = ((self.color >> 16) & 0xFF) as f32 / 255.0;
        let b = ((self.color >> 8) & 0xFF) as f32 / 255.0;
        let a = (self.color & 0xFF) as f32 / 255.0;
        [r, g, b, a]
    }
}

pub struct VoxelGrid {
    pub size: usize,
    pub data: Vec<Voxel>,
}


impl VoxelGrid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![Voxel::empty(); size * size * size],
        }
    }

    pub fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.size * self.size) + (y * self.size) + x
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &Voxel {
        &self.data[self.get_index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) {
        let index = self.get_index(x, y, z);
        self.data[index] = voxel;
    }

    pub fn fill_with_test_pattern(&mut self) {
        for z in 0..self.size {
            for y in 0..self.size / 2 {
                for x in 0..self.size {
                    self.set(
                        x,
                        y,
                        z,
                        Voxel {
                            voxel_type: 1,
                            color: 128 
                        },
                    );
                }
            }
        }
    }
}
