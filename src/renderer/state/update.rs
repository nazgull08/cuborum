use crate::renderer::state::State;

pub fn update(state: &mut State) {
    // Обновляем камеру (перемещение и повороты)
    state.camera.process_keyboard();
    
    // Если в будущем будут обновления вокселей (например, добавление или удаление),
    // мы будем загружать их в GPU-буфер здесь:
    //
    // state.queue.write_buffer(&state.voxel_buffer, 0, bytemuck::cast_slice(&state.voxel_grid.data));

    // Логика обновления других элементов игры может идти здесь
}
