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

#[macro_export]
macro_rules! cube {
    () => {
        Shape::Cube(Cube::new())
    };
}

#[macro_export]
macro_rules! cylinder {
    () => {
        Shape::Cylinder(Cylinder::new())
    };
    ($minimum:expr, $maximum:expr) => {
        Shape::Cylinder(Cylinder::new_with_min_max($minimum, $maximum))
    };
    ($minimum:expr, $maximum:expr, $closed:expr) => {
        Shape::Cylinder(Cylinder::new_with_min_max_closed(
            $minimum, $maximum, $closed,
        ))
    };
}

#[macro_export]
macro_rules! cone {
    () => {
        Shape::Cone(Cone::new())
    };
    ($minimum:expr, $maximum:expr) => {
        Shape::Cone(Cone::new_with_min_max($minimum, $maximum))
    };
    ($minimum:expr, $maximum:expr, $closed:expr) => {
        Shape::Cone(Cone::new_with_min_max_closed($minimum, $maximum, $closed))
    };
}

#[macro_export]
macro_rules! group {
    () => {
        Shape::Group(Group::new())
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
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
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
            Shape::Group(g) => g.local_intersect(&local_ray),
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

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let transform_inverse = self.transform().inverse().unwrap();
        let local_point = transform_inverse * p;
        let local_normal = match self {
            Shape::Sphere(s) => s.local_normal_at(local_point),
            Shape::Plane(p) => p.local_normal_at(local_point),
            Shape::Cube(c) => c.local_normal_at(local_point),
            Shape::Cylinder(c) => c.local_normal_at(local_point),
            Shape::Cone(c) => c.local_normal_at(local_point),
            Shape::Group(c) => c.local_normal_at(local_point),
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
            Shape::Group(g) => g.set_material(material),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::MaterialBuilder, matrix::IDENTITY_MATRIX, sphere};
    use rand::Rng;

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
}
