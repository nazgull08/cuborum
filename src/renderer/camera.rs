use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

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
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 3.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect_ratio,
            fov: 45.0_f32.to_radians(),
            near: 0.1,
            far: 100.0,
            yaw: -90.0,   // Стартовое направление — вдоль -Z
            pitch: 0.0,
            speed: 0.05,
            sensitivity: 0.1,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect_ratio, self.fov, self.near, self.far).to_homogeneous()
    }

    pub fn process_keyboard(&mut self, direction: &str) {
        let move_dir = match direction {
            "forward" => self.direction,
            "backward" => -self.direction,
            "left" => -self.direction.cross(&self.up).normalize(),
            "right" => self.direction.cross(&self.up).normalize(),
            _ => Vector3::zeros(),
        };

        self.position += move_dir * self.speed;
    }

    pub fn process_mouse(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch -= delta_y * self.sensitivity;

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
}
