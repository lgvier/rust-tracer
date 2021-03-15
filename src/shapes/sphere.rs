use crate::{
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    point,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
    pub parent_id: Option<usize>,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent_id: None,
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
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

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - point!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::PI;

    use crate::{arena::Arena, material::MaterialBuilder, ray, sphere, vector};

    #[test]
    fn sphere_ray_intersects_at_two_pts() {
        let arena = Arena::new();
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&arena, &r);
        assert_eq!(2, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(6., xs[1].t);
    }

    #[test]
    fn sphere_ray_intersects_tangent() {
        let arena = Arena::new();
        let r = ray!(0., 1., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&arena, &r);
        assert_eq!(2, xs.len());
        assert_eq!(5., xs[0].t);
        assert_eq!(5., xs[1].t);
    }

    #[test]
    fn sphere_ray_misses() {
        let arena = Arena::new();
        let r = ray!(0., 2., -5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&arena, &r);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_ray_within() {
        let arena = Arena::new();
        let r = ray!(0., 0., 0.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&arena, &r);
        assert_eq!(2, xs.len());
        assert_eq!(-1., xs[0].t);
        assert_eq!(1., xs[1].t);
    }

    #[test]
    fn sphere_ray_behind() {
        let arena = Arena::new();
        let r = ray!(0., 0., 5.; 0., 0., 1.);
        let s = sphere!();
        let xs = s.intersect(&arena, &r);
        assert_eq!(2, xs.len());
        assert_eq!(-6., xs[0].t);
        assert_eq!(-4., xs[1].t);
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
        let arena = Arena::new();
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::scaling(2., 2., 2.));
        let xs = s.intersect(&arena, &r);
        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0].t);
        assert_eq!(7., xs[1].t);
    }

    #[test]
    fn sphere_set_transform_translated_intersect() {
        let arena = Arena::new();
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let xs = s.intersect(&arena, &r);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_normal_x_axis() {
        let arena = Arena::new();
        let s = sphere!();
        let n = s.normal_at(&arena, point!(1., 0., 0.));
        assert_eq!(vector!(1., 0., 0.), n);
    }

    #[test]
    fn sphere_normal_y_axis() {
        let arena = Arena::new();
        let s = sphere!();
        let n = s.normal_at(&arena, point!(0., 1., 0.));
        assert_eq!(vector!(0., 1., 0.), n);
    }

    #[test]
    fn sphere_normal_z_axis() {
        let arena = Arena::new();
        let s = sphere!();
        let n = s.normal_at(&arena, point!(0., 0., 1.));
        assert_eq!(vector!(0., 0., 1.), n);
    }

    #[test]
    fn sphere_normal_nonaxial() {
        let arena = Arena::new();
        let s = sphere!();
        let n = s.normal_at(
            &arena,
            point!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
        );
        assert_eq!(
            vector!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
            n
        );
    }

    #[test]
    fn sphere_normal_is_normalized_vector() {
        let arena = Arena::new();
        let s = sphere!();
        let n = s.normal_at(
            &arena,
            point!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
        );
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn sphere_normal_translated() {
        let arena = Arena::new();
        let mut s = sphere!();
        s.set_transform(Matrix::translation(0., 1., 0.));
        let n = s.normal_at(&arena, point!(0., 1.70711, -0.70711));
        assert_eq!(vector!(0., 0.70711, -0.70711), n);
    }

    #[test]
    fn sphere_normal_transformed() {
        let arena = Arena::new();
        let mut s = sphere!();
        s.set_transform(Matrix::scaling(1., 0.5, 1.) * Matrix::rotation_z(PI / 5.));
        let n = s.normal_at(&arena, point!(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.));
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
}
