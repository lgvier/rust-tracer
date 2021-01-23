use crate::{color::Color, tuple::Tuple};

#[macro_export]
macro_rules! point_light {
    ($position:expr, $intensity:expr) => {
        PointLight::new($position, $intensity)
    };
}

#[derive(Clone, Copy, Debug)]
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
    use crate::{color::WHITE, point};

    use super::*;

    #[test]
    fn point_has_position_and_intensity() {
        let intensity = WHITE;
        let position = point!();
        let light = point_light!(position, intensity);
        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
