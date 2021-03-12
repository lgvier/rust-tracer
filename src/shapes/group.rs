use crate::{
    intersection::Intersection,
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
    tuple::Tuple,
    vector,
};

use super::Shape;

#[derive(Debug, PartialEq)]
pub struct Group {
    pub transform: Matrix,
    pub children: Vec<Shape>,
}

impl<'a> Group {
    pub fn new() -> Self {
        Self {
            transform: IDENTITY_MATRIX,
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, child: Shape) {
        self.children.push(child);
    }

    pub fn set_material(&mut self, material: Material) {
        self.children
            .iter_mut()
            .for_each(|child| child.set_material(material));
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in &self.children {
            result.extend(object.intersect(local_ray));
        }
        Intersection::sort(&mut result);
        result
    }

    pub fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        vector!(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray, ray::Ray, shapes::sphere::Sphere, sphere};

    #[test]
    fn group() {
        let mut group = Group::new();
        let sphere = sphere!();
        group.add(sphere);
    }

    #[test]
    fn intersect_non_empty_group() {
        let mut group = Group::new();
        let s1 = sphere!();
        let mut s2 = sphere!();
        s2.set_transform(Matrix::translation(0., 0., -3.));
        let mut s3 = sphere!();

        s3.set_transform(Matrix::translation(5., 0., 0.));
        group.add(s1);
        group.add(s2);
        group.add(s3);

        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let xs = group.local_intersect(&r);

        assert_eq!(4, xs.len());
    }

    #[test]
    fn intersect_transformed_group() {
        let mut group = Group::new();

        let mut sphere = sphere!();
        sphere.set_transform(Matrix::translation(5., 0., 0.));
        group.add(sphere);

        let r = ray!(10., 0., 10.; 0., 0., 1.);

        let mut group_shape = Shape::Group(group);
        group_shape.set_transform(Matrix::scaling(2., 2., 2.));

        let xs = group_shape.intersect(&r);
        assert_eq!(2, xs.len());
    }
}
