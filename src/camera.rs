use crate::vec3::Vec3;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub scale: f32,
    pub dist: f32
}

impl Camera{
    pub const fn new() -> Self{
        Self { yaw: 0.6, pitch: 0.4, scale: 800.0, dist: 5.0 }
    }

    pub fn project(&self, p: Vec3, w: usize, h: usize) -> Option<(i32, i32)> {
        let p = p.rotate_y(self.yaw).rotate_x(self.pitch);
        let z = p.z + self.dist;
        
        if z <= 0.01{
            return None;
        }

        let sx = p.x * self.scale / z + w as f32 / 2.0;
        let sy = -p.y * self.scale / z + h as f32 / 2.0;
        Some((sx as i32, sy as i32))
    }
}
