use na::*;

use crate::types::*;

pub struct Transformation {
    rotate_x: Float,
    rotate_y: Float,
    rotate_z: Float,
    scale: Vector4<Float>,
    translate: Vector3<Float>,
}

macro_rules! make_rotate_fn {
    ( $name:ident ) => {
        pub fn $name(&self, angle: Float) -> Self {
            Self {
                $name: angle,
                ..*self
            }
        }
    }
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
            scale: Vector4::new(1.0, 1.0, 1.0, 1.0),
            translate: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    make_rotate_fn!(rotate_x);
    make_rotate_fn!(rotate_y);
    make_rotate_fn!(rotate_z);

    pub fn scale(&self, x: Float, y: Float, z: Float) -> Self {
        Self {
            scale: Vector4::new(x, y, z, 1.0),
            ..*self
        }
    }

    pub fn translate(&self, x: Float, y: Float, z: Float) -> Self {
        Self {
            translate: Vector3::new(x, y, z),
            ..*self
        }
    }

    pub fn matrix(&self) -> Projective3<Float> {
        let non_uniform_scaling: Affine3<_> =
            Affine3::from_matrix_unchecked(Matrix::from_diagonal(&self.scale));
        let m = non_uniform_scaling
            * Rotation3::from_axis_angle(&Vector3::x_axis(), self.rotate_x)
            * Rotation3::from_axis_angle(&Vector3::y_axis(), self.rotate_y)
            * Rotation3::from_axis_angle(&Vector3::z_axis(), self.rotate_z)
            * Translation3::from(self.translate);
        m.set_category::<TProjective>()
    }
}
