use crate::point::*;
use crate::vector::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3f,
}

impl Ray {
    fn from_parameter(&self, t: Float) -> Point {
        let Point(o) = self.origin;
        Point(o + self.direction * t)
    }
}
