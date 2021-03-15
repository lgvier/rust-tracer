use crate::{
    arena::Arena,
    intersection::Intersection,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
};

#[derive(Debug, PartialEq)]
pub struct Group {
    id: usize,
    pub transform: Matrix,
    pub parent_id: Option<usize>,
    pub children_ids: Vec<usize>,
}

impl Group {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            transform: IDENTITY_MATRIX,
            parent_id: None,
            children_ids: Vec::new(),
        }
    }

    pub fn add_child(&mut self, arena: &mut Arena, child_id: usize) {
        arena.apply_changes(child_id, |c| c.set_parent_id(Some(self.id)));
        self.children_ids.push(child_id);
    }

    pub fn add_children(&mut self, arena: &mut Arena, children_ids: &[usize]) {
        for child_id in children_ids {
            self.add_child(arena, *child_id);
        }
    }

    pub fn local_intersect<'a>(&self, arena: &'a Arena, local_ray: &Ray) -> Vec<Intersection<'a>> {
        let mut result = vec![];
        for child_id in &self.children_ids {
            result.extend(arena.get(*child_id).intersect(arena, local_ray));
        }
        Intersection::sort(&mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray, shapes::Shape, sphere};

    #[test]
    fn group() {
        let mut arena = Arena::new();
        let mut group = Group::new(arena.next_id());

        let sphere = sphere!();
        let sphere_id = arena.add(sphere);
        group.add_child(&mut arena, sphere_id);
        arena.add_with_id(group.id, Shape::Group(group));
    }

    #[test]
    fn intersect_non_empty_group() {
        let mut arena = Arena::new();

        let mut s1 = sphere!();
        let s1_transform = IDENTITY_MATRIX;
        s1.set_transform(s1_transform);

        let mut s2 = sphere!();
        let s2_transform = Matrix::translation(0., 0., -3.);
        s2.set_transform(s2_transform);

        let mut s3 = sphere!();
        s3.set_transform(Matrix::translation(5., 0., 0.));

        let group_id = arena.next_id();
        let mut group = Group::new(group_id);
        let children_ids = &[arena.add(s1), arena.add(s2), arena.add(s3)];
        group.add_children(&mut arena, children_ids);

        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let xs = group.local_intersect(&arena, &r);

        assert_eq!(4, xs.len());
        assert_eq!(&s2_transform, xs[0].object.transform());
        assert_eq!(&s2_transform, xs[1].object.transform());
        assert_eq!(&s1_transform, xs[2].object.transform());
        assert_eq!(&s1_transform, xs[3].object.transform());
    }

    #[test]
    fn intersect_transformed_group() {
        let mut arena = Arena::new();

        let mut sphere = sphere!();
        sphere.set_transform(Matrix::translation(5., 0., 0.));
        let sphere_id = arena.add(sphere);

        let group_id = arena.next_id();
        let mut group_inner = Group::new(group_id);
        group_inner.add_child(&mut arena, sphere_id);

        let mut group = Shape::Group(group_inner);
        group.set_transform(Matrix::scaling(2., 2., 2.));

        arena.add_with_id(group_id, group);

        let r = ray!(10., 0., 10.; 0., 0., 1.);
        let xs = arena.get(group_id).intersect(&arena, &r);
        assert_eq!(2, xs.len());
    }
}
