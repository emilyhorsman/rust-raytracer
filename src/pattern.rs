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
    pub object_to_pattern_space: Projective3<Float>,
}

impl Pattern for StripePattern {
    fn color_at(&self, point: Point3f) -> Color {
        let pattern_point = self.object_to_pattern_space * point;
        if (pattern_point.x as i64) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
