use super::point;
use super::tuple::Tuple;

#[macro_export]
macro_rules! ray {
    // () => {};
    // ray!(2., 3., 4.; 1., 0., 0.);
    ($($origin: expr),+; $($direction: expr),+) => {
        Ray::new(Tuple::point($($origin),*), Tuple::vector($($direction),*))
    };
    // ray!(origin, direction)
    ($origin:expr, $direction:expr) => {
        Ray::new($origin, $direction)
    };
}

#[derive(Clone, Copy)]
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

#[cfg(test)]
mod tests {

    use crate::vector;

    use super::*;

    #[test]
    fn ray_ctor() {
        let origin = point!(1., 2., 3.);
        let direction = vector!(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn ray_compute_point_from_distance() {
        let r = ray!(2., 3., 4.; 1., 0., 0.);
        assert_eq!(point!(2., 3., 4.), r.position(0.));
        assert_eq!(point!(3., 3., 4.), r.position(1.));
        assert_eq!(point!(1., 3., 4.), r.position(-1.));
        assert_eq!(point!(4.5, 3., 4.), r.position(2.5));
    }
}
