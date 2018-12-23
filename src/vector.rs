use std::ops::{Add, Mul, Sub};

use crate::nearly_eq::NearlyEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Float = f64;

pub type Vec3f = Vec3<Float>;

impl Vec3f {
    pub fn norm(self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let n = self.norm();
        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl<T> Vec3<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<A, B, C: NearlyEq<A, B>> NearlyEq<Vec3<A>, B> for Vec3<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Vec3<A>, eps: &B) -> bool {
        nearly_eq::NearlyEq::eq(&self.x, &other.x, eps)
            && nearly_eq::NearlyEq::eq(&self.y, &other.y, eps)
            && nearly_eq::NearlyEq::eq(&self.z, &other.z, eps)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn it_computes_cross_product() {
        let u = Vec3 { x: 1, y: 2, z: 3 };
        let v = Vec3 { x: 2, y: 3, z: 4 };
        assert_eq!(u.cross(v), Vec3 { x: -1, y: 2, z: -1 });
        assert_eq!(v.cross(u), Vec3 { x: 1, y: -2, z: 1 });
    }

    #[test]
    fn it_computes_dot_product() {
        let u = Vec3 { x: 1, y: 2, z: 3 };
        let v = Vec3 { x: 2, y: 3, z: 4 };
        assert_eq!(u.dot(v), 20);
    }
}
