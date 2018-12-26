use crate::ray::*;
use crate::types::*;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> Option<Float>;
    fn normal_at(&self, world_point: Point3f) -> Vec3f;
}
