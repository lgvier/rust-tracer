use crate::{
    intersection,
    intersection::Intersection,
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    point,
    ray::Ray,
    tuple::Tuple,
};

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

impl Shape {
    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<f64> {
        match self {
            Shape::Sphere(s) => s.intersect(r),
        }
    }

    pub fn hit<'a>(&'a self, r: &Ray) -> Option<Intersection> {
        let xs = self
            .intersect(r)
            .iter()
            .map(|e| intersection!(*e, self))
            .collect();
        Intersection::hit(xs)
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        match self {
            Shape::Sphere(s) => s.normal_at(p),
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(s) => s.set_material(material),
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

    pub fn intersect(&self, r: &Ray) -> Vec<f64> {
        let obj_ray = r * self.transform.inverse().unwrap();
        let sphere_to_ray = obj_ray.origin - point!();
        let a = obj_ray.direction.dot(&obj_ray.direction);
        let b = 2. * obj_ray.direction.dot(&sphere_to_ray);
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

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let obj_point = transform_inverse * p;
        let obj_normal = (obj_point - point!()).normalize();
        let world_normal = (transform_inverse.transpose() * obj_normal).to_vector();
        world_normal.normalize()
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray, vector};
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
        let mut s = sphere!();
        let t = Matrix::translation(2., 3., 4.);
        s.set_transform(t);
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
        assert_eq!(Material::default(), s.material);
    }

    #[test]
    fn sphere_set_material() {
        let mut s = sphere!();
        let m = Material::default().with_ambient(1.);
        s.set_material(m);
        assert_eq!(1., s.material.ambient);
    }
}
