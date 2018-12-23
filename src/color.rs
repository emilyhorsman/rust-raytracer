use crate::vector::*;

pub struct Color(pub Vec3f);

impl Color {
    /// Produces a bitmap-compatible u8 array with members in [0, 255].
    pub fn to_u8_array(&self) -> [u8; 3] {
        let Color(Vec3f { x, y, z }) = self;
        [(x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8]
    }
}

pub type Image = Vec<Vec<Color>>;
