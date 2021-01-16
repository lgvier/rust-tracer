use super::shapes::{Shape, Sphere};
use super::sphere;

#[macro_export]
macro_rules! intersection {
    // ray!(origin, direction)
    ($t:expr, $object:expr) => {
        Intersection::new($t, $object)
    };
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

impl Intersection<'_> {
    pub fn new<'a>(t: f64, object: &'a Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn hit<'a>(xs: &'a Vec<&'a Intersection<'a>>) -> &'a Intersection<'a> {
        xs[0]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn intersection_ctor() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i = Intersection::new(3.5, shape);
        assert_eq!(3.5, i.t);
        // match &i.object {
        //     &Shape::Sphere(_) => {
        //         //  x = s == s2;
        //     }
        //     _ => {
        //         panic!("");
        //     }
        // }
    }

    #[test]
    fn intersection_hit_positive() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(1., shape);
        let i2 = intersection!(2., shape);
        let xs = vec![&i2, &i1];

        let i = Intersection::hit(&xs);
        assert_eq!(&i1, i);
    }

    #[test]
    fn intersection_hit_negative() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(-1., shape);
        let i2 = intersection!(2., shape);
        let xs = vec![&i2, &i1];

        let i = Intersection::hit(&xs);
        assert_eq!(&i2, i);
    }

    #[test]
    fn intersection_hit_all_negative() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(-2., shape);
        let i2 = intersection!(-1., shape);
        let xs = vec![&i2, &i1];

        let i = Intersection::hit(&xs);
        assert_eq!(&i2, i);
    }
}
