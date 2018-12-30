use na::Vector3;

use crate::types::*;

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Mul)]
pub struct Color(pub Vec3f);

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Color(Vector3::new(r, g, b))
    }

    /// Produces a bitmap-compatible u8 array with members in [0, 255].
    pub fn to_u8_array(&self) -> [u8; 3] {
        let Color(v) = self;
        [
            (v.x * 255.0).round() as u8,
            (v.y * 255.0).round() as u8,
            (v.z * 255.0).round() as u8,
        ]
    }

    /// Apparently this is technically called the Hadamard or Schur product.
    pub fn mix(self, other: Self) -> Self {
        let Color(u) = self;
        let Color(v) = other;
        Color(u.component_mul(&v))
    }

    pub fn clamp(self) -> Self {
        let Color(v) = self;
        Self::new(v.x.min(1.0), v.y.min(1.0), v.z.min(1.0))
    }
}

pub type Image = Vec<Vec<Color>>;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn it_mixes_with_hadamard_product() {
        let a = Color(Vector3::new(1.0, 0.2, 0.4));
        let b = Color(Vector3::new(0.9, 1.0, 0.1));
        let Color(u) = a.mix(b);
        assert_relative_eq!(u, Vector3::new(0.9, 0.2, 0.04));
    }
}
