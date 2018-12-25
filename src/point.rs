use crate::vector::*;

#[derive(Add, Sub, Debug, PartialEq)]
pub struct Point(pub Vec3f);

#[inline]
pub fn p(x: Float, y: Float, z: Float) -> Point {
    Point(v(x, y, z))
}
