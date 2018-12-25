use crate::intersections::*;
use crate::ray::*;
use crate::shape::*;
use crate::types::*;

pub struct Sphere {
    pub origin: Point3f,
    pub radius: Float,
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Float> {
        ray_sphere_intersection(ray).map(|(a, b)| a.min(b))
    }
}
