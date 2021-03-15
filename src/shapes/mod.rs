pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use crate::{
    arena::Arena,
    intersection::Intersection,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    shapes::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere,
    },
    tuple::Tuple,
};

#[macro_export]
macro_rules! sphere {
    () => {
        $crate::shapes::Shape::Sphere($crate::shapes::sphere::Sphere::new())
    };
}

#[macro_export]
macro_rules! plane {
    () => {
        $crate::shapes::Shape::Plane($crate::shapes::plane::Plane::new())
    };
}

#[macro_export]
macro_rules! cube {
    () => {
        $crate::shapes::Shape::Cube($crate::shapes::cube::Cube::new())
    };
}

#[macro_export]
macro_rules! cylinder {
    () => {
        $crate::shapes::Shape::Cylinder($crate::shapes::cylinder::Cylinder::new())
    };
    ($minimum:expr, $maximum:expr) => {
        $crate::shapes::Shape::Cylinder($crate::shapes::cylinder::Cylinder::new_with_min_max(
            $minimum, $maximum,
        ))
    };
    ($minimum:expr, $maximum:expr, $closed:expr) => {
        $crate::shapes::Shape::Cylinder(
            $crate::shapes::cylinder::Cylinder::new_with_min_max_closed(
                $minimum, $maximum, $closed,
            ),
        )
    };
}

#[macro_export]
macro_rules! cone {
    () => {
        $crate::shapes::Shape::Cone($crate::shapes::cone::Cone::new())
    };
    ($minimum:expr, $maximum:expr) => {
        $crate::shapes::Shape::Cone($crate::shapes::cone::Cone::new_with_min_max(
            $minimum, $maximum,
        ))
    };
    ($minimum:expr, $maximum:expr, $closed:expr) => {
        $crate::shapes::Shape::Cone($crate::shapes::cone::Cone::new_with_min_max_closed(
            $minimum, $maximum, $closed,
        ))
    };
}

// #[macro_export]
// macro_rules! group {
//     // group!()
//     () => (
//         $crate::shapes::Shape::Group($crate::shapes::group::Group::empty())
//     );
//     // group!(shape1, shape2)
//     ($($x:expr),+ $(,)?) => (
//         $crate::shapes::Shape::Group($crate::shapes::group::Group::new(
//             // copied from Rust's vec! macro
//             <[_]>::into_vec(Box::new([$($x),+]))))
//     );
// }

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
    Cone(Cone),
    Group(Group),
}

impl Shape {
    pub fn intersect<'a>(&'a self, arena: &'a Arena, r: &Ray) -> Vec<Intersection> {
        let local_ray = r * self.transform().inverse().unwrap();
        match self {
            Shape::Sphere(s) => self.as_intersections(s.local_intersect(&local_ray)),
            Shape::Plane(p) => self.as_intersections(p.local_intersect(&local_ray)),
            Shape::Cube(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Cylinder(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Cone(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Group(g) => g.local_intersect(arena, &local_ray),
        }
    }

    fn as_intersections(&self, xs: Vec<f64>) -> Vec<Intersection> {
        xs.iter().map(|t| Intersection::new(*t, self)).collect()
    }

    pub fn transform(&self) -> &Matrix {
        match self {
            Shape::Sphere(s) => &s.transform,
            Shape::Plane(p) => &p.transform,
            Shape::Cube(c) => &c.transform,
            Shape::Cylinder(c) => &c.transform,
            Shape::Cone(c) => &c.transform,
            Shape::Group(g) => &g.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(s) => s.transform = transform,
            Shape::Plane(p) => p.transform = transform,
            Shape::Cube(c) => c.transform = transform,
            Shape::Cylinder(c) => c.transform = transform,
            Shape::Cone(c) => c.transform = transform,
            Shape::Group(g) => g.transform = transform,
        }
    }

    pub fn normal_at<'a>(&'a self, arena: &'a Arena, p: Tuple) -> Tuple {
        let local_point = self.world_to_object(arena, p);
        let local_normal = match self {
            Shape::Sphere(s) => s.local_normal_at(local_point),
            Shape::Plane(p) => p.local_normal_at(local_point),
            Shape::Cube(c) => c.local_normal_at(local_point),
            Shape::Cylinder(c) => c.local_normal_at(local_point),
            Shape::Cone(c) => c.local_normal_at(local_point),
            Shape::Group(_) => panic!("Called normal_at on a group"),
        };
        self.normal_to_world(arena, local_normal)
    }

    fn world_to_object<'a>(&'a self, arena: &'a Arena, point: Tuple) -> Tuple {
        let mut point = point;
        if let Some(parent) = self.get_parent(arena) {
            point = parent.world_to_object(arena, point);
        }
        self.transform().inverse().unwrap() * point
    }

