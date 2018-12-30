use std::f64::consts::*;

use na::*;

use crate::color::Color;
use crate::intersections::*;
use crate::material::*;
use crate::model_transformation::*;
use crate::ray::*;
use crate::shape::*;
use crate::types::*;

pub struct Plane {
    pub object_to_world_space: Projective3<Float>,
    pub material: Material,
}

impl Plane {
    pub fn floor(y: Float, mat: Material) -> Self {
        Self {
            object_to_world_space: ModelTransformation::new().translate(0.0, y, 0.0).matrix(),
            material: mat,
        }
    }

    pub fn ceiling(y: Float, mat: Material) -> Self {
        Self {
            object_to_world_space: ModelTransformation::new()
                .translate(0.0, y, 0.0)
                .rotate_z(PI)
                .matrix(),
            material: mat,
        }
    }

    pub fn right_wall(x: Float, mat: Material) -> Self {
        Self {
            object_to_world_space: ModelTransformation::new()
                .translate(x, 0.0, 0.0)
                .rotate_z(FRAC_PI_2)
                .matrix(),
            material: mat,
        }
    }

    pub fn left_wall(x: Float, mat: Material) -> Self {
        Self {
            object_to_world_space: ModelTransformation::new()
                .translate(x, 0.0, 0.0)
                .rotate_z(-FRAC_PI_2)
                .matrix(),
            material: mat,
        }
    }

    pub fn back_wall(z: Float, mat: Material) -> Self {
        Self {
            object_to_world_space: ModelTransformation::new()
                .translate(0.0, 0.0, z)
                .rotate_x(-FRAC_PI_2)
                .matrix(),
            material: mat,
        }
    }
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
        self.object_to_world_space * Vector3::y()
    }

    fn color_at(&self, _: &Point3f) -> Color {
        self.material.color
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn it_computes_floor_world_normal() {
        assert_relative_eq!(
            Plane::floor(0.0, Default::default()).normal_at(Point3::new(0.0, 0.0, 0.0)),
            Vector3::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn it_computes_right_side_wall_world_normal() {
        assert_relative_eq!(
            Plane::right_wall(0.0, Default::default()).normal_at(Point3::new(0.0, 0.0, 0.0)),
            Vector3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn it_computes_left_side_wall_world_normal() {
        assert_relative_eq!(
            Plane::left_wall(0.0, Default::default()).normal_at(Point3::new(0.0, 0.0, 0.0)),
            Vector3::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn it_computes_ceiling_world_normal() {
        assert_relative_eq!(
            Plane::ceiling(0.0, Default::default()).normal_at(Point3::new(0.0, 0.0, 0.0)),
            Vector3::new(0.0, -1.0, 0.0)
        );
    }

    #[test]
    fn it_computes_back_wall_world_normal() {
        assert_relative_eq!(
            Plane::back_wall(0.0, Default::default()).normal_at(Point3::new(0.0, 0.0, 0.0)),
            Vector3::new(0.0, 0.0, -1.0)
        );
    }
}
