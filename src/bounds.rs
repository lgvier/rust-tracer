use core::ops::Add;
use std::f64::INFINITY;

use crate::{matrix::Matrix, point, ray::Ray, tuple::Tuple, EPSILON};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Tuple,
    pub max: Tuple,
}

impl BoundingBox {
    pub fn new(min: Tuple, max: Tuple) -> BoundingBox {
        BoundingBox { min, max }
    }

    pub fn empty() -> BoundingBox {
        BoundingBox::new(
            Tuple::point(INFINITY, INFINITY, INFINITY),
            Tuple::point(-INFINITY, -INFINITY, -INFINITY),
        )
    }

    pub fn contains_point(&self, p: Tuple) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    pub fn contains_box(&self, other: BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    pub fn transform(&self, matrix: Matrix) -> BoundingBox {
        BoundingBox::empty()
            + (matrix * self.min)
            + (matrix * point!(self.min.x, self.min.y, self.max.z))
            + (matrix * point!(self.min.x, self.max.y, self.min.z))
            + (matrix * point!(self.min.x, self.max.y, self.max.z))
            + (matrix * point!(self.max.x, self.min.y, self.min.z))
            + (matrix * point!(self.max.x, self.min.y, self.max.z))
            + (matrix * point!(self.max.x, self.max.y, self.min.z))
            + (matrix * self.max)
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) =
            BoundingBox::check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) =
            BoundingBox::check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) =
            BoundingBox::check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);
        let tmin = xtmin.max(ytmin.max(ztmin));
        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmin > tmax {
            // miss
            false
        } else {
            true
        }
    }

    // Reused in cube.rs
    pub fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * INFINITY, tmax_numerator * INFINITY)
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn split(&self) -> (Self, Self) {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;

        let greatest = dx.max(dy.max(dz));

        let Tuple {
            x: mut x0,
            y: mut y0,
            z: mut z0,
            ..
        } = self.min;
        let Tuple {
            x: mut x1,
            y: mut y1,
            z: mut z1,
            ..
        } = self.max;

        if greatest == dx {
            x0 = x0 + dx / 2.0;
            x1 = x0;
        } else if greatest == dy {
            y0 = y0 + dy / 2.0;
            y1 = y0;
        } else {
            z0 = z0 + dz / 2.0;
            z1 = z0;
        }

        let mid_min = point!(x0, y0, z0);
        let mid_max = point!(x1, y1, z1);

        (
            BoundingBox::new(self.min, mid_max),
            BoundingBox::new(mid_min, self.max),
        )
    }
}

