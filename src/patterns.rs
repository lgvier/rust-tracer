use crate::{color::Color, tuple::Tuple};

#[macro_export]
macro_rules! solid {
    ($color:expr) => {
        Pattern::Solid($color)
    };
    ($r:expr, $g: expr, $b:expr) => {
        Pattern::Solid(Color::new($r, $g, $b));
    };
}

#[macro_export]
macro_rules! stripes {
    ($a:expr, $b:expr) => {
        Pattern::Stripes(StripePattern::new($a, $b))
    };
    // stripes!(0.1, 1., 0.5; 1., 0.8, 0.1)
    ($($a: expr),+; $($b: expr),+) => {
        Pattern::Stripes(StripePattern::new(Color::new($($a),*), Color::new($($b),*)))
    };
}

#[macro_export]
macro_rules! gradient {
    ($a:expr, $b:expr) => {
        Pattern::Gradient(GradientPattern::new($a, $b))
    };
    // gradient!(0.1, 1., 0.5; 1., 0.8, 0.1)
    ($($a: expr),+; $($b: expr),+) => {
        Pattern::Gradient(GradientPattern::new(Color::new($($a),*), Color::new($($b),*)))
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pattern {
    Solid(Color),
    Stripes(StripePattern),
    Gradient(GradientPattern),
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        match self {
            Pattern::Solid(color) => *color,
            Pattern::Stripes(pattern) => pattern.color_at(point),
            Pattern::Gradient(pattern) => pattern.color_at(point),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern { a, b }
    }

    fn color_at(&self, p: Tuple) -> Color {
        if p.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern { a, b }
    }

    fn color_at(&self, p: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = p.x - p.x.floor();
        self.a + (distance * fraction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color,
        color::{BLACK, WHITE},
        point,
    };

    #[test]
    fn stripes() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.a);
        assert_eq!(BLACK, pattern.b);
    }

    #[test]
    fn stripes_is_constant_in_y() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.color_at(point!(0., 1., 0.)));
        assert_eq!(WHITE, pattern.color_at(point!(0., 2., 0.)));
    }

    #[test]
    fn stripes_is_constant_in_z() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 1.)));
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 2.)));
    }

    #[test]
    fn stripes_alternates_in_x() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.color_at(point!(0.9, 0., 0.)));
        assert_eq!(BLACK, pattern.color_at(point!(1., 0., 0.)));
        assert_eq!(BLACK, pattern.color_at(point!(-0.1, 0., 0.)));
        assert_eq!(BLACK, pattern.color_at(point!(-1., 0., 0.)));
        assert_eq!(WHITE, pattern.color_at(point!(-1.1, 0., 0.)));
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 0.)));
        assert_eq!(
            color!(0.75, 0.75, 0.75),
            pattern.color_at(point!(0.25, 0., 0.))
        );
        assert_eq!(color!(0.5, 0.5, 0.5), pattern.color_at(point!(0.5, 0., 0.)));
        assert_eq!(
            color!(0.25, 0.25, 0.25),
            pattern.color_at(point!(0.75, 0., 0.))
        );
    }
}
