use std::mem;

use crate::{
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
    tuple::Tuple,
    vector, EPSILON,
};

#[derive(Debug, PartialEq)]
pub struct Cone {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
    pub transform: Matrix,
    pub material: Material,
}

impl Cone {
    pub fn new() -> Self {
        Cone {
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
            closed: false,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    pub fn new_with_min_max(minimum: f64, maximum: f64) -> Self {
        Cone {
            minimum,
            maximum,
            closed: false,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    pub fn new_with_min_max_closed(minimum: f64, maximum: f64, closed: bool) -> Self {
        Cone {
            minimum,
            maximum,
            closed,
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let mut xs = vec![];

        let a = local_ray.direction.x.powi(2) - local_ray.direction.y.powi(2)
            + local_ray.direction.z.powi(2);
        let b = 2. * local_ray.origin.x * local_ray.direction.x
            - 2. * local_ray.origin.y * local_ray.direction.y
            + 2. * local_ray.origin.z * local_ray.direction.z;
        let c =
            local_ray.origin.x.powi(2) - local_ray.origin.y.powi(2) + local_ray.origin.z.powi(2);

        if a.abs() >= EPSILON {
            // ray is not parallel to the y axis​
            let discriminant = b.powi(2) - 4. * a * c;

            if discriminant >= 0. {
                // ray intersects the cone
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
        } else {
            xs.push(-c / (b * 2.))
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
            if Cone::check_cap(local_ray, t, self.minimum) {
                xs.push(t);
            }
        }
        {
            let t = (self.maximum - local_ray.origin.y) / local_ray.direction.y;
            if Cone::check_cap(local_ray, t, self.maximum) {
                xs.push(t);
            }
        }
    }

    fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x * x + z * z) <= radius * radius
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let y = (local_point.x.powi(2) + local_point.z.powi(2)).sqrt();
        let y = if local_point.y > 0.0 { -y } else { y };
        vector!(local_point.x, y, local_point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, point, ray, vector};

    #[test]
    fn intersecting_cone() {
        let c = Cone::new();
        let t = |origin: Tuple, direction: Tuple, t1: f64, t2: f64| {
            let r = ray!(origin, direction.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(
                2,
                xs.len(),
                "len for origin: {:?}, direction: {:?}",
                origin,
                direction
            );
            assert!(
                approx_eq(t1, dbg!(xs[0])),
                "xs[0] for origin: {:?}, direction: {:?}",
                origin,
                direction
            );
            assert!(
                approx_eq(t2, dbg!(xs[1])),
                "xs[1] for origin: {:?}, direction: {:?}",
                origin,
                direction
            );
        };
        t(point!(0., 0., -5.), vector!(0., 0., 1.), 5., 5.);
        t(point!(0., 0., -5.), vector!(1., 1., 1.), 8.66025, 8.66025);
        t(
            point!(1., 1., -5.),
            vector!(-0.5, -1., 1.),
            4.55006,
            49.44994,
        );
    }

    #[test]
    fn intersecting_ray_with_ray_parallel_to_halves() {
        let c = Cone::new();
        let r = ray!(point!(0., 0., -1.), vector!(0., 1., 1.).normalize());
        let xs = c.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert!(approx_eq(0.35355, dbg!(xs[0])));
    }

    #[test]
    fn intersecting_caps_closed_cone() {
        let c = Cone::new_with_min_max_closed(-0.5, 0.5, true);
        let t = |origin: Tuple, direction: Tuple, count: usize| {
            let r = ray!(origin, direction.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(
                count,
                xs.len(),
                "count for point: {:?}, direction: {:?}",
                origin,
                direction
            );
        };
        t(point!(0., 0., -5.), vector!(0., 1., 0.), 0);
        t(point!(0., 0., -0.25), vector!(0., 1., 1.), 2);
        t(point!(0., 0., -0.25), vector!(0., 1., 0.), 4);
    }

    #[test]
    fn normal() {
        let c = Cone::new();
        let t = |point: Tuple, normal: Tuple| {
            let n = c.local_normal_at(point);
            assert_eq!(normal, n, "normal at {:?}", point);
        };
        t(point!(0., 0., 0.), vector!(0., 0., 0.));
        t(point!(1., 1., 1.), vector!(1., -2f64.sqrt(), 1.));
        t(point!(-1., -1., 0.), vector!(-1., 1., 0.));
    }
}
