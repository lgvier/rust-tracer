use crate::{
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
    shapes::group::Group,
    tuple::Tuple,
    vector, EPSILON,
};
use std::{
    mem, ptr,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
    pub transform: Matrix,
    pub material: Material,
    pub parent: Option<Arc<RwLock<Group>>>,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
            closed: false,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent: None,
        }
    }

    pub fn new_with_min_max(minimum: f64, maximum: f64) -> Self {
        Cylinder {
            minimum,
            maximum,
            closed: false,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent: None,
        }
    }

    pub fn new_with_min_max_closed(minimum: f64, maximum: f64, closed: bool) -> Self {
        Cylinder {
            minimum,
            maximum,
            closed,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent: None,
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let mut xs = vec![];

        let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);
        if a.abs() >= EPSILON {
            // ray is not parallel to the y axis​

            let b = 2. * local_ray.origin.x * local_ray.direction.x
                + 2. * local_ray.origin.z * local_ray.direction.z;
            let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.;

            let discriminant = b.powi(2) - 4. * a * c;

            if discriminant >= 0. {
                // ray intersects the cylinder​
                let mut t0 = (-b - discriminant.sqrt()) / (2. * a);
                let mut t1 = (-b + discriminant.sqrt()) / (2. * a);
                if t0 > t1 {
                    mem::swap(&mut t0, &mut t1);
                }

                let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
                if self.minimum < y0 && y0 < self.maximum {
                    xs.push(t0);
                }
                let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
                if self.minimum < y1 && y1 < self.maximum {
                    xs.push(t1);
                }
            }
        }

        self.intersect_caps(local_ray, &mut xs);
        xs
    }

    fn intersect_caps(&self, local_ray: &Ray, xs: &mut Vec<f64>) {
        if !self.closed || local_ray.direction.y.abs() < EPSILON {
            return;
        }
        {
            let t = (self.minimum - local_ray.origin.y) / local_ray.direction.y;
            if Cylinder::check_cap(local_ray, t) {
                xs.push(t);
            }
        }
        {
            let t = (self.maximum - local_ray.origin.y) / local_ray.direction.y;
            if Cylinder::check_cap(local_ray, t) {
                xs.push(t);
            }
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x * x + z * z) <= 1.
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        // compute the square of the distance from the y axis
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1. && local_point.y >= (self.maximum - EPSILON) {
            vector!(0., 1., 0.)
        } else if dist < 1. && local_point.y <= (self.minimum + EPSILON) {
            vector!(0., -1., 0.)
        } else {
            vector!(local_point.x, 0., local_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, point, ray, vector};

    #[test]
    fn ray_misses_cylinder() {
        let c = Cylinder::new();
        let t = |origin: Tuple, direction: Tuple| {
            let r = ray!(origin, direction.normalize());
            let xs = c.local_intersect(&r);
            assert!(
                xs.is_empty(),
                "origin: {:?}, direction: {:?}",
                origin,
                direction
            );
        };
        t(point!(1., 0., 0.), vector!(0., 1., 0.));
        t(point!(0., 0., 0.), vector!(0., 1., 0.));
        t(point!(0., 0., -5.), vector!(1., 1., 1.));
    }

    #[test]
    fn ray_strikes_cylinder() {
        let c = Cylinder::new();
        let t = |origin: Tuple, direction: Tuple, t1: f64, t2: f64| {
            let r = ray!(origin, direction.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(
                2,
                xs.len(),
                "origin: {:?}, direction: {:?}",
                origin,
                direction
            );
            assert!(
                approx_eq(t1, dbg!(xs[0])),
                "t1 for origin: {:?}, direction: {:?}",
                origin,
                direction
            );
            assert!(
                approx_eq(t2, dbg!(xs[1])),
                "t2 for origin: {:?}, direction: {:?}",
                origin,
                direction
            );
        };
        t(point!(1., 0., -5.), vector!(0., 0., 1.), 5., 5.);
        t(point!(0., 0., -5.), vector!(0., 0., 1.), 4., 6.);
        t(point!(0.5, 0., -5.), vector!(0.1, 1., 1.), 6.80798, 7.08872);
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let c = Cylinder::new();
        let t = |point: Tuple, normal: Tuple| {
            let n = c.local_normal_at(point);
            assert_eq!(normal, n, "normal at {:?}", point);
        };
        t(point!(1., 0., 0.), vector!(1., 0., 0.));
        t(point!(0., 5., -1.), vector!(0., 0., -1.));
        t(point!(0., -2., 1.), vector!(0., 0., 1.));
        t(point!(-1., 1., 0.), vector!(-1., 0., 0.));
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let c = Cylinder::new_with_min_max(1., 2.);
        let t = |point: Tuple, direction: Tuple, count: usize| {
            let r = ray!(point, direction.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(
                count,
                xs.len(),
                "count for point: {:?}, direction: {:?}",
                point,
                direction
            );
        };
        t(point!(0., 1.5, 0.), vector!(0.1, 1., 0.), 0);
        t(point!(0., 3., -5.), vector!(0., 0., 1.), 0);
        t(point!(0., 0., -5.), vector!(0., 0., 1.), 0);
        t(point!(0., 2., -5.), vector!(0., 0., 1.), 0);
        t(point!(0., 1., -5.), vector!(0., 0., 1.), 0);
        t(point!(0., 1.5, -2.), vector!(0., 0., 1.), 2);
    }

    #[test]
    fn intersecting_caps_closed_cylinder() {
        let c = Cylinder::new_with_min_max_closed(1., 2., true);
        let t = |point: Tuple, direction: Tuple, count: usize| {
            let r = ray!(point, direction.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(
                count,
                xs.len(),
                "count for point: {:?}, direction: {:?}",
                point,
                direction
            );
        };
        t(point!(0., 3., 0.), vector!(0., -1., 0.), 2);
        t(point!(0., 3., -2.), vector!(0., -1., 2.), 2);
        t(point!(0., 4., -2.), vector!(0., -1., 1.), 2);
        t(point!(0., 0., -2.), vector!(0., 1., 2.), 2);
        t(point!(0., -1., -2.), vector!(0., 1., 1.), 2);
    }

    #[test]
    fn normal_vector_on_cylinders_end_caps() {
        let c = Cylinder::new_with_min_max_closed(1., 2., true);
        let t = |point: Tuple, normal: Tuple| {
            let n = c.local_normal_at(point);
            assert_eq!(normal, n, "normal at {:?}", point);
        };
        t(point!(0., 1., 0.), vector!(0., -1., 0.));
        t(point!(0.5, 1., 0.), vector!(0., -1., 0.));
        t(point!(0., 1., 0.5), vector!(0., -1., 0.));
        t(point!(0., 2., 0.), vector!(0., 1., 0.));
        t(point!(0.5, 2., 0.), vector!(0., 1., 0.));
        t(point!(0., 2., 0.5), vector!(0., 1., 0.));
    }
}
