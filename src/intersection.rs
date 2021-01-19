use crate::shapes::Shape;

#[macro_export]
macro_rules! intersection {
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

    //pub fn hit<'a>(xs: &'a Vec<&'a Intersection<'a>>) -> Option<&'a Intersection<'a>> {
    pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
        let mut min = f64::MAX;
        let mut response: Option<Intersection> = None;
        for i in xs {
            if i.t >= 0. && i.t < min {
                min = i.t;
                response = Some(i);
            }
        }
        response
        // xs.iter()
        //     .filter(|i| i.t >= 0.)
        //     .fold(None, |acc, &i| match acc {
        //         Some(ai) => {
        //             if i.t < ai.t {
        //                 Some(i)
        //             } else {
        //                 acc
        //             }
        //         }
        //         None => Some(i),
        //     })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Sphere;
    use crate::sphere;

    #[test]
    fn intersection_ctor() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i = Intersection::new(3.5, shape);
        assert_eq!(3.5, i.t);
        assert!(shape == i.object);
    }

    #[test]
    fn intersection_hit_positive() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(1., shape);
        let i2 = intersection!(2., shape);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(Some(intersection!(1., shape)), i);
    }

    #[test]
    fn intersection_hit_negative() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(-1., shape);
        let i2 = intersection!(2., shape);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(Some(intersection!(2., shape)), i);
    }

    #[test]
    fn intersection_hit_all_negative() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(-2., shape);
        let i2 = intersection!(-1., shape);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(None, i);
    }
    #[test]
    fn intersection_hit_lowest_non_negative() {
        let s = sphere!();
        let shape = &Shape::Sphere(s);
        let i1 = intersection!(5., shape);
        let i2 = intersection!(7., shape);
        let i3 = intersection!(-3., shape);
        let i4 = intersection!(2., shape);
        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);
        assert_eq!(Some(intersection!(2., shape)), i);
    }
}
