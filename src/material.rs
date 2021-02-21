use crate::{
    color::{Color, BLACK, WHITE},
    light::PointLight,
    patterns::Pattern,
    shapes::Shape,
    solid,
    tuple::Tuple,
};

const DEFAULT_MATERIAL: Material = Material {
    pattern: solid!(WHITE),
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.,
    reflective: 0.,
};

#[derive(Copy, Clone, Debug, PartialEq, Builder)]
#[builder(default)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
}

impl Material {
    pub fn new(
        pattern: Pattern,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
    ) -> Self {
        Self {
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
        }
    }

    pub fn lightning(
        &self,
        object: &Shape,
        light: &PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.color_at_object(object, point);
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let light_dot_normal = lightv.dot(&normalv);

        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let diffuse;
        let specular;
        if light_dot_normal < 0. {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0. {
                specular = BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        DEFAULT_MATERIAL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color,
        color::GREEN,
        intersection::Intersection,
        patterns::StripePattern,
        plane, point, ray,
        ray::Ray,
        shapes::{Plane, Sphere},
        sphere, stripe_pattern, vector,
    };

    #[test]
    fn default() {
        let material = Material::default();
        assert_eq!(solid!(WHITE), material.pattern);
        assert_eq!(0.1, material.ambient);
        assert_eq!(0.9, material.diffuse);
        assert_eq!(0.9, material.specular);
        assert_eq!(200., material.shininess);

        let mut material = DEFAULT_MATERIAL;
        material.ambient = 0.5;
        assert_eq!(0.5, material.ambient);
        assert_eq!(0.1, DEFAULT_MATERIAL.ambient);
    }

    #[test]
    fn builder() {
        let m1 = MaterialBuilder::default()
            .pattern(solid!(GREEN))
            .build()
            .unwrap();
        assert_eq!(solid!(GREEN), m1.pattern);
        assert_eq!(0.1, m1.ambient);
        assert_eq!(0.9, m1.diffuse);
        assert_eq!(0.9, m1.specular);
        assert_eq!(200., m1.shininess);

        let m2 = MaterialBuilder::default().ambient(0.2).build().unwrap();
        assert_eq!(solid!(WHITE), m2.pattern);
        assert_eq!(0.2, m2.ambient);
        assert_eq!(0.9, m2.diffuse);
        assert_eq!(0.9, m2.specular);
        assert_eq!(200., m2.shininess);
    }

    #[test]
    fn lightning_eye_between_light_and_surface() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);

        let result = material.lightning(&object, &light, position, eyev, normalv, false);
        assert_eq!(color!(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lightning_eye_between_light_and_surface_eye_offset_45_deg() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);

        let result = material.lightning(&object, &light, position, eyev, normalv, false);
        assert_eq!(WHITE, result);
    }

    #[test]
    fn lightning_eye_opposite_surface_light_offset_45_deg() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 10., -10.), WHITE);

        let result = material.lightning(&object, &light, position, eyev, normalv, false);
        assert_eq!(color!(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lightning_eye_in_path_of_reflecting_vector() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 10., -10.), WHITE);

        let result = material.lightning(&object, &light, position, eyev, normalv, false);
        assert_eq!(color!(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lightning_light_behind_surface() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., 10.), WHITE);

        let result = material.lightning(&object, &light, position, eyev, normalv, false);
        assert_eq!(color!(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lightning_with_surface_in_shadow() {
        let material = Material::default();
        let position = point!(0., 0., 0.);
        let object = sphere!();

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);
        let in_shadow = true;

        let result = material.lightning(&object, &light, position, eyev, normalv, in_shadow);
        assert_eq!(color!(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lightning_with_pattern_applied() {
        let material = MaterialBuilder::default()
            .pattern(stripe_pattern!(WHITE, BLACK))
            .ambient(1.)
            .diffuse(0.)
            .specular(0.)
            .build()
            .unwrap();
        let object = sphere!();

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);
        let in_shadow = false;

        let c1 = material.lightning(
            &object,
            &light,
            point!(0.9, 0., 0.),
            eyev,
            normalv,
            in_shadow,
        );
        let c2 = material.lightning(
            &object,
            &light,
            point!(1.1, 0., 0.),
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(WHITE, c1);
        assert_eq!(BLACK, c2);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = plane!();
        let r = ray!(
            point!(0., 1., -1.),
            vector!(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.)
        );
        let i = Intersection::new(2f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(
            vector!(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.),
            comps.reflectv
        )
    }
}
