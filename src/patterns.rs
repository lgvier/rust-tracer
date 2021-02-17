use crate::{
    color::Color,
    matrix::{Matrix, IDENTITY_MATRIX},
    shapes::Shape,
    tuple::Tuple,
};

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
        Pattern::Stripes(StripePattern::new($a, $b))
    };
    // stripe_pattern!(0.1, 1., 0.5; 1., 0.8, 0.1)
    ($($a: expr),+; $($b: expr),+) => {
        Pattern::Stripes(StripePattern::new(Color::new($($a),*), Color::new($($b),*)))
    };
}

#[macro_export]
macro_rules! gradient_pattern {
    ($a:expr, $b:expr) => {
        Pattern::Gradient(GradientPattern::new($a, $b))
    };
    // gradient_pattern!(0.1, 1., 0.5; 1., 0.8, 0.1)
    ($($a: expr),+; $($b: expr),+) => {
        Pattern::Gradient(GradientPattern::new(Color::new($($a),*), Color::new($($b),*)))
    };
}

#[macro_export]
macro_rules! ring_pattern {
    ($a:expr, $b:expr) => {
        Pattern::Ring(RingPattern::new($a, $b))
    };
    // ring_pattern!(0.1, 1., 0.5; 1., 0.8, 0.1)
    ($($a: expr),+; $($b: expr),+) => {
        Pattern::Ring(RingPattern::new(Color::new($($a),*), Color::new($($b),*)))
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pattern {
    Solid(Color),
    Stripes(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
}

impl Pattern {
    pub fn color_at_object(&self, object: &Shape, world_point: Tuple) -> Color {
        match self {
            Pattern::Solid(color) => *color,
            Pattern::Stripes(pattern) => {
                pattern.color_at(self.to_pattern_point(object, world_point))
            }
            Pattern::Gradient(pattern) => {
                pattern.color_at(self.to_pattern_point(object, world_point))
            }
            Pattern::Ring(pattern) => pattern.color_at(self.to_pattern_point(object, world_point)),
        }
    }

    fn to_pattern_point(&self, object: &Shape, world_point: Tuple) -> Tuple {
        let object_point = object.transform().inverse().unwrap() * world_point;
        self.transform().inverse().unwrap() * object_point
    }

    pub fn transform(&self) -> &Matrix {
        match self {
            Pattern::Solid(_) => &IDENTITY_MATRIX,
            Pattern::Stripes(pattern) => &pattern.transform,
            Pattern::Gradient(pattern) => &pattern.transform,
            Pattern::Ring(pattern) => &pattern.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Pattern::Solid(_) => (),
            Pattern::Stripes(pattern) => pattern.transform = transform,
            Pattern::Gradient(pattern) => pattern.transform = transform,
            Pattern::Ring(pattern) => pattern.transform = transform,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    transform: Matrix,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern {
            a,
            b,
            transform: IDENTITY_MATRIX,
        }
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
    transform: Matrix,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern {
            a,
            b,
            transform: IDENTITY_MATRIX,
        }
    }

    fn color_at(&self, p: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = p.x - p.x.floor();
        self.a + (distance * fraction)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RingPattern {
    pub a: Color,
    pub b: Color,
    transform: Matrix,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        RingPattern {
            a,
            b,
            transform: IDENTITY_MATRIX,
        }
    }

    fn color_at(&self, p: Tuple) -> Color {
        if (p.x * p.x + p.z * p.z).sqrt().floor() % 2. == 0. {
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
        color,
        color::{BLACK, WHITE},
        point,
        shapes::{Shape, Sphere},
        sphere,
    };

    #[test]
    fn stripe_pattern() {
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
    fn stripes_with_object_transformation() {
        let mut object = sphere!();
        object.set_transform(Matrix::scaling(2., 2., 2.));
        let pattern = stripe_pattern!(WHITE, BLACK);
        let c = pattern.color_at_object(&object, point!(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = sphere!();
        let mut pattern = stripe_pattern!(WHITE, BLACK);
        pattern.set_transform(Matrix::scaling(2., 2., 2.));
        let c = pattern.color_at_object(&object, point!(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn stripes_with_object_and_pattern_transformations() {
        let mut object = sphere!();
        object.set_transform(Matrix::scaling(2., 2., 2.));
        let mut pattern = stripe_pattern!(WHITE, BLACK);
        pattern.set_transform(Matrix::translation(0.5, 0., 0.));
        let c = pattern.color_at_object(&object, point!(2.5, 0., 0.));
        assert_eq!(WHITE, c);
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

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(WHITE, BLACK);
        assert_eq!(WHITE, pattern.color_at(point!(0., 0., 0.)));
        assert_eq!(BLACK, pattern.color_at(point!(1., 0., 0.)));
        assert_eq!(BLACK, pattern.color_at(point!(0., 0., 1.)));
        assert_eq!(BLACK, pattern.color_at(point!(0.708, 0., 0.708)));
    }
}
