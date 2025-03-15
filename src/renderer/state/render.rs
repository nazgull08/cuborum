use crate::renderer::state::State;
use wgpu::{TextureView, CommandEncoder};

pub fn render(state: &mut State) {
    let camera_matrix = state.camera.projection_matrix() * state.camera.view_matrix();
    state.queue.write_buffer(&state.camera_buffer, 0, bytemuck::cast_slice(camera_matrix.as_slice()));

    let surface_texture = state.surface.get_current_texture().expect("Failed to acquire next swapchain texture");
    let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state.device.create_command_encoder(&Default::default());

    state.run_compute_pass(&mut encoder);

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    render_pass.set_bind_group(0, &state.camera_bind_group, &[]);
    render_pass.set_bind_group(1, &state.voxel_render_bind_group, &[]); // Воксели
    render_pass.set_pipeline(&state.voxel_pipeline);
    render_pass.draw(0..(state.voxel_grid.size * state.voxel_grid.size * state.voxel_grid.size) as u32, 0..1);

    drop(render_pass);

    state.queue.submit([encoder.finish()]);
    state.window.pre_present_notify();
    surface_texture.present();
}
