use super::point;
use super::ray::Ray;
use super::tuple::Tuple;
// use crate::tuple;

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
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }
    pub fn intersect(&self, r: &Ray) -> Vec<f64> {
        let sphere_to_ray = r.origin - point!();
        let a = r.direction.dot(&r.direction);
        let b = 2. * r.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / 2. * a;
            let t2 = (-b + discriminant.sqrt()) / 2. * a;
            vec![t1, t2]
        }
    }
}

#[cfg(Test)]
mod tests {

    #[test]
    fn sphere_ray_intersects_at_two_pts() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_eq!(4., xs[0]);
        assert_eq!(6., xs[1]);
    }

    #[test]
    fn sphere_ray_intersects_tangent() {
        let r = ray!(0., 1., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_eq!(5., xs[0]);
        assert_eq!(5., xs[1]);
    }

    #[test]
    fn sphere_ray_misses() {
        let r = ray!(0., 2., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_ray_within() {
        let r = ray!(0., 0., 0.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_eq!(-1., xs[0]);
        assert_eq!(1., xs[1]);
    }

    #[test]
    fn sphere_ray_behind() {
        let r = ray!(0., 0., 5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_eq!(-6., xs[0]);
        assert_eq!(-4., xs[1]);
    }
}
