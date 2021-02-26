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
    MAX_REFLECTION_RECURSION,
};

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new(light: PointLight, objects: Vec<Shape>) -> Self {
        Self { light, objects }
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        self.color_at_internal(r, MAX_REFLECTION_RECURSION)
    }

    fn color_at_internal(&self, r: &Ray, remaining: usize) -> Color {
        let xs = self.intersect(&r);
        let xs_refs = xs.iter().collect::<Vec<&Intersection>>();

        match xs.iter().find(|i| i.t >= 0.) {
            Some(i) => {
                let comps = i.prepare_computations(&r, &xs_refs[..]);
                self.shade_hit(&comps, remaining)
            }
            None => BLACK,
        }
    }

    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in &self.objects {
            for t in object.intersect(r) {
                result.push(Intersection::new(t, object));
            }
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        result
    }

    fn shade_hit(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.object.material().lightning(
            comps.object,
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);
        surface + reflected + refracted
    }

    fn is_shadowed(&self, point: Tuple) -> bool {
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

    fn reflected_color(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        let reflective = comps.object.material().reflective;
        if reflective == 0.0 {
            return BLACK;
        }

        let reflect_ray = ray!(comps.over_point, comps.reflectv);
        let color = self.color_at_internal(&reflect_ray, remaining - 1);
        color * reflective
    }

    fn refracted_color(&self, comps: &PreparedComputations, remaining: usize) -> Color {
        if remaining <= 0 {
            return BLACK;
        }
        let transparency = comps.object.material().transparency;
        if transparency == 0. {
            return BLACK;
        }

        // detect total internal reflection using Snell's Law
        let n_ratio = comps.n1 / comps.n2;
        // cos(theta_i) is the same as the dot product of the two vectors​
        let cos_i = comps.eyev.dot(&comps.normalv);
        // Find sin(theta_t)^2 via trigonometric identity​
        let sin2_t = (n_ratio * n_ratio) * (1. - (cos_i * cos_i));
        if sin2_t > 1. {
            return BLACK;
        }

        let cos_t = (1. - sin2_t).sqrt();
        // Compute the direction of the refracted ray​
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        // Create the refracted ray​
        let refracted_ray = ray!(comps.under_point, direction);

        self.color_at_internal(&refracted_ray, remaining - 1) * transparency
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
    use crate::{
        color, color::RED, material::Material, patterns::TestPattern, plane, ray, shapes::Plane,
        tuple::Tuple, vector,
    };

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
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn color_intersection_behind_ray() {
        let mut w = World::default();
        let inner_color;
        {
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
            inner_color = match inner_material.pattern {
                Pattern::Solid(c) => c,
                _ => panic!("expected solid pattern"),
            };
            inner.set_material(inner_material);
        }

        let r = ray!(point!(0., 0., 0.75), vector!(0., 0., -1.));
        let c = w.color_at(&r);
        assert_eq!(inner_color, c);
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
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn reflected_color_non_reflective_material() {
        let mut w = World::default();
        {
            let shape = &mut w.objects[1];
            let material = Material {
                ambient: 1.,
                ..*shape.material()
            };
            shape.set_material(material);
        }

        let r = ray!(point!(0., 0., 0.), vector!(0., 0., 1.));
        let i = Intersection::new(1., &w.objects[1]);
        let comps = i.prepare_computations(&r, &[&i]);
        let color = w.reflected_color(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(BLACK, color);
    }

    #[test]
    fn reflected_color_reflective_material() {
        let mut w = World::default();
        {
            let mut shape = plane!();
            shape.set_material(MaterialBuilder::default().reflective(0.5).build().unwrap());
            shape.set_transform(Matrix::translation(0., -1., 0.));
            w.objects.push(shape);
        }

        let r = ray!(
            point!(0., 0., -3.),
            vector!(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        let i = Intersection::new(2f64.sqrt(), &w.objects.last().unwrap());
        let comps = i.prepare_computations(&r, &[&i]);
        let color = w.reflected_color(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0.19033, 0.23791, 0.14274), color);
    }

    #[test]
    fn shade_hit_reflective_material() {
        let mut w = World::default();
        {
            let mut shape = plane!();
            shape.set_material(MaterialBuilder::default().reflective(0.5).build().unwrap());
            shape.set_transform(Matrix::translation(0., -1., 0.));
            w.objects.push(shape);
        }
        let r = ray!(
            point!(0., 0., -3.),
            vector!(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        let i = Intersection::new(2f64.sqrt(), &w.objects.last().unwrap());
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0.87676, 0.92434, 0.82917), c);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces_doesnt_cause_infinite_recursion() {
        let light = PointLight::new(point!(0., 0., 0.), WHITE);
        let mut lower = plane!();
        lower.set_material(MaterialBuilder::default().reflective(1.).build().unwrap());
        lower.set_transform(Matrix::translation(0., -1., 0.));
        let mut upper = plane!();
        upper.set_material(MaterialBuilder::default().reflective(1.).build().unwrap());
        upper.set_transform(Matrix::translation(0., 1., 0.));
        let w = World::new(light, vec![lower, upper]);
        let r = ray!(point!(0., 0., 0.), vector!(0., 1., 0.));
        w.color_at(&r); // should terminate succesfully
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = World::default();
        {
            let mut shape = plane!();
            shape.set_material(MaterialBuilder::default().reflective(0.5).build().unwrap());
            shape.set_transform(Matrix::translation(0., -1., 0.));
            w.objects.push(shape);
        }

        let r = ray!(
            point!(0., 0., -3.),
            vector!(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        let i = Intersection::new(2f64.sqrt(), &w.objects.last().unwrap());
        let comps = i.prepare_computations(&r, &[&i]);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(BLACK, color);
    }

    #[test]
    fn find_refracted_color_opaque_object() {
        let w = World::default();
        let s = &w.objects[0];
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let i1 = Intersection::new(4., s);
        let i2 = Intersection::new(6., s);
        let comps = i1.prepare_computations(&r, &[&i1, &i2]);
        let c = w.refracted_color(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(BLACK, c);
    }

    #[test]
    fn refracted_color_max_recursion() {
        let mut w = World::default();
        {
            let shape = &mut w.objects[0];
            let material = Material {
                transparency: 1.,
                refractive_index: 1.5,
                ..*shape.material()
            };
            shape.set_material(material)
        }
        let s = &w.objects[0];
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let i1 = Intersection::new(4., s);
        let i2 = Intersection::new(6., s);
        let comps = i1.prepare_computations(&r, &[&i1, &i2]);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(BLACK, c);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        {
            let shape = &mut w.objects[0];
            let material = Material {
                transparency: 1.,
                refractive_index: 1.5,
                ..*shape.material()
            };
            shape.set_material(material)
        }
        let s = &w.objects[0];
        let r = ray!(point!(0., 0., 2f64.sqrt() / 2.), vector!(0., 1., 0.));
        let i1 = Intersection::new(-2f64.sqrt() / 2., s);
        let i2 = Intersection::new(2f64.sqrt() / 2., s);
        // NOTE: this time you're inside the sphere, so you need​
        // to look at the second intersection
        let comps = i2.prepare_computations(&r, &[&i1, &i2]);
        let c = w.refracted_color(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(BLACK, c);
    }

    #[test]
    fn refracted_color_refracted_ray() {
        let mut w = World::default();
        {
            let shape = &mut w.objects[0];
            let material = Material {
                ambient: 1.,
                // the test pattern will return a color based on the point of intersection,
                // which means the test can inspect the returned color to determine whether or not the ray was refracted
                pattern: Pattern::Test(TestPattern::new()),
                ..*shape.material()
            };
            shape.set_material(material)
        }
        {
            let shape = &mut w.objects[1];
            let material = Material {
                transparency: 1.,
                refractive_index: 1.5,
                ..*shape.material()
            };
            shape.set_material(material)
        }
        let r = ray!(point!(0., 0., 0.1), vector!(0., 1., 0.));
        let i1 = Intersection::new(-0.9899, &w.objects[0]);
        let i2 = Intersection::new(-0.4899, &w.objects[1]);
        let i3 = Intersection::new(0.4899, &w.objects[1]);
        let i4 = Intersection::new(0.9899, &w.objects[0]);
        let comps = i3.prepare_computations(&r, &[&i1, &i2, &i3, &i4]);
        let c = w.refracted_color(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0., 0.99887, 0.04722), c);
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();

        let mut floor = plane!();
        floor.set_transform(Matrix::translation(0., -1., 0.));
        floor.set_material(
            MaterialBuilder::default()
                .transparency(0.5)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        w.objects.push(floor);

        let mut ball = sphere!();
        ball.set_material(
            MaterialBuilder::default()
                .pattern(solid!(RED))
                .ambient(0.5)
                .build()
                .unwrap(),
        );
        ball.set_transform(Matrix::translation(0., -3.5, -0.5));
        w.objects.push(ball);

        let r = ray!(
            point!(0., 0., -3.),
            vector!(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        let i = Intersection::new(
            2f64.sqrt(),
            &w.objects[w.objects.len() - 2], /* floor */
        );
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, MAX_REFLECTION_RECURSION);
        assert_eq!(color!(0.93642, 0.68642, 0.68642), c);
    }
}
