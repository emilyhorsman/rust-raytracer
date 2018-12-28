use na::*;

use crate::color::Color;
use crate::intersections::*;
use crate::material::*;
use crate::ray::*;
use crate::shape::*;
use crate::types::*;

pub struct Plane {
    pub object_to_world_space: Projective3<Float>,
    pub material: Material,
}

impl Shape for Plane {
    fn intersection(&self, ray: &Ray) -> Option<Float> {
        let ray_transformation = self.object_to_world_space.inverse();
        ray_plane_intersection(&Ray {
            origin: ray_transformation * ray.origin,
            direction: ray_transformation * ray.direction,
        })
        .and_then(|t| if t >= EPSILON { Some(t) } else { None })
    }

    fn normal_at(&self, _: Point3f) -> Vec3f {
        self.object_to_world_space * Vector3::new(0.0, 1.0, 0.0)
    }

    fn color_at(&self, _: &Point3f) -> Color {
        self.material.color
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
