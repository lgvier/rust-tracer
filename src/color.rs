use crate::approx_eq;
use core::ops::{Add, Div, Mul, Sub};
use pad::PadStr;
use std::str::FromStr;

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::color::Color::new($r, $g, $b)
    };
}

pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
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
pub const YELLOW: Color = Color {
    r: 1.,
    g: 1.,
    b: 0.,
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

impl FromStr for Color {
    type Err = std::num::ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        assert_eq!("#", &hex_code[0..1]);
        let hex_padded = hex_code.pad_to_width_with_char(7, '0');
        let r: u8 = u8::from_str_radix(&hex_padded[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_padded[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_padded[5..7], 16)?;
        Ok(Color::new(
            r as f64 / 255.,
            g as f64 / 255.,
            b as f64 / 255.,
        ))
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

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctor() {
        let c = Color::new(0., 0., 1.);
        assert_eq!(0., c.r);
        assert_eq!(0., c.g);
        assert_eq!(1., c.b);
    }

    #[test]
    fn eq() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.9, 0.6, 0.75);
        assert_eq!(c, c2);
    }
    #[test]
    fn ne() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_ne!(c, c2);
    }

    #[test]
    fn add() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c + c2;
        assert_eq!(Color::new(1.6, 0.7, 1.), result);
    }

    #[test]
    fn sub() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c - c2;
        assert_eq!(Color::new(0.2, 0.5, 0.5), result);
    }
    #[test]
    fn mul() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2.);
        assert_eq!(Color::new(0.1, 0.3, 0.04), c * Color::new(0.5, 1., 0.1));
    }
}
