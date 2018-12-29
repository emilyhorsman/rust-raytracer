use na::*;

use crate::types::*;

pub struct ViewTransformation {
    pub from: Point3f,
    pub to: Point3f,
    pub up: Vec3f,
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
    pub fn matrix(&self) -> Isometry3<Float> {
        Isometry3::look_at_rh(&self.from, &self.to, &self.up)
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn default_transformation_is_identity_matrix() {
        let transform: ViewTransformation = Default::default();
        assert_relative_eq!(transform.matrix().to_homogeneous(), Matrix4::identity());
    }

    #[test]
    fn looking_in_positive_z_direction() {
        let m = ViewTransformation {
            to: Point3::new(0.0, 0.0, 1.0),
            ..Default::default()
        };
        let mut correct = Matrix4::identity();
        correct.set_diagonal(&Vector4::new(-1.0, 1.0, -1.0, 1.0));
        assert_relative_eq!(m.matrix().to_homogeneous(), correct);
    }

    #[test]
    fn translates_the_world() {
        // An eye positioned at (0, 0, 8) should translate the world -8 units on the
        // z-axis.
        let m = ViewTransformation {
            from: Point3::new(0.0, 0.0, 8.0),
            to: Point3::new(0.0, 0.0, 0.0),
            ..Default::default()
        };
        let mut correct = Matrix4::identity();
        correct.set_column(3, &Vector4::new(0.0, 0.0, -8.0, 1.0));
        assert_relative_eq!(m.matrix().to_homogeneous(), correct);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let m = ViewTransformation {
            from: Point3::new(1.0, 3.0, 2.0),
            to: Point3::new(4.0, -2.0, 8.0),
            up: Vector3::new(1.0, 1.0, 0.0),
        };
        let correct = Matrix4::new(
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
        );
        assert!(relative_eq!(m.matrix().to_homogeneous(), correct, epsilon = 0.05));
    }
}
