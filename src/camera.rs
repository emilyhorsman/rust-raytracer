use na::*;

use crate::ray::*;
use crate::types::*;

pub struct Camera {
    canvas_width: usize,
    canvas_height: usize,
    field_of_view_radians: Float,
    transform: Projective3<Float>,
}

impl Camera {
    /// Computes the size of a pixel in world space units.
    pub fn compute_pixel_size(&self) -> (Float, Float, Float) {
        let half_view = (self.field_of_view_radians / 2.0).tan();
        let aspect_ratio = self.canvas_width as f64 / self.canvas_height as f64;
        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        (
            half_width,
            half_height,
            half_width * 2.0 / self.canvas_width as f64,
        )
    }

    pub fn ray_for_pixel(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        let (half_width, half_height, pixel_size) = self.compute_pixel_size();
        // Center the pixel.
        let x_offset = (pixel_x as f64 + 0.5) * pixel_size;
        let y_offset = (pixel_y as f64 + 0.5) * pixel_size;

        let world_x = half_width - x_offset;
        let world_y = half_height - y_offset;

        let inv = self.transform.inverse();
        // Assumption: The camera and canvas are 1 unit apart with the canvas situated at
        // z = -1.
        let pixel = inv * Point3::new(world_x, world_y, -1.0);
        let origin = inv * Point3::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use approx::*;

    use super::*;

    #[test]
    fn it_computes_horizontal_canvas_pixel_size() {
        let c = Camera {
            canvas_width: 200,
            canvas_height: 125,
            field_of_view_radians: FRAC_PI_2,
            transform: Projective3::identity(),
        };
        assert_relative_eq!(c.compute_pixel_size().2, 0.01);
    }

    #[test]
    fn it_computes_vertical_canvas_pixel_size() {
        let c = Camera {
            canvas_width: 125,
            canvas_height: 200,
            field_of_view_radians: FRAC_PI_2,
            transform: Projective3::identity(),
        };
        assert_relative_eq!(c.compute_pixel_size().2, 0.01);
    }

    #[test]
    fn it_computes_ray_through_canvas_center() {
        let c = Camera {
            canvas_width: 201,
            canvas_height: 101,
            field_of_view_radians: FRAC_PI_2,
            transform: Projective3::identity(),
        };
        let r = c.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin, Point3::new(0.0, 0.0, 0.0));
        assert_relative_eq!(r.direction, Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn it_computes_ray_through_canvas_corner() {
        let c = Camera {
            canvas_width: 201,
            canvas_height: 101,
            field_of_view_radians: FRAC_PI_2,
            transform: Projective3::identity(),
        };
        let r = c.ray_for_pixel(0, 0);
        assert_relative_eq!(r.origin, Point3::new(0.0, 0.0, 0.0));
        assert!(relative_eq!(
            r.direction,
            Vector3::new(0.66519, 0.33259, -0.66851),
            epsilon = 1e-5
        ));
    }

    #[test]
    fn it_computes_ray_with_camera_transform() {
        let t = Rotation3::from_axis_angle(&Vector3::y_axis(), FRAC_PI_4)
            * Translation3::from(Vector3::new(0.0, -2.0, 5.0))
            * Projective3::identity();
        let c = Camera {
            canvas_width: 201,
            canvas_height: 101,
            field_of_view_radians: FRAC_PI_2,
            transform: t,
        };
        let r = c.ray_for_pixel(100, 50);
        assert_relative_eq!(r.origin, Point3::new(0.0, 2.0, -5.0));
        let k = FRAC_PI_4.sin();
        assert!(relative_eq!(
            r.direction,
            Vector3::new(k, 0.0, -k),
            epsilon = 1e-15
        ));
    }
}
