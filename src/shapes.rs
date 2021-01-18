use crate::matrix::{Matrix, IDENTITY_MATRIX};
use crate::point;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[macro_export]
macro_rules! sphere {
    () => {
        Sphere::new()
    };
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
        }
    }
    pub fn intersect(&self, r: Ray) -> Vec<f64> {
        let local_ray = r * self.transform.inverse().unwrap();
        let sphere_to_ray = local_ray.origin - point!();
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2. * local_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            vec![t1, t2]
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray;

    #[test]
    fn sphere_ray_intersects_at_two_pts() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4., xs[0]);
        assert_eq!(6., xs[1]);
    }

    #[test]
    fn sphere_ray_intersects_tangent() {
        let r = ray!(0., 1., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5., xs[0]);
        assert_eq!(5., xs[1]);
    }

    #[test]
    fn sphere_ray_misses() {
        let r = ray!(0., 2., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_ray_within() {
        let r = ray!(0., 0., 0.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1., xs[0]);
        assert_eq!(1., xs[1]);
    }

    #[test]
    fn sphere_ray_behind() {
        let r = ray!(0., 0., 5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6., xs[0]);
        assert_eq!(-4., xs[1]);
    }

    #[test]
    fn sphere_set_transform() {
        let mut s = sphere!();
        let t = Matrix::translation(2., 3., 4.);
        s.set_transform(t);
        assert_eq!(t, s.transform);
    }

    #[test]
    fn sphere_set_transform_intersect() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::scaling(2., 2., 2.));
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0]);
        assert_eq!(7., xs[1]);
    }
}
