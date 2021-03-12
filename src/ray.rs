use crate::{matrix::Matrix, tuple::Tuple};
use std::ops::Mul;

#[macro_export]
macro_rules! ray {
    // ray!(2., 3., 4.; 1., 0., 0.);
    ($($origin: expr),+; $($direction: expr),+) => {
        $crate::ray::Ray::new(
            $crate::tuple::Tuple::point($($origin),*),
            $crate::tuple::Tuple::vector($($direction),*))
    };
    // ray!(origin, direction)
    ($origin:expr, $direction:expr) => {
        $crate::ray::Ray::new($origin, $direction)
    };
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }
    pub fn position(&self, t: f64) -> Tuple {
        return self.origin + self.direction * t;
    }
}

impl Mul<Matrix> for Ray {
    type Output = Ray;

    fn mul(self, other: Matrix) -> Ray {
        Ray {
            origin: self.origin * other,
            direction: self.direction * other,
        }
    }
}

impl Mul<Matrix> for &Ray {
    type Output = Ray;

    fn mul(self, other: Matrix) -> Ray {
        Ray {
            origin: self.origin * other,
            direction: self.direction * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, vector};

    #[test]
    fn ctor() {
        let origin = point!(1., 2., 3.);
        let direction = vector!(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = ray!(2., 3., 4.; 1., 0., 0.);
        assert_eq!(point!(2., 3., 4.), r.position(0.));
        assert_eq!(point!(3., 3., 4.), r.position(1.));
        assert_eq!(point!(1., 3., 4.), r.position(-1.));
        assert_eq!(point!(4.5, 3., 4.), r.position(2.5));
    }
}
