use crate::{
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    point,
    ray::Ray,
    tuple::Tuple,
    vector, EPSILON,
};

#[macro_export]
macro_rules! sphere {
    () => {
        Shape::Sphere(Sphere::new())
    };
}

#[macro_export]
macro_rules! plane {
    () => {
        Shape::Plane(Plane::new())
    };
}

#[macro_export]
macro_rules! cube {
    () => {
        Shape::Cube(Cube::new())
    };
}

#[macro_export]
macro_rules! cylinder {
    () => {
        Shape::Cylinder(Cylinder::new())
    };
}

/*
enum vs boxed trait polymorphism:
https://stackoverflow.com/questions/52240099/should-i-use-enums-or-boxed-trait-objects-to-emulate-polymorphism
*/

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

impl Shape {
    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<f64> {
        let local_ray = r * self.transform().inverse().unwrap();
        match self {
            Shape::Sphere(s) => s.local_intersect(&local_ray),
            Shape::Plane(p) => p.local_intersect(&local_ray),
            Shape::Cube(c) => c.local_intersect(&local_ray),
            Shape::Cylinder(c) => c.local_intersect(&local_ray),
        }
    }

    pub fn transform(&self) -> &Matrix {
        match self {
            Shape::Sphere(s) => &s.transform,
            Shape::Plane(p) => &p.transform,
            Shape::Cube(c) => &c.transform,
            Shape::Cylinder(c) => &c.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(s) => s.transform = transform,
            Shape::Plane(p) => p.transform = transform,
            Shape::Cube(c) => c.transform = transform,
            Shape::Cylinder(c) => c.transform = transform,
        }
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform().inverse().unwrap();
        let local_point = transform_inverse * p;
        let local_normal = match self {
            Shape::Sphere(s) => s.local_normal_at(local_point),
            Shape::Plane(p) => p.local_normal_at(local_point),
            Shape::Cube(c) => c.local_normal_at(local_point),
            Shape::Cylinder(c) => c.local_normal_at(local_point),
        };
        let world_normal = (transform_inverse.transpose() * local_normal).to_vector();
        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material,
            Shape::Plane(p) => &p.material,
            Shape::Cube(c) => &c.material,
            Shape::Cylinder(c) => &c.material,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(s) => s.material = material,
            Shape::Plane(p) => p.material = material,
            Shape::Cube(c) => c.material = material,
            Shape::Cylinder(c) => c.material = material,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
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

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - point!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Plane {
    transform: Matrix,
    material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            // ray is parallel to the plane
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;
            vec![t]
        }
    }

    fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        vector!(0., 1., 0.)
    }
}

#[derive(Debug, PartialEq)]
pub struct Cube {
    transform: Matrix,
    material: Material,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
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

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
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

#[derive(Debug, PartialEq)]
pub struct Cylinder {
    transform: Matrix,
    material: Material,
}

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);
        if a.abs() < EPSILON {
            // ray is parallel to the y axis​
            return vec![];
        }

        let b = 2. * local_ray.origin.x * local_ray.direction.x
            + 2. * local_ray.origin.z * local_ray.direction.z;
        let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            // ray does not intersect the cylinder​
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            vec![t1, t2]
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        vector!(local_point.x, 0., local_point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, material::MaterialBuilder, ray, vector};
    use rand::Rng;
    use std::f64::consts::PI;

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

    #[test]
    fn sphere_set_transform() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2., 3., 4.);
        s.transform = t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn sphere_set_transform_scaled_intersect() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::scaling(2., 2., 2.));
        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0]);
        assert_eq!(7., xs[1]);
    }

    #[test]
    fn sphere_set_transform_translated_intersect() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let xs = s.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_normal_x_axis() {
        let s = sphere!();
        let n = s.normal_at(point!(1., 0., 0.));
        assert_eq!(vector!(1., 0., 0.), n);
    }

    #[test]
    fn sphere_normal_y_axis() {
        let s = sphere!();
        let n = s.normal_at(point!(0., 1., 0.));
        assert_eq!(vector!(0., 1., 0.), n);
    }

    #[test]
    fn sphere_normal_z_axis() {
        let s = sphere!();
        let n = s.normal_at(point!(0., 0., 1.));
        assert_eq!(vector!(0., 0., 1.), n);
    }

    #[test]
    fn sphere_normal_nonaxial() {
        let s = sphere!();
        let n = s.normal_at(point!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.));
        assert_eq!(
            vector!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
            n
        );
    }

    #[test]
    fn sphere_normal_is_normalized_vector() {
        let s = sphere!();
        let n = s.normal_at(point!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn sphere_normal_translated() {
        let mut s = sphere!();
        s.set_transform(Matrix::translation(0., 1., 0.));
        let n = s.normal_at(point!(0., 1.70711, -0.70711));
        assert_eq!(vector!(0., 0.70711, -0.70711), n);
    }

    #[test]
    fn sphere_normal_transformed() {
        let mut s = sphere!();
        s.set_transform(Matrix::scaling(1., 0.5, 1.) * Matrix::rotation_z(PI / 5.));
        let n = s.normal_at(point!(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.));
        assert_eq!(vector!(0., 0.97014, -0.24254), n);
    }

    #[test]
    fn sphere_has_a_default_material() {
        let s = sphere!();
        assert_eq!(Material::default(), *s.material());
    }

    #[test]
    fn sphere_set_material() {
        let mut s = sphere!();
        s.set_material(MaterialBuilder::default().ambient(1.).build().unwrap());
        assert_eq!(1., s.material().ambient);
    }

    fn test_shape() -> Shape {
        let mut rng = rand::thread_rng();
        match rng.gen::<u8>() % 2 {
            0 => sphere!(),
            _ => plane!(),
        }
    }

    #[test]
    fn shape_default_transformation() {
        let s = test_shape();
        assert_eq!(&IDENTITY_MATRIX, s.transform());
    }

    #[test]
    fn shape_assign_transformation() {
        let mut s = test_shape();
        s.set_transform(Matrix::translation(2., 3., 4.));
        assert_eq!(&Matrix::translation(2., 3., 4.), s.transform());
    }

    #[test]
    fn shape_default_material() {
        let s = test_shape();
        assert_eq!(&Material::default(), s.material());
    }

    #[test]
    fn shape_assign_material() {
        let mut s = test_shape();
        s.set_material(MaterialBuilder::default().ambient(1.).build().unwrap());
        assert_eq!(1., s.material().ambient);
    }

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(point!(0., 0., 0.));
        let n2 = p.local_normal_at(point!(10., 0., -10.));
        let n3 = p.local_normal_at(point!(-5., 0., 150.));
        assert_eq!(vector!(0., 1., 0.), n1);
        assert_eq!(vector!(0., 1., 0.), n2);
        assert_eq!(vector!(0., 1., 0.), n3);
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = ray!(0., 10., 0.; 0., 0., 1.);
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::new();
        let r = ray!(0., 0., 0.; 0., 0., 1.);
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = ray!(0., 1., 0.; 0., -1., 0.);
        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(1., xs[0]);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::new();
        let r = ray!(0., -1., 0.; 0., 1., 0.);
        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(1., xs[0]);
    }

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
}
