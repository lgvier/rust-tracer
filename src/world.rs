use crate::{
    color::{Color, BLACK, WHITE},
    intersection::{Intersection, PreparedComputations},
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    patterns::Pattern,
    point, ray,
    ray::Ray,
    shapes::{Shape, Sphere},
    solid, sphere,
    tuple::Tuple,
};

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new(light: PointLight, objects: Vec<Shape>) -> Self {
        Self { light, objects }
    }

    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        // self.objects
        //     .iter()
        //     .flat_map(|object| {
        //         object
        //             .intersect(r)
        //             .iter()
        //             .map(move |t| Intersection::new(*t, object))
        //     })
        //     .sort...
        //     .collect()
        let mut result = vec![];
        for object in &self.objects {
            for t in object.intersect(r) {
                result.push(Intersection::new(t, object));
            }
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        result
    }

    pub fn shade_hit(&self, comps: &PreparedComputations) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.object.material().lightning(
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        )
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let xs = self.intersect(&r);
        match xs.iter().find(|i| i.t >= 0.) {
            Some(i) => {
                let comps = i.prepare_computations(&r);
                self.shade_hit(&comps)
            }
            None => BLACK,
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = ray!(point, direction);
        let xs = self.intersect(&r);
        match xs.iter().find(|i| i.t >= 0.) {
            Some(i) => i.t < distance,
            None => false,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(point!(-10., 10., -10.), WHITE);

        let mut s1 = sphere!();
        let s1_material = MaterialBuilder::default()
            .pattern(solid!(0.8, 1., 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build()
            .unwrap();
        s1.set_material(s1_material);

        let mut s2 = sphere!();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World::new(light, vec![s1, s2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, material::Material, ray, tuple::Tuple, vector};

    #[test]
    fn intersect_with_ray() {
        let w = World::default();
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let xs = w.intersect(&r);
        println!("xs length: {:?}", xs.len());
        println!("xs: {:?}", xs);
        assert_eq!(4, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6., xs[3].t);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let s = &w.objects[0];
        let i = Intersection::new(4., s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(color!(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn color_intersection_behind_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        let outer_material = Material {
            ambient: 1.,
            ..*outer.material()
        };
        outer.set_material(outer_material);
        let inner = &mut w.objects[1];
        let inner_material = Material {
            ambient: 1.,
            ..*inner.material()
        };
        let inner_color = match inner_material.pattern {
            Pattern::Solid(c) => c,
            _ => panic!("expected solid pattern"),
        };
        inner.set_material(inner_material);

        let r = ray!(point!(0., 0., 0.75), vector!(0., 0., -1.));
        let c = &w.color_at(&r);
        assert_eq!(inner_color, *c);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = point!(0., 10., 0.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = point!(10., -10., 10.);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = point!(-20., 20., -20.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = point!(-2., 2., -2.);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_intersection_in_shadow() {
        let light = PointLight::new(point!(0., 0., -10.), WHITE);
        let s1 = sphere!();
        let mut s2 = sphere!();
        s2.set_transform(Matrix::translation(0., 0., 10.));
        let w = World::new(light, vec![s1, s2]);

        let r = ray!(point!(0., 0., 5.), vector!(0., 0., 1.));
        let i = Intersection::new(4., &w.objects[1]);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(color!(0.1, 0.1, 0.1), c);
    }
}
