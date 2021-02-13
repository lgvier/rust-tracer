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
macro_rules! stripe_pattern {
    ($a:expr, $b:expr) => {
        Pattern::Stripe(StripePattern::new($a, $b))
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pattern {
    Solid(Color),
    Stripe(StripePattern),
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        match self {
            Pattern::Solid(color) => *color,
            Pattern::Stripe(pattern) => pattern.stripe_at(point),
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

    fn stripe_at(&self, p: Tuple) -> Color {
        if p.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color::{BLACK, WHITE},
        point,
    };

    #[test]
    fn stripe_pattern() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.a);
        assert_eq!(BLACK, pattern.b);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 1., 0.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 2., 0.)));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 0., 1.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 0., 2.)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.stripe_at(point!(0., 0., 0.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(0.9, 0., 0.)));
        assert_eq!(BLACK, pattern.stripe_at(point!(1., 0., 0.)));
        assert_eq!(BLACK, pattern.stripe_at(point!(-0.1, 0., 0.)));
        assert_eq!(BLACK, pattern.stripe_at(point!(-1., 0., 0.)));
        assert_eq!(WHITE, pattern.stripe_at(point!(-1.1, 0., 0.)));
    }
}
