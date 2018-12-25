use crate::ray::*;
use crate::types::*;

pub trait Shape {
    fn intersection(&self, ray: Ray) -> Option<Float>;
}
