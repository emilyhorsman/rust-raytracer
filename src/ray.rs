use crate::point::*;
use crate::vector::*;

pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    fn from_parameter(&self, t: Float) -> Point {
        Point(self.origin + self.direction * t)
    }
}
