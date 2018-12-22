use std::ops::{Add,Sub,Mul};

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub type Vec3f = Vec3<f64>;

impl<T: Add<Output=T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Add<Output=T>+Mul<Output=T>> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Copy+Mul<Output=T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}
