pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use crate::{
    intersection::Intersection,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    shapes::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere,
    },
    tuple::Tuple,
};
use std::{
    ptr,
    sync::{Arc, RwLock},
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

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Arc<RwLock<Group>>),
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Shape {
    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection> {
        let local_ray = r * self.transform().inverse().unwrap();
        match self {
            Shape::Sphere(s) => self.as_intersections(s.local_intersect(&local_ray)),
            Shape::Plane(p) => self.as_intersections(p.local_intersect(&local_ray)),
            Shape::Cube(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Cylinder(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Cone(c) => self.as_intersections(c.local_intersect(&local_ray)),
            Shape::Group(g) => g.read().unwrap().local_intersect(&local_ray),
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
            Shape::Group(g) => &g.read().unwrap().transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(s) => s.transform = transform,
            Shape::Plane(p) => p.transform = transform,
            Shape::Cube(c) => c.transform = transform,
            Shape::Cylinder(c) => c.transform = transform,
            Shape::Cone(c) => c.transform = transform,
            Shape::Group(g) => g.write().unwrap().transform = transform,
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
            Shape::Cone(c) => c.local_normal_at(local_point),
            Shape::Group(g) => g.read().unwrap().local_normal_at(local_point),
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
            Shape::Group(g) => g.write().unwrap().set_material(material),
        }
    }

    // // TODO pub needed?
    // pub fn get_parent(&self) -> Option<Arc<RwLock<Group>>> {
    //     match self {
    //         Shape::Sphere(s) => s.parent,
    //         Shape::Plane(p) => p.parent,
    //         Shape::Cube(c) => c.parent,
    //         Shape::Cylinder(c) => c.parent,
    //         Shape::Cone(c) => c.parent,
    //         Shape::Group(g) => g.read().unwrap().parent,
    //     }
    // }

    pub fn set_parent(&mut self, parent: Option<Arc<RwLock<Group>>>) {
        match self {
            Shape::Sphere(s) => s.parent = parent,
            Shape::Plane(p) => p.parent = parent,
            Shape::Cube(c) => c.parent = parent,
            Shape::Cylinder(c) => c.parent = parent,
            Shape::Cone(c) => c.parent = parent,
            Shape::Group(g) => g.write().unwrap().parent = parent,
        }
    }

    // TODO pub needed?
    pub fn world_to_object(&self, point: Tuple) -> Tuple {
        let mut p = point;
        // if let Some(parent) = self.get_parent() {
        //     p = parent.world_to_object(point);
        // }
        // if !parents.is_empty() {
        //     parents[0].world_to_object(&parents[1..], point);
        // }
        self.transform().inverse().unwrap() * p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::MaterialBuilder, matrix::IDENTITY_MATRIX, point, sphere};
    use rand::Rng;
    use std::{f64::consts::PI, sync::RwLock};

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
        // let mut s = sphere!();
        // let raw_sphere_ptr: *const Shape = &s; // needed to call the world_to_object method below

        // s.set_transform(Matrix::translation(5., 0., 0.));

        // let mut g2 = group!(s);
        // g2.set_transform(Matrix::scaling(2., 2., 2.));

        // let mut g1 = group!(g2);
        // g1.set_transform(Matrix::rotation_y(PI / 2.));
        // let _p = unsafe { (*raw_sphere_ptr).world_to_object(point!(-2., 0., -10.)) };
        // assert_eq!(point!(0., 0., -1.), p);
    }

    enum Shp {
        Group(Arc<RwLock<Group>>),
        Node(Node),
    }

    struct Group {
        pub parent: Option<Arc<RwLock<Group>>>,
        pub children: Vec<Shp>,
    }

    struct Node {
        pub parent: Option<Arc<RwLock<Group>>>,
    }

    #[test]
    fn self_reference() {
        let root_rc = Arc::new(RwLock::new(Group {
            parent: None,
            children: Vec::new(),
        }));

        let child_group = Arc::new(RwLock::new(Group {
            parent: Some(root_rc.clone()),
            children: Vec::new(),
        }));

        root_rc
            .write()
            .unwrap()
            .children
            .push(Shp::Group(child_group.clone()));

        let node = Node {
            parent: Some(child_group.clone()),
        };
        child_group.write().unwrap().children.push(Shp::Node(node));
    }
}
