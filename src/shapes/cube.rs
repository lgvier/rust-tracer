use crate::{
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
    shapes::group::Group,
    tuple::Tuple,
    vector, EPSILON,
};
use std::{
    ptr,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Cube {
    pub transform: Matrix,
    pub material: Material,
    pub parent: Option<Arc<RwLock<Group>>>,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent: None,
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let (xtmin, xtmax) = Cube::check_axis(local_ray.origin.x, local_ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(local_ray.origin.y, local_ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(local_ray.origin.z, local_ray.direction.z);
        let tmin = xtmin.max(ytmin.max(ztmin));
        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmin > tmax {
            // miss
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (
                tmin_numerator * f64::INFINITY,
                tmax_numerator * f64::INFINITY,
            )
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let x_abs = local_point.x.abs();
        let y_abs = local_point.y.abs();
        let z_abs = local_point.z.abs();
        let maxc = x_abs.max(y_abs.max(z_abs));
        if maxc == x_abs {
            vector!(local_point.x, 0., 0.)
        } else if maxc == y_abs {
            vector!(0., local_point.y, 0.)
        } else {
            vector!(0., 0., local_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray, vector};

    #[test]
    fn ray_intersects_cube() {
        let c = Cube::new();
        let t = |desc, origin, direction, t1, t2| {
            let r = ray!(origin, direction);
            let xs = c.local_intersect(&r);
            assert_eq!(2, xs.len(), "len {}", desc);
            assert_eq!(xs[0], t1, "xs[0] {}", desc);
            assert_eq!(xs[1], t2, "xs[1] {}", desc);
        };
        t("+x", point!(5., 0.5, 0.), vector!(-1., 0., 0.), 4., 6.);
        t("-x", point!(-5., 0.5, 0.), vector!(1., 0., 0.), 4., 6.);
        t("+y", point!(0.5, 5., 0.), vector!(0., -1., 0.), 4., 6.);
        t("-y", point!(0.5, -5., 0.), vector!(0., 1., 0.), 4., 6.);
        t("+z", point!(0.5, 0., 5.), vector!(0., 0., -1.), 4., 6.);
        t("-z", point!(0.5, 0., -5.), vector!(0., 0., 1.), 4., 6.);
        t("inside", point!(0., 0.5, 0.), vector!(0., 0., 1.), -1., 1.);
    }

    #[test]
    fn ray_misses_cube() {
        let c = Cube::new();
        let t = |origin, direction| {
            let r = ray!(origin, direction);
            let xs = c.local_intersect(&r);
            assert_eq!(0, xs.len(), "len {:?}, {:?}", origin, direction);
        };
        t(point!(-2., 0., 0.), vector!(0.2673, 0.5345, 0.8018));
        t(point!(0., -2., 0.), vector!(0.8018, 0.2673, 0.5345));
        t(point!(0., 0., -2.), vector!(0.5345, 0.8018, 0.2673));
        t(point!(2., 0., 2.), vector!(0., 0., -1.));
        t(point!(0., 2., 2.), vector!(0., -1., 0.));
        t(point!(2., 2., 0.), vector!(-1., 0., 0.));
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let c = Cube::new();
        let t = |point, normal| {
            let n = c.local_normal_at(point);
            assert_eq!(normal, n, "normal for point: {:?}", point);
        };
        t(point!(1., 0.5, -0.8), vector!(1., 0., 0.));
        t(point!(-1., -0.2, 0.9), vector!(-1., 0., 0.));
        t(point!(-0.4, 1., -0.1), vector!(0., 1., 0.));
        t(point!(0.3, -1., -0.7), vector!(0., -1., 0.));
        t(point!(-0.6, 0.3, 1.), vector!(0., 0., 1.));
        t(point!(0.4, 0.4, -1.), vector!(0., 0., -1.));
        t(point!(1., 1., 1.), vector!(1., 0., 0.));
        t(point!(-1., -1., -1.), vector!(-1., 0., 0.));
    }
}
