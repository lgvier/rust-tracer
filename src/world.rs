use crate::{
    color,
    color::{Color, BLACK, WHITE},
    intersection::{Intersection, PreparedComputations},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    point, point_light,
    ray::Ray,
    shapes::{Shape, Sphere},
    sphere,
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

    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
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
        comps
            .object
            .material()
            .lightning(&self.light, comps.point, comps.eyev, comps.normalv)
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        let hit = xs.iter().find(|i| i.t >= 0.);
        match hit {
            Some(i) => {
                let comps = i.prepare_computations(&r);
                self.shade_hit(&comps)
            }
            None => BLACK,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = point_light!(point!(-10., 10., -10.), WHITE);

        let mut s1 = sphere!();
        // TODO: Material builder?
        let s1_material = Material::default()
            .with_color(color!(0.8, 1., 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        s1.set_material(s1_material);

        let mut s2 = sphere!();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World::new(light, vec![Shape::Sphere(s1), Shape::Sphere(s2)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{intersection, ray, tuple::Tuple, vector};

    #[test]
    fn world_intersect_with_ray() {
        let w = World::default();
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let xs = w.intersect(r);
        println!("xs length: {:?}", xs.len());
        println!("xs: {:?}", xs);
        assert_eq!(4, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6., xs[3].t);
    }

    #[test]
    fn world_shading_an_intersection() {
        let w = World::default();
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let s = &w.objects[0];
        let i = intersection!(4., s);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(color!(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn world_color_intersection_behind_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.set_material(outer.material().with_ambient(1.));
        let inner = &mut w.objects[1];
        let inner_color = inner.material().color;
        inner.set_material(inner.material().with_ambient(1.));

        let r = ray!(point!(0., 0., 0.75), vector!(0., 0., -1.));
        let c = &w.color_at(r);
        assert_eq!(inner_color, *c);
    }
}
