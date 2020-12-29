use crate::utils::approx_eq;
use core::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const RED: Color = Color {
    r: 1.,
    g: 0.,
    b: 0.,
};
pub const GREEN: Color = Color {
    r: 0.,
    g: 1.,
    b: 0.,
};
pub const BLUE: Color = Color {
    r: 0.,
    g: 0.,
    b: 1.,
};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    fn to_u8(c: f64) -> u8 {
        const MAX: f64 = 255.;
        (c * MAX).min(MAX).max(0.) as u8
    }
    pub fn write_as_u8_rgb(&self, buff: &mut Vec<u8>, index: usize) {
        buff[index] = Self::to_u8(self.r);
        buff[index + 1] = Self::to_u8(self.g);
        buff[index + 2] = Self::to_u8(self.b);
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.r, other.r) && approx_eq(self.g, other.g) && approx_eq(self.b, other.b)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Self {
        Color::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_ctor() {
        let c = Color::new(0., 0., 1.);
        assert_eq!(0., c.r);
        assert_eq!(0., c.g);
        assert_eq!(1., c.b);
    }

    #[test]
    fn color_eq() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.9, 0.6, 0.75);
        assert_eq!(c, c2);
    }
    #[test]
    fn color_ne() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_ne!(c, c2);
    }

    #[test]
    fn color_add() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c + c2;
        assert_eq!(Color::new(1.6, 0.7, 1.), result);
    }

    #[test]
    fn color_sub() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c - c2;
        assert_eq!(Color::new(0.2, 0.5, 0.5), result);
    }
    #[test]
    fn color_mul() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2.);
        assert_eq!(Color::new(0.1, 0.3, 0.04), c * Color::new(0.5, 1., 0.1));
    }
}
