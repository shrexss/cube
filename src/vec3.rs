#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3{
    pub const ZERO: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};

    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Self {x, y, z}
    }
    pub fn rotate_y(self, a: f32) -> Vec3 {
        let (s, c) = a.sin_cos();
        Vec3::new(self.x * c + self.z * s, self.y, -self.x * s + self.z * c)
    }
    pub fn rotate_x(self, a: f32) -> Vec3 {
        let (s, c) = a.sin_cos();
        Vec3::new(self.x, self.y * c - self.z * s, self.y * s + self.z * c)
    }
}