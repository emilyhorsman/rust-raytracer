use na::*;

use crate::color::*;
use crate::types::*;

pub trait Pattern {
    fn color_at(&self, point: Point3f) -> Color;
}

pub struct SolidPattern(pub Color);

impl Pattern for SolidPattern {
    fn color_at(&self, _: Point3f) -> Color {
        self.0
    }
}

pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl Pattern for StripePattern {
    fn color_at(&self, point: Point3f) -> Color {
        let pattern_point = Similarity3::from_scaling(10.0) * point;
        if (pattern_point.x as i64) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
