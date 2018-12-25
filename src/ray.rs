use na::{Point3, Vector3};

use crate::types::*;

pub struct Ray {
    pub origin: Point3f,
    pub direction: Vec3f,
}

impl Ray {
    fn from_parameter(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn it_computes_ray_from_parameter_t() {
        let r = Ray {
            origin: Point3::new(2.0, 3.0, 4.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };
        assert_relative_eq!(r.from_parameter(0.0), Point3::new(2.0, 3.0, 4.0),);
        assert_relative_eq!(r.from_parameter(-1.0), Point3::new(1.0, 3.0, 4.0),);
    }
}
