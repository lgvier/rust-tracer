use std::time::Instant;

use indicatif::HumanDuration;
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{
    canvas::Canvas,
    color::Color,
    matrix::{Matrix, IDENTITY_MATRIX},
    point, ray,
    ray::Ray,
    world::World,
};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pub pixel_size: f64,
    transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.) / hsize as f64;
        Self {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        self.ray_for_pixel_with_offset(px, py, 0.5, 0.5)
    }

    fn ray_for_pixel_with_offset(&self, px: usize, py: usize, ox: f64, oy: f64) -> Ray {
        // offset from edge of the canvas to pixel's center
        let xoffset = (px as f64 + ox) * self.pixel_size;
        let yoffset = (py as f64 + oy) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        // camera looks toward -z, so +x is to the left
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction
        // remember that the canvas is at z = -1
        let transform_inverse = self.transform.inverse().unwrap();
        let pixel = transform_inverse * point!(world_x, world_y, -1);
        let origin = transform_inverse * point!(0, 0, 0);
        let direction = (pixel - origin).normalize();
        ray!(origin, direction)
    }

    pub fn render(&self, world: &World, antialiasing: bool) -> Canvas {
        let start = Instant::now();
        let progress_bar = if self.vsize > 50 {
            println!("Rendering...");
            Some(ProgressBar::new(self.vsize as u64))
        } else {
            None
        };

        let pixels = (0..self.vsize)
            .into_par_iter()
            .flat_map(|y| {
                let row = (0..self.hsize)
                    .into_iter()
                    .map(|x| (x, y, self.color_at(world, x, y, antialiasing)))
                    .collect::<Vec<_>>();
                if let Some(pb) = &progress_bar {
                    pb.inc(1);
                }
                row
            })
            .collect::<Vec<_>>();

        let mut image = Canvas::new(self.hsize, self.vsize);
        pixels
            .iter()
            .for_each(|(x, y, color)| image.write_pixel(*x, *y, *color));

        if let Some(pb) = &progress_bar {
            pb.finish();
            println!("Completed in {}", HumanDuration(start.elapsed()));
        }
        image
    }

    fn color_at(&self, world: &World, x: usize, y: usize, antialiasing: bool) -> Color {
        let color_center = world.color_at(&self.ray_for_pixel(x, y));
        if antialiasing {
            let mut color_sum = color_center;
            for &(ox, oy) in &[(0.20, 0.20), (0.80, 0.20), (0.20, 0.80), (0.80, 0.80)] {
                color_sum =
                    color_sum + world.color_at(&self.ray_for_pixel_with_offset(x, y, ox, oy));
            }
            color_sum / 5.
        } else {
            color_center
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::PI;

    use crate::{approx_eq, color, vector};

    #[test]
    fn ctor() {
        let hsize = 100;
        let vsize = 120;
        let field_of_view = PI / 2.;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(hsize, c.hsize);
        assert_eq!(vsize, c.vsize);
        assert_eq!(field_of_view, c.field_of_view);
    }

    #[test]
    fn pixel_size() {
        let c = Camera::new(200, 125, PI / 2.);
        assert!(approx_eq(0.01, c.pixel_size));
    }

    #[test]
    fn ray_trough_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(point!(0, 0, 0), r.origin);
        assert_eq!(vector!(0, 0, -1), r.direction);
    }

    #[test]
    fn ray_trough_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(point!(0, 0, 0), r.origin);
        assert_eq!(vector!(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.set_transform(Matrix::rotation_y(PI / 4.) * Matrix::translation(0, -2, 5));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(point!(0, 2, -5), r.origin);
        assert_eq!(vector!(2f64.sqrt() / 2., 0, -2f64.sqrt() / 2.), r.direction);
    }

    #[test]
    fn render_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = point!(0, 0, -5);
        let to = point!(0, 0, 0);
        let up = point!(0, 1, 0);
        c.set_transform(Matrix::view_transform(from, to, up));
        let image = c.render(&w, false);
        assert_eq!(color!(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5));
    }
}
