use crate::{
    color::{Color, BLACK, WHITE},
    light::PointLight,
    tuple::Tuple,
};

const DEFAULT_MATERIAL: Material = Material {
    color: WHITE,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.,
};

#[derive(Copy, Clone, Debug, PartialEq, Builder)]
#[builder(default)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lightning(
        &self,
        light: &PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let light_dot_normal = lightv.dot(&normalv);

        let ambient = effective_color * self.ambient;
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
    use crate::{color, point, vector};

    #[test]
    fn default() {
        let material = Material::default();
        assert_eq!(WHITE, material.color);
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
    fn lightning_eye_between_light_and_surface() {
        let material = Material::default();
        let position = point!(0., 0., 0.);

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);

        let result = material.lightning(&light, position, eyev, normalv);
        assert_eq!(color!(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lightning_eye_between_light_and_surface_eye_offset_45_deg() {
        let material = Material::default();
        let position = point!(0., 0., 0.);

        let eyev = vector!(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., -10.), WHITE);

        let result = material.lightning(&light, position, eyev, normalv);
        assert_eq!(WHITE, result);
    }

    #[test]
    fn lightning_eye_opposite_surface_light_offset_45_deg() {
        let material = Material::default();
        let position = point!(0., 0., 0.);

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 10., -10.), WHITE);

        let result = material.lightning(&light, position, eyev, normalv);
        assert_eq!(color!(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lightning_eye_in_path_of_reflecting_vector() {
        let material = Material::default();
        let position = point!(0., 0., 0.);

        let eyev = vector!(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 10., -10.), WHITE);

        let result = material.lightning(&light, position, eyev, normalv);
        assert_eq!(color!(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lightning_light_behind_surface() {
        let material = Material::default();
        let position = point!(0., 0., 0.);

        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(point!(0., 0., 10.), WHITE);

        let result = material.lightning(&light, position, eyev, normalv);
        assert_eq!(color!(0.1, 0.1, 0.1), result);
    }
}
