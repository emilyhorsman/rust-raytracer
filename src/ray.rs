use crate::point::*;
use crate::vector::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3f,
}

impl Ray {
    fn from_parameter(&self, t: Float) -> Point {
        let Point(o) = self.origin;
        Point(o + self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::point::Point;
    use crate::vector::Vec3f;

    #[test]
    fn it_computes_ray_from_parameter_t() {
        let r = Ray {
            origin: Point(Vec3f {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }),
            direction: Vec3f {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        assert_eq!(
            r.from_parameter(0.0),
            Point(Vec3f {
                x: 2.0,
                y: 3.0,
                z: 4.0
            })
        );
        assert_eq!(
            r.from_parameter(-1.0),
            Point(Vec3f {
                x: 1.0,
                y: 3.0,
                z: 4.0
            })
        );
    }
}
