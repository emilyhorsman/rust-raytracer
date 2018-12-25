use na::{Point3, Similarity3, Translation3};

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
        let object_transformation = Similarity3::from_scaling(self.radius)
            * Translation3::from(self.origin - Point3::new(0.0, 0.0, 0.0));
        let ray_transformation = object_transformation.inverse();
        ray_sphere_intersection(Ray {
            origin: ray_transformation * ray.origin,
            direction: ray_transformation * ray.direction,
        })
        .map(|(a, b)| a.min(b))
    }
}
