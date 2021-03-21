use crate::{
    arena::Arena,
    bounds::BoundingBox,
    intersection::Intersection,
    matrix::{Matrix, IDENTITY_MATRIX},
    ray::Ray,
};

use super::Shape;

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

    pub fn add_child(&mut self, child_id: usize, arena: &mut Arena) {
        assert!(!self.children_ids.contains(&child_id));
        arena.apply_changes(child_id, |c| c.set_parent_id(Some(self.id)));
        self.children_ids.push(child_id);
    }

    pub fn add_children(&mut self, children_ids: &[usize], arena: &mut Arena) {
        for child_id in children_ids {
            self.add_child(*child_id, arena);
        }
    }

    fn remove_child(&mut self, child_id: usize) -> bool {
        let index = self.children_ids.iter().position(|id| *id == child_id);
        match index {
            Some(i) => {
                self.children_ids.remove(i);
                return true;
            }
            None => false,
        }
    }

    fn remove_children(&mut self, children_ids: &[usize]) {
        for child_id in children_ids {
            self.remove_child(*child_id);
        }
    }

    pub fn local_intersect<'a>(&self, arena: &'a Arena, local_ray: &Ray) -> Vec<Intersection<'a>> {
        if self.bounds(arena).intersects(&local_ray) {
            let mut result = self
                .children_ids
                .iter()
                .map(|child_id| arena.get(*child_id))
                .flat_map(|c| c.intersect(arena, local_ray))
                .collect::<Vec<_>>();
            Intersection::sort(&mut result);
            result
        } else {
            vec![]
        }
    }

    pub fn bounds<'a>(&self, arena: &'a Arena) -> BoundingBox {
        self.children_ids
            .iter()
            .map(|child_id| arena.get(*child_id))
            .fold(BoundingBox::empty(), |bb, c| {
                bb + c.parent_space_bounds(arena)
            })
    }

    fn partition_children(&mut self, arena: &Arena) -> (Vec<usize>, Vec<usize>) {
        let (left, right) = self.bounds(arena).split();

        let mut left_ids = vec![];
        let mut right_ids = vec![];

        for child_id in &self.children_ids {
            let child = arena.get(*child_id);
            let child_bounds = child.parent_space_bounds(arena);
            if left.contains_box(child_bounds) {
                left_ids.push(*child_id);
            } else if right.contains_box(child_bounds) {
                right_ids.push(*child_id);
            } else {
                continue;
            }
        }

        self.remove_children(&left_ids);
        self.remove_children(&right_ids);

        (left_ids, right_ids)
    }

    fn make_subgroup(&mut self, children_ids: &[usize], arena: &mut Arena) {
        let subgroup_id = arena.next_id();
        let mut subgroup = Group::new(subgroup_id);
        subgroup.add_children(children_ids, arena);
        arena.add_with_id(subgroup_id, Shape::Group(subgroup));
        self.add_child(subgroup_id, arena);
    }

    pub fn divide(&mut self, threshold: usize, arena: &mut Arena) {
        if threshold <= self.children_ids.len() {
            let (left, right) = self.partition_children(arena);
            if !left.is_empty() {
                self.make_subgroup(&left, arena);
            }
            if !right.is_empty() {
                self.make_subgroup(&right, arena);
            }
        }
        // for child_id in &self.children_ids {
        //     let child = arena.get(*child_id);
        //     child.divide(threshold, arena);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray, shapes::Shape, sphere};

    #[test]
    fn group() {
        let mut arena = Arena::new();
        let group_id = arena.next_id();
        let mut group = Group::new(group_id);

        let sphere_id = arena.add(sphere!());
        group.add_child(sphere_id, &mut arena);

        arena.add_with_id(group_id, Shape::Group(group));

        assert_eq!(
            Some(arena.get(group_id)),
            arena.get(sphere_id).get_parent(&arena)
        );
    }

    #[test]
    fn intersect_non_empty_group() {
        let mut arena = Arena::new();

        let mut s1 = sphere!();
        let s1_transform = IDENTITY_MATRIX;
        s1.set_transform(s1_transform);

        let mut s2 = sphere!();
        let s2_transform = Matrix::translation(0, 0, -3);
        s2.set_transform(s2_transform);

        let mut s3 = sphere!();
        s3.set_transform(Matrix::translation(5, 0, 0));

        let group_id = arena.next_id();
        let mut group = Group::new(group_id);
        group.add_children(&[arena.add(s1), arena.add(s2), arena.add(s3)], &mut arena);

        let r = ray!(0, 0, -5; 0, 0, 1);
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
        sphere.set_transform(Matrix::translation(5, 0, 0));

        let group_id = arena.next_id();
        let mut group_inner = Group::new(group_id);
        group_inner.add_child(arena.add(sphere), &mut arena);

        let mut group = Shape::Group(group_inner);
        group.set_transform(Matrix::scaling(2, 2, 2));

        arena.add_with_id(group_id, group);

        let r = ray!(10, 0, 10; 0, 0, 1);
        let xs = arena.get(group_id).intersect(&arena, &r);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn partitioning_groups_children() {
        let mut arena = Arena::new();

        let mut s1 = sphere!();
        s1.set_transform(Matrix::translation(-2, 0, 0));

        let mut s2 = sphere!();
        s2.set_transform(Matrix::translation(2, 0, 0));

        let s3 = sphere!();

        let group_id = arena.next_id();
        let mut group_inner = Group::new(group_id);
        let s1_id = arena.add(s1);
        let s2_id = arena.add(s2);
        let s3_id = arena.add(s3);
        group_inner.add_children(&[s1_id, s2_id, s3_id], &mut arena);

        let (left, right) = group_inner.partition_children(&mut arena);

        assert_eq!(vec![s3_id], group_inner.children_ids);
        assert_eq!(vec![s1_id], left);
        assert_eq!(vec![s2_id], right);
    }

    #[test]
    fn create_subgroup_from_children() {
        let mut arena = Arena::new();

        let s1 = sphere!();
        let s2 = sphere!();

        let group_id = arena.next_id();
        let mut g = Group::new(group_id);

        let s1_id = arena.add(s1);
        let s2_id = arena.add(s2);
        g.make_subgroup(&[s1_id, s2_id], &mut arena);

        assert_eq!(1, g.children_ids.len());
        let subgroup = match arena.get(g.children_ids[0]) {
            Shape::Group(g) => g,
            _ => panic!("not a group"),
        };
        assert_eq!(vec![s1_id, s2_id], subgroup.children_ids);
    }

    #[test]
    fn divide() {
        let mut arena = Arena::new();

        let mut s1 = sphere!();
        s1.set_transform(Matrix::translation(-2, -2, 0));

        let mut s2 = sphere!();
        s2.set_transform(Matrix::translation(-2, 2, 0));

        let mut s3 = sphere!();
        s3.set_transform(Matrix::scaling(4, 4, 4));

        let group_id = arena.next_id();
        let mut group_inner = Group::new(group_id);
        let s1_id = arena.add(s1);
        let s2_id = arena.add(s2);
        let s3_id = arena.add(s3);
        group_inner.add_children(&[s1_id, s2_id, s3_id], &mut arena);

        group_inner.divide(1, &mut arena);
        assert_eq!(2, group_inner.children_ids.len());

        println!("{:?}", arena.get(group_inner.children_ids[0]));
        println!("{:?}", arena.get(group_inner.children_ids[1]));
        assert_eq!(s3_id, group_inner.children_ids[0]);

        let subgroup = match arena.get(group_inner.children_ids[1]) {
            Shape::Group(g) => g,
            _ => panic!("not a group"),
        };
        assert_eq!(2, subgroup.children_ids.len());
        assert_eq!(s1_id, subgroup.children_ids[0]);
        assert_eq!(s2_id, subgroup.children_ids[1]);
    }
}
