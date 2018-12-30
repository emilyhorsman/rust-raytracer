use na::{Point3, Vector3};

pub type Float = f64;

pub type Vec3f = Vector3<Float>;

pub type Point3f = Point3<Float>;

pub const EPSILON: Float = std::f64::EPSILON;

pub const BIAS: Float = 1e-13;
