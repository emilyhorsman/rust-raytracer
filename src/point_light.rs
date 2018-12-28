use crate::color::*;
use crate::types::*;

pub struct PointLight {
    pub color: Color,
    pub position: Point3f,
}

impl PointLight {
    pub fn direction_from(&self, point: &Point3f) -> (Float, Vec3f) {
        let v = self.position - point;
        (v.norm(), v.normalize())
    }
}
