use crate::{color::Color, tuple::Tuple};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::WHITE, point};

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = WHITE;
        let position = point!();
        let light = PointLight::new(position, intensity);
        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
