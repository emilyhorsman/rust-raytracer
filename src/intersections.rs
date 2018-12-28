use na::Point3;

use crate::ray::*;
use crate::types::*;

pub fn ray_sphere_intersection(ray: &Ray) -> Option<(Float, Float)> {
    // Assume the sphere is of radius 1 and at the origin.
    let sphere_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);
    let a = ray.direction.norm_squared();
    let b = 2.0 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.norm_squared() - 1.0;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let d = discriminant.sqrt();
    Some(((-b - d) / (2.0 * a), (-b + d) / (2.0 * a)))
}

pub fn ray_plane_intersection(ray: &Ray) -> Option<Float> {
    if ray.direction.y.abs() <= EPSILON {
        return None;
    }

    Some(-ray.origin.y / ray.direction.y)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use na::{Point3, Vector3};

    use super::*;

    #[test]
    fn it_computes_sphere_intersection() {
        let r = Ray {
            origin: Point3::new(0.0, 0.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let (t1, t2) = ray_sphere_intersection(&r).unwrap();
        assert_relative_eq!(t1, 4.0);
        assert_relative_eq!(t2, 6.0);
    }

    #[test]
    fn it_computes_sphere_intersection_at_tangent() {
        let r = Ray {
            origin: Point3::new(0.0, 1.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let (t1, t2) = ray_sphere_intersection(&r).unwrap();
        assert_relative_eq!(t1, 5.0);
        assert_relative_eq!(t2, 5.0);
    }

    #[test]
    fn it_misses_sphere() {
        let r = Ray {
            origin: Point3::new(0.0, 2.0, -5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(ray_sphere_intersection(&r).is_none());
    }

    #[test]
    fn it_computes_sphere_intersection_from_inside_sphere() {
        let r = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let (t1, t2) = ray_sphere_intersection(&r).unwrap();
        assert_relative_eq!(t1, -1.0);
        assert_relative_eq!(t2, 1.0);
    }

    #[test]
    fn it_computes_sphere_intersection_from_in_front_of_sphere() {
        let r = Ray {
            origin: Point3::new(0.0, 0.0, 5.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        let (t1, t2) = ray_sphere_intersection(&r).unwrap();
        assert_relative_eq!(t1, -6.0);
        assert_relative_eq!(t2, -4.0);
    }

    #[test]
    fn it_does_not_intersect_when_parallel_with_plane() {
        let r = Ray {
            origin: Point3::new(0.0, 10.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(ray_plane_intersection(&r).is_none());
    }

    #[test]
    fn it_does_not_intersect_when_coplanar() {
        let r = Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(ray_plane_intersection(&r).is_none());
    }

    #[test]
    fn it_intersects_plane_from_above() {
        let r = Ray {
            origin: Point3::new(0.0, 1.0, 0.0),
            direction: Vector3::new(0.0, -1.0, 0.0),
        };
        let t = ray_plane_intersection(&r).unwrap();
        assert_relative_eq!(t, 1.0);
    }

    #[test]
    fn it_intersects_plane_from_below() {
        let r = Ray {
            origin: Point3::new(0.0, -1.0, 0.0),
            direction: Vector3::new(0.0, 1.0, 0.0),
        };
        let t = ray_plane_intersection(&r).unwrap();
        assert_relative_eq!(t, 1.0);
    }
}
