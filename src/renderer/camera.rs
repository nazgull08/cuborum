use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use std::collections::HashSet;
use winit::keyboard::Key;

pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,   // Горизонтальный угол
    pub pitch: f32, // Вертикальный угол
    pub speed: f32,  // Скорость движения
    pub sensitivity: f32, // Чувствительность мыши
    pub last_cursor_x: f32,
    pub last_cursor_y: f32,
    pub pressed_keys: HashSet<Key>, // ✅ Теперь храним нажатые клавиши
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 3.0), // Камера ближе к объекту
            direction: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect_ratio,
            fov: 45.0_f32.to_radians(),
            near: 0.1,
            far: 100.0,
            yaw: -90.0, // Теперь смотрим вдоль -Z
            pitch: 0.0,
            sensitivity: 0.05, // ✅ Чувствительность мыши
            speed: 0.02, // ✅ Чувствительность мыши
            last_cursor_x: 0.0,
            last_cursor_y: 0.0,
            pressed_keys: HashSet::new(),
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect_ratio, self.fov, self.near, self.far).to_homogeneous()
    }

    pub fn process_keyboard(&mut self) {
        let mut move_dir = Vector3::zeros();

        if self.pressed_keys.contains(&Key::Character("w".into())) {
            move_dir += self.direction;
        }
        if self.pressed_keys.contains(&Key::Character("s".into())) {
            move_dir -= self.direction;
        }
        if self.pressed_keys.contains(&Key::Character("a".into())) {
            move_dir -= self.direction.cross(&self.up).normalize();
        }
        if self.pressed_keys.contains(&Key::Character("d".into())) {
            move_dir += self.direction.cross(&self.up).normalize();
        }
        if self.pressed_keys.contains(&Key::Named(winit::keyboard::NamedKey::Space)) {
            move_dir += self.up; // Подъём вверх
        }
        if self.pressed_keys.contains(&Key::Named(winit::keyboard::NamedKey::Control)) {
            move_dir -= self.up; // Спуск вниз
        }
        if self.pressed_keys.contains(&Key::Character("r".into())) {
            self.reset(); // Сброс положения камеры
        }

        if move_dir.norm() > 0.0 {
            self.position += move_dir.normalize() * self.speed;
        }
    }

    pub fn process_mouse(&mut self, cursor_x: f32, cursor_y: f32) {
        let delta_x = cursor_x - self.last_cursor_x;
        let delta_y = self.last_cursor_y - cursor_y; // Инверсия Y

        self.last_cursor_x = cursor_x;
        self.last_cursor_y = cursor_y;

        self.yaw += delta_x * self.sensitivity;
        self.pitch += delta_y * self.sensitivity;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let yaw_radians = self.yaw.to_radians();
        let pitch_radians = self.pitch.to_radians();

        self.direction = Vector3::new(
            yaw_radians.cos() * pitch_radians.cos(),
            pitch_radians.sin(),
            yaw_radians.sin() * pitch_radians.cos(),
        )
        .normalize();
    }

    pub fn reset(&mut self) {
        self.position = Point3::new(0.0, 0.0, 3.0);
        self.yaw = -90.0;
        self.pitch = 0.0;
        self.direction = Vector3::new(0.0, 0.0, -1.0);
    }


    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }
}
