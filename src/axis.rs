use crate::vec3::Vec3;

pub struct Axis{
    pub start: Vec3,
    pub end: Vec3,
    pub color: u32
}

impl Axis{
    pub const fn new(end: Vec3, color: u32) -> Self{
        Self { start: Vec3::ZERO, end: end, color: color }
    }
}
