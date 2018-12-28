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
    pub fn matrix(&self) -> Projective3<Float> {
        let forward = (self.to - self.from).normalize();
        let up = self.up.normalize();
        let left = forward.cross(&up);
        // The given up vector does not need to be perfectly orthogonal.
        let true_up = left.cross(&forward);
        let mut orientation = Matrix4::identity();
        // TODO: Cleanup.
        orientation.set_row(0, &left.transpose().insert_column(3, 0.0));
        orientation.set_row(1, &true_up.transpose().insert_column(3, 0.0));
        orientation.set_row(2, &(-forward).transpose().insert_column(3, 0.0));

        let from_translation = (Point3::new(0.0, 0.0, 0.0) - self.from).insert_row(3, 1.0);
        let mut translation = Matrix4::identity();
        translation.set_column(3, &from_translation);

        Projective3::from_matrix_unchecked(orientation * translation)
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

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
        }
        .matrix();
        let mut correct = Matrix4::identity();
        correct.set_diagonal(&Vector4::new(-1.0, 1.0, -1.0, 1.0));
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
        }
        .matrix();
        let mut correct = Matrix4::identity();
        correct.set_column(3, &Vector4::new(0.0, 0.0, -8.0, 1.0));
        assert_relative_eq!(*m.matrix(), correct);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let m = ViewTransformation {
            from: Point3::new(1.0, 3.0, 2.0),
            to: Point3::new(4.0, -2.0, 8.0),
            up: Vector3::new(1.0, 1.0, 0.0),
        }
        .matrix();
        let correct = Matrix4::new(
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
        );
        assert!(relative_eq!(*m.matrix(), correct, epsilon = 1e-5));
    }
}