impl Add<BoundingBox> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, other: Self) -> Self {
        BoundingBox::new(
            Tuple::point(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Tuple::point(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }
}

impl Add<Tuple> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, other: Tuple) -> Self {
        BoundingBox::new(
            Tuple::point(
                self.min.x.min(other.x),
                self.min.y.min(other.y),
                self.min.z.min(other.z),
            ),
            Tuple::point(
                self.max.x.max(other.x),
                self.max.y.max(other.y),
                self.max.z.max(other.z),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{ray, vector};

    #[test]
    fn add_points() {
        let b = BoundingBox::empty() + point!(-5, 2, 0) + point!(7, 0, -3);
        assert_eq!(point!(-5, 0, -3), b.min);
        assert_eq!(point!(7, 2, 0), b.max);
    }
    #[test]
    fn add_box() {
        let b1 = BoundingBox::new(point!(-5, 2, 0), point!(7, 4, 4));
        let b2 = BoundingBox::new(point!(8, -7, -2), point!(14, 2, 8));
        let result = b1 + b2;
        assert_eq!(point!(-5, -7, -2), result.min);
        assert_eq!(point!(14, 4, 8), result.max);
    }

    #[test]
    fn contains_point() {
        let b = BoundingBox::new(point!(5, -2, 0), point!(14, 4, 7));
        for &(point, result) in &[
            (point!(5, -2, 0), true),
            (point!(11, 4, 7), true),
            (point!(8, 1, 3), true),
            (point!(3, 0, 3), false),
            (point!(8, -4, 3), false),
            (point!(8, 1, -1), false),
            // (point!(13, 1, 3), false),
            (point!(8, 5, 3), false),
            (point!(8, 1, 8), false),
        ] {
            assert_eq!(result, b.contains_point(point), "contains {:?}", point);
        }
    }

    #[test]
    fn contains_box() {
        let b = BoundingBox::new(point!(5, -2, 0), point!(14, 4, 7));
        for &(other, result) in &[
            (BoundingBox::new(point!(5, -2, 0), point!(11, 4, 7)), true),
            (BoundingBox::new(point!(6, -1, 1), point!(10, 3, 6)), true),
            (BoundingBox::new(point!(4, -3, -1), point!(10, 3, 6)), false),
            (BoundingBox::new(point!(6, -1, 1), point!(12, 5, 8)), false),
        ] {
            assert_eq!(result, b.contains_box(other), "contains {:?}", other);
        }
    }

    #[test]
    fn transform() {
        let b = BoundingBox::new(point!(-1, -1, -1), point!(1, 1, 1));
        let matrix = Matrix::rotation_x(PI / 4.) * Matrix::rotation_y(PI / 4.);

        let transformed = b.transform(matrix);
        assert_eq!(point!(-1.41421, -1.707106, -1.707106), transformed.min);
        assert_eq!(point!(1.41421, 1.707106, 1.707106), transformed.max);
    }

    #[test]
    fn intersecting_with_a_bounding_box_at_the_origin() {
        let bb = BoundingBox::new(point!(-1, -1, -1), point!(1, 1, 1));
        for &(origin, direction, expected) in &[
            (point!(5, 0.5, 0), vector!(-1, 0, 0), true),
            (point!(-5, 0.5, 0), vector!(1, 0, 0), true),
            (point!(0.5, 5, 0), vector!(0, -1, 0), true),
            (point!(0.5, -5, 0), vector!(0, 1, 0), true),
            (point!(0.5, 0, 5), vector!(0, 0, -1), true),
            (point!(0.5, 0, -5), vector!(0, 0, 1), true),
            (point!(0, 0.5, 0), vector!(0, 0, 1), true),
            (point!(-2, 0, 0), vector!(2, 4, 6), false),
            (point!(0, -2, 0), vector!(6, 2, 4), false),
            (point!(0, 0, -2), vector!(4, 6, 2), false),
            (point!(2, 0, 2), vector!(0, 0, -1), false),
            (point!(0, 2, 2), vector!(0, -1, 0), false),
            (point!(2, 2, 0), vector!(-1, 0, 0), false),
        ] {
            let r = ray!(origin, direction.normalize());
            assert_eq!(expected, bb.intersects(&r));
        }
    }

    #[test]
    fn intersecting_with_a_non_cubic_bounding_box() {
        let bb = BoundingBox::new(point!(5, -2, 0), point!(11, 4, 7));
        for &(origin, direction, expected) in &[
            (point!(15, 1, 2), vector!(-1, 0, 0), true),
            (point!(-5, -1, 4), vector!(1, 0, 0), true),
            (point!(7, 6, 5), vector!(0, -1, 0), true),
            (point!(9, -5, 6), vector!(0, 1, 0), true),
            (point!(8, 2, 12), vector!(0, 0, -1), true),
            (point!(6, 0, -5), vector!(0, 0, 1), true),
            (point!(8, 1, 3.5), vector!(0, 0, 1), true),
            (point!(9, -1, -8), vector!(2, 4, 6), false),
            (point!(8, 3, -4), vector!(6, 2, 4), false),
            (point!(9, -1, -2), vector!(4, 6, 2), false),
            (point!(4, 0, 9), vector!(0, 0, -1), false),
            (point!(8, 6, -1), vector!(0, -1, 0), false),
            (point!(12, 5, 4), vector!(-1, 0, 0), false),
        ] {
            let r = ray!(origin, direction.normalize());
            assert_eq!(expected, bb.intersects(&r));
        }
    }

    #[test]
    fn split_perfect_cube() {
        let bb = BoundingBox::new(point!(-1, -4, -5), point!(9, 6, 5));
        let (left, right) = bb.split();
        assert_eq!(point!(-1, -4, -5), left.min);
        assert_eq!(point!(4, 6, 5), left.max);
        assert_eq!(point!(4, -4, -5), right.min);
        assert_eq!(point!(9, 6, 5), right.max);
    }

    #[test]
    fn split_x_wide_box() {
        let bb = BoundingBox::new(point!(-1, -2, -3), point!(9, 5.5, 3));
        let (left, right) = bb.split();
        assert_eq!(point!(-1, -2, -3), left.min);
        assert_eq!(point!(4, 5.5, 3), left.max);
        assert_eq!(point!(4, -2, -3), right.min);
        assert_eq!(point!(9, 5.5, 3), right.max);
    }

    #[test]
    fn split_y_wide_box() {
        let bb = BoundingBox::new(point!(-1, -2, -3), point!(5, 8, 3));
        let (left, right) = bb.split();
        assert_eq!(point!(-1, -2, -3), left.min);
        assert_eq!(point!(5, 3, 3), left.max);
        assert_eq!(point!(-1, 3, -3), right.min);
        assert_eq!(point!(5, 8, 3), right.max);
    }

    #[test]
    fn split_z_wide_box() {
        let bb = BoundingBox::new(point!(-1, -2, -3), point!(5, 3, 7));
        let (left, right) = bb.split();
        assert_eq!(point!(-1, -2, -3), left.min);
        assert_eq!(point!(5, 3, 2), left.max);
        assert_eq!(point!(-1, -2, 2), right.min);
        assert_eq!(point!(5, 3, 7), right.max);
    }
}
