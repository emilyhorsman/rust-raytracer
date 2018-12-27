use crate::types::*;

pub struct Ray {
    pub origin: Point3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn point_at(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

pub fn reflect(incoming: &Vec3f, surface_normal: &Vec3f) -> Vec3f {
    incoming - *surface_normal * 2.0 * incoming.dot(surface_normal)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use approx::assert_relative_eq;
    use na::{Point3, Vector3};

    use super::*;

    #[test]
    fn it_computes_ray_point_at_t() {
        let r = Ray {
            origin: Point3::new(2.0, 3.0, 4.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };
        assert_relative_eq!(r.point_at(0.0), Point3::new(2.0, 3.0, 4.0),);
        assert_relative_eq!(r.point_at(-1.0), Point3::new(1.0, 3.0, 4.0),);
    }

    #[test]
    fn it_reflects_simple_45_case() {
        assert_relative_eq!(
            reflect(&Vector3::new(1.0, -1.0, 0.0), &Vector3::new(0.0, 1.0, 0.0)),
            Vector3::new(1.0, 1.0, 0.0)
        );
    }

    #[test]
    fn it_reflects_off_slanted_surface() {
        let k = FRAC_PI_4.sin();
        assert_relative_eq!(
            reflect(&Vector3::new(0.0, -1.0, 0.0), &Vector3::new(k, k, 0.0)),
            Vector3::new(1.0, 0.0, 0.0)
        );
    }
}
