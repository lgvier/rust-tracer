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

/*
enum vs boxed trait polymorphism:
https://stackoverflow.com/questions/52240099/should-i-use-enums-or-boxed-trait-objects-to-emulate-polymorphism
*/

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<f64> {
        let local_ray = r * self.transform().inverse().unwrap();
        match self {
            Shape::Sphere(s) => s.local_intersect(&local_ray),
            Shape::Plane(p) => p.local_intersect(&local_ray),
        }
    }

    pub fn transform(&self) -> &Matrix {
        match self {
            Shape::Sphere(s) => &s.transform,
            Shape::Plane(p) => &p.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(s) => s.transform = transform,
            Shape::Plane(p) => p.transform = transform,
        }
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform().inverse().unwrap();
        let local_point = transform_inverse * p;
        let local_normal = match self {
            Shape::Sphere(s) => s.local_normal_at(local_point),
            Shape::Plane(p) => p.local_normal_at(local_point),
        };
        let world_normal = (transform_inverse.transpose() * local_normal).to_vector();
        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material,
            Shape::Plane(p) => &p.material,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(s) => s.material = material,
            Shape::Plane(p) => p.material = material,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::MaterialBuilder, ray, vector};
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
}
