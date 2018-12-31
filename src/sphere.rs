use std::f64::consts::*;

use na::*;

use crate::color::*;
use crate::intersections::*;
use crate::material::*;
use crate::model_transformation::*;
use crate::ray::*;
use crate::shape::*;
use crate::types::*;

pub struct Sphere {
    pub object_to_world_space: Projective3<Float>,
    pub material: Material,
}

impl From<ModelTransformation> for Sphere {
    fn from(t: ModelTransformation) -> Self {
        Self {
            object_to_world_space: t.matrix(),
            material: Material::default(),
        }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Float> {
        let ray_transformation = self.object_to_world_space.inverse();
        ray_sphere_intersection(&Ray {
            origin: ray_transformation * ray.origin,
            direction: ray_transformation * ray.direction,
        })
        .and_then(|(a, b)| {
            // Pick the minimum intersection not behind the ray.
            if a >= 0.0 && a < b {
                Some(a)
            } else if b >= 0.0 {
                Some(b)
            } else {
                None
            }
        })
    }

    fn normal_at(&self, world_point: Point3f) -> Vec3f {
        let object_point = self.object_to_world_space.inverse() * world_point;
        let object_normal = object_point - Point3::new(0.0, 0.0, 0.0);
        // We want to invert the scaling component of the object to world space transformation
        // applied the normal direction vector. The inverse of any rotation matrix is its
        // transpose. Any matrix composed of rotation and scaling matrices can be orthogonally
        // diagonalized meaning that the transpose(inv(mat)) will invert the diagonal matrix
        // (the scaling component) and leave the rotation matrices.
        let mut world_normal = self
            .object_to_world_space
            .inverse()
            .to_homogeneous()
            .transpose()
            * Vector4::new(object_normal.x, object_normal.y, object_normal.z, 0.0);
        // However, matrices with a translation component muck up the orthogonal
        // diagonalization a bit, so we zero the w component of the normal before
        // normalizing.
        world_normal.w = 0.0;
        world_normal.normalize().xyz()
    }

    fn color_at(&self, world_point: &Point3f) -> Color {
        let object_point = self.object_to_world_space.inverse() * world_point;
        // TODO: UV mapping here does not support general ellipsoids, only spheres
        // (w/ radius 1 on all axes).
        let theta = (-object_point.z).atan2(object_point.x);
        let u = (theta + PI) / (2.0 * PI);
        let phi = (-object_point.y).acos();
        let v = phi / PI;
        let uv_mapped_point = Point3::new(u, 0.0, v);
        self.material.color.color_at(uv_mapped_point)
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_only_returns_non_negative_intersection() {
        let sphere = Sphere::from(ModelTransformation::new());
        let r = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert_relative_eq!(sphere.intersection(&r).unwrap(), 1.0);
    }

    #[test]
    fn it_computes_normal() {
        let sphere = Sphere::from(ModelTransformation::new());
        let k = (3.0).sqrt() / 3.0;
        assert_relative_eq!(
            sphere.normal_at(Point3::new(k, k, k)),
            Vector3::new(k, k, k)
        );
    }

    #[test]
    fn it_computes_normal_for_translated_sphere() {
        let sphere = Sphere::from(ModelTransformation::new().translate(0.0, 1.0, 0.0));
        let k = FRAC_PI_4.sin();
        assert_relative_eq!(
            sphere.normal_at(Point3::new(0.0, k + 1.0, -k)),
            Vector3::new(0.0, k, -k)
        );
    }

    #[test]
    fn it_computes_normal_for_scaled_sphere() {
        let t = Affine3::from_matrix_unchecked(Matrix::from_diagonal(&Vector4::new(
            1.0, 0.5, 1.0, 1.0,
        ))) * Rotation3::from_axis_angle(&Vector3::z_axis(), PI / 5.0)
            * Projective3::identity();
        let sphere = Sphere {
            object_to_world_space: t,
            material: Material::default(),
        };
        assert_relative_eq!(
            sphere.normal_at(Point3::new(0.0, (2.0).sqrt() / 2.0, -(2.0).sqrt() / 2.0)),
            Vector3::new(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }
}
