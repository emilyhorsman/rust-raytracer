use crate::vector::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub Vec3f);

impl Color {
    /// Produces a bitmap-compatible u8 array with members in [0, 255].
    pub fn to_u8_array(&self) -> [u8; 3] {
        let Color(Vec3f { x, y, z }) = self;
        [(x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8]
    }

    /// Apparently this is technically called the Hadamard or Schur product.
    pub fn mix(self, other: Self) -> Self {
        let Color(Vec3f {
            x: x1,
            y: y1,
            z: z1,
        }) = self;
        let Color(Vec3f {
            x: x2,
            y: y2,
            z: z2,
        }) = other;
        Color(Vec3f {
            x: x1 * x2,
            y: y1 * y2,
            z: z1 * z2,
        })
    }
}

pub type Image = Vec<Vec<Color>>;

#[cfg(test)]
mod tests {
    use super::Color;
    use crate::vector::Vec3f;

    #[test]
    fn it_mixes_with_hadamard_product() {
        let a = Color(Vec3f {
            x: 1.0,
            y: 0.2,
            z: 0.4,
        });
        let b = Color(Vec3f {
            x: 0.9,
            y: 1.0,
            z: 0.1,
        });
        let Color(u) = a.mix(b);
        assert!(u.approx_eq(&Vec3f {
            x: 0.9,
            y: 0.2,
            z: 0.04
        }));
    }
}
