use wgpu::Device;
use wgpu::ShaderModule;
use std::fs;

/// Загружает WGSL-шейдер из файла
pub fn load_shader(device: &Device, path: &str, label: &str) -> ShaderModule {
    let shader_src = fs::read_to_string(path).expect(&format!("Failed to read shader file: {}", path));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(shader_src.into()),
    })
}