    fn normal_to_world<'a>(&'a self, arena: &'a Arena, normal: Tuple) -> Tuple {
        let mut normal = self.transform().inverse().unwrap().transpose() * normal;
        normal.w = 0.;
        normal = normal.normalize();
        if let Some(parent) = self.get_parent(arena) {
            normal = parent.normal_to_world(arena, normal);
        }
        normal
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material,
            Shape::Plane(p) => &p.material,
            Shape::Cube(c) => &c.material,
            Shape::Cylinder(c) => &c.material,
            Shape::Cone(c) => &c.material,
            Shape::Group(_) => panic!("A Group doesnt have a material"),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(s) => s.material = material,
            Shape::Plane(p) => p.material = material,
            Shape::Cube(c) => c.material = material,
            Shape::Cylinder(c) => c.material = material,
            Shape::Cone(c) => c.material = material,
            Shape::Group(_) => panic!("A Group doesnt have a material"),
        }
    }

    pub fn set_parent_id(&mut self, parent_id: Option<usize>) {
        match self {
            Shape::Sphere(s) => s.parent_id = parent_id,
            Shape::Plane(p) => p.parent_id = parent_id,
            Shape::Cube(c) => c.parent_id = parent_id,
            Shape::Cylinder(c) => c.parent_id = parent_id,
            Shape::Cone(c) => c.parent_id = parent_id,
            Shape::Group(g) => g.parent_id = parent_id,
        }
    }

    pub fn get_parent<'a>(&'a self, arena: &'a Arena) -> Option<&Shape> {
        let parent_id = match self {
            Shape::Sphere(s) => s.parent_id,
            Shape::Plane(p) => p.parent_id,
            Shape::Cube(c) => c.parent_id,
            Shape::Cylinder(c) => c.parent_id,
            Shape::Cone(c) => c.parent_id,
            Shape::Group(g) => g.parent_id,
        };
        parent_id.map(|id| arena.get(id))
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::f64::consts::PI;

    use super::*;
    use crate::{material::MaterialBuilder, matrix::IDENTITY_MATRIX, point, sphere, vector};

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
    fn converting_point_from_world_to_object_space() {
        let mut arena = Arena::new();

        let mut s = sphere!();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let s_id = arena.add(s);

        let g2_id = arena.next_id();
        let mut g2_inner = Group::new(g2_id);
        g2_inner.add_child(&mut arena, s_id);
        let mut g2 = Shape::Group(g2_inner);
        g2.set_transform(Matrix::scaling(2., 2., 2.));
        arena.add_with_id(g2_id, g2);

        let g1_id = arena.next_id();
        let mut g1_inner = Group::new(g1_id);
        g1_inner.add_child(&mut arena, g2_id);
        let mut g1 = Shape::Group(g1_inner);
        g1.set_transform(Matrix::rotation_y(PI / 2.));
        arena.add_with_id(g1_id, g1);

        let p = arena
            .get(s_id)
            .world_to_object(&arena, point!(-2., 0., -10.));
        assert_eq!(point!(0., 0., -1.), p);
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let mut arena = Arena::new();

        let mut s = sphere!();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let s_id = arena.add(s);

        let g2_id = arena.next_id();
        let mut g2_inner = Group::new(g2_id);
        g2_inner.add_child(&mut arena, s_id);
        let mut g2 = Shape::Group(g2_inner);
        g2.set_transform(Matrix::scaling(1., 2., 3.));
        arena.add_with_id(g2_id, g2);

        let g1_id = arena.next_id();
        let mut g1_inner = Group::new(g1_id);
        g1_inner.add_child(&mut arena, g2_id);
        let mut g1 = Shape::Group(g1_inner);
        g1.set_transform(Matrix::rotation_y(PI / 2.));
        arena.add_with_id(g1_id, g1);

        let n = arena.get(s_id).normal_to_world(
            &arena,
            point!(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
        );
        assert_eq!(vector!(0.28571, 0.42857, -0.85714), n);
    }

    #[test]
    fn finding_normal_on_child_object() {
        let mut arena = Arena::new();

        let mut s = sphere!();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let s_id = arena.add(s);

        let g2_id = arena.next_id();
        let mut g2_inner = Group::new(g2_id);
        g2_inner.add_child(&mut arena, s_id);
        let mut g2 = Shape::Group(g2_inner);
        g2.set_transform(Matrix::scaling(1., 2., 3.));
        arena.add_with_id(g2_id, g2);

        let g1_id = arena.next_id();
        let mut g1_inner = Group::new(g1_id);
        g1_inner.add_child(&mut arena, g2_id);
        let mut g1 = Shape::Group(g1_inner);
        g1.set_transform(Matrix::rotation_y(PI / 2.));
        arena.add_with_id(g1_id, g1);

        let n = arena
            .get(s_id)
            .normal_at(&arena, point!(1.7321, 1.1547, -5.5774));
        assert_eq!(vector!(0.2857, 0.42854, -0.85716), n);
    }
}
