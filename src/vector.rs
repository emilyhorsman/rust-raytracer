use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Float = f64;

pub type Vec3f = Vec3<Float>;

impl Vec3f {
    pub fn norm(&self) -> Float {
        (self * self).sqrt()
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        self.x /= n;
        self.y /= n;
        self.z /= n;
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

impl<T: Add<Output = T> + Mul<Output = T>> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<'a, 'b, T> Mul<&'b Vec3<T>> for &'a Vec3<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = T;

    fn mul(self, other: &'b Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
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
