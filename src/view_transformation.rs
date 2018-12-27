use na::*;

use crate::types::*;

pub struct ViewTransformation {
    from: Point3f,
    to: Point3f,
    up: Vec3f,
}

impl Default for ViewTransformation {
    fn default() -> Self {
        Self {
            from: Point3::new(0.0, 0.0, 0.0),
            to: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }
}

impl ViewTransformation {
    pub fn matrix(&self) -> Projective3<Float> {
        Projective3::identity()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn default_transformation_is_identity_matrix() {
        let transform: ViewTransformation = Default::default();
        let m = transform.matrix();
        assert_relative_eq!(*m.matrix(), Matrix4::identity());
    }

    #[test]
    fn looking_in_positive_z_direction() {
        let m = ViewTransformation {
            to: Point3::new(0.0, 0.0, 1.0),
            ..Default::default()
        }.matrix();
        let mut correct = Matrix4::identity();
        correct.set_diagonal(&Vector4::new(1.0, -1.0, 1.0, 1.0));
        assert_relative_eq!(*m.matrix(), correct);
    }

    #[test]
    fn translates_the_world() {
        // An eye positioned at (0, 0, 8) should translate the world -8 units on the
        // z-axis.
        let m = ViewTransformation {
            from: Point3::new(0.0, 0.0, 8.0),
            to: Point3::new(0.0, 0.0, 0.0),
            ..Default::default()
        }.matrix();
        let mut correct = Matrix4::identity();
        correct.set_column(3, &Vector4::new(0.0, 0.0, -8.0, 1.0));
        assert_relative_eq!(*m.matrix(), correct);
    }
}
