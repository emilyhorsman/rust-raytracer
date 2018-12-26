use na::*;

use crate::intersections::*;
use crate::ray::*;
use crate::shape::*;
use crate::types::*;

pub struct Sphere {
    pub origin: Point3f,
    pub radius: Float,
}

impl Sphere {
    fn object_to_world_space(&self) -> Projective3<Float> {
        Similarity3::from_scaling(self.radius)
            * Translation3::from(self.origin - Point3::new(0.0, 0.0, 0.0))
            * Projective3::identity()
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Float> {
        let ray_transformation = self.object_to_world_space().inverse();
        ray_sphere_intersection(Ray {
            origin: ray_transformation * ray.origin,
            direction: ray_transformation * ray.direction,
        })
        .map(|(a, b)| a.min(b))
    }

    fn normal_at(&self, world_point: Point3f) -> Vec3f {
        let object_point = self.object_to_world_space().inverse() * world_point;
        let object_normal = object_point - Point3::new(0.0, 0.0, 0.0);
        // I don't really understand why this must be the inverse transpose.
        let mut world_normal = self.object_to_world_space().inverse().matrix().transpose()
            * Vector4::new(object_normal.x, object_normal.y, object_normal.z, 0.0);
        world_normal.w = 0.0;
        world_normal.normalize().xyz()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn it_computes_normal() {
        let s = Sphere {
            origin: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let k = (3.0).sqrt() / 3.0;
        assert_relative_eq!(s.normal_at(Point3::new(k, k, k)), Vector3::new(k, k, k));
    }

    #[test]
    fn it_computes_normal_for_translated_sphere() {
        let s = Sphere {
            origin: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        };
        let k = (std::f64::consts::PI / 4.0).sin();
        assert_relative_eq!(
            s.normal_at(Point3::new(0.0, k + 1.0, -k)),
            Vector3::new(0.0, k, -k)
        );
    }
}
