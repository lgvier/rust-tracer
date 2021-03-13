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
pub struct Plane {
    pub transform: Matrix,
    pub material: Material,
    pub parent: Option<Arc<RwLock<Group>>>,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            parent: None,
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            // ray is parallel to the plane
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;
            vec![t]
        }
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        vector!(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray, vector};

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
