use crate::{
    intersection::Intersection,
    material::Material,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
    shapes::Shape,
    tuple::Tuple,
    vector,
};
use std::{
    ptr,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Group {
    pub transform: Matrix,
    pub children: Vec<Shape>,
    pub parent: Option<Arc<RwLock<Group>>>,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl<'a> Group {
    pub fn new(children: Vec<Shape>) -> Self {
        let mut group = Self {
            transform: IDENTITY_MATRIX,
            children,
            parent: None,
        };
        // group
        //     .children
        //     .iter_mut()
        //     .for_each(|c| c.set_parent(Some(Arc::clone(&inner))));
        group
    }

    pub fn empty() -> Self {
        Group::new(Vec::new())
    }

    pub fn add(&mut self, mut child: Shape) {
        // child.set_parent(Some(Arc::clone(&self.inner)));
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
        // TODO
        vector!(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray, sphere};

    #[test]
    fn group() {
        let mut group = Group::empty();
        let sphere = sphere!();
        group.add(sphere);
    }

    #[test]
    fn intersect_non_empty_group() {
        let mut s1 = sphere!();
        let s1_transform = IDENTITY_MATRIX;
        s1.set_transform(s1_transform);

        let mut s2 = sphere!();
        let s2_transform = Matrix::translation(0., 0., -3.);
        s2.set_transform(s2_transform);

        let mut s3 = sphere!();
        s3.set_transform(Matrix::translation(5., 0., 0.));

        let group = Group::new(vec![s1, s2, s3]);

        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let xs = group.local_intersect(&r);

        assert_eq!(4, xs.len());
        assert_eq!(&s2_transform, xs[0].object.transform());
        assert_eq!(&s2_transform, xs[1].object.transform());
        assert_eq!(&s1_transform, xs[2].object.transform());
        assert_eq!(&s1_transform, xs[3].object.transform());
    }

    #[test]
    fn intersect_transformed_group() {
        // let mut sphere = sphere!();
        // sphere.set_transform(Matrix::translation(5., 0., 0.));

        // let mut group = group!(sphere);
        // group.set_transform(Matrix::scaling(2., 2., 2.));

        // let r = ray!(10., 0., 10.; 0., 0., 1.);
        // let xs = group.intersect(&r);
        // assert_eq!(2, xs.len());
    }
}
