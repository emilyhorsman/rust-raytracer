use na::*;

use crate::intersections::*;
use crate::ray::*;
use crate::shape::*;
use crate::transformation::*;
use crate::types::*;

pub struct Sphere {
    pub object_to_world_space: Projective3<Float>,
}

impl From<Transformation> for Sphere {
    fn from(t: Transformation) -> Self {
        Self {
            object_to_world_space: t.matrix(),
        }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Float> {
        let ray_transformation = self.object_to_world_space.inverse();
        ray_sphere_intersection(Ray {
            origin: ray_transformation * ray.origin,
            direction: ray_transformation * ray.direction,
        })
        .map(|(a, b)| a.min(b))
    }

    fn normal_at(&self, world_point: Point3f) -> Vec3f {
        let object_point = self.object_to_world_space.inverse() * world_point;
        let object_normal = object_point - Point3::new(0.0, 0.0, 0.0);
        // I don't really understand why this must be the inverse transpose.
        let mut world_normal = self.object_to_world_space.inverse().matrix().transpose()
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
        let sphere = Sphere::from(Transformation::new());
        let k = (3.0).sqrt() / 3.0;
        assert_relative_eq!(
            sphere.normal_at(Point3::new(k, k, k)),
            Vector3::new(k, k, k)
        );
    }

    #[test]
    fn it_computes_normal_for_translated_sphere() {
        let sphere = Sphere::from(Transformation::new().translate(0.0, 1.0, 0.0));
        let k = (std::f64::consts::PI / 4.0).sin();
        assert_relative_eq!(
            sphere.normal_at(Point3::new(0.0, k + 1.0, -k)),
            Vector3::new(0.0, k, -k)
        );
    }

    #[test]
    fn it_computes_normal_for_scaled_sphere() {
        let t = Transformation::new()
            .scale(1.0, 0.5, 1.0)
            .rotate_z(std::f64::consts::PI / 5.0);
        let sphere = Sphere::from(t);
        assert_relative_eq!(
            sphere.normal_at(Point3::new(0.0, (2.0).sqrt() / 2.0, -(2.0).sqrt() / 2.0)),
            Vector3::new(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }
}
