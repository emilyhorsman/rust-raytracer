use crate::color::*;
use crate::types::*;

pub struct PointLight {
    pub color: Color,
    pub position: Point3f,
}

impl PointLight {
    pub fn direction_from(&self, point: &Point3f) -> Vec3f {
        (self.position - point).normalize()
    }
}
