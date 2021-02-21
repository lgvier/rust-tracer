use crate::{ray::Ray, shapes::Shape, tuple::Tuple, EPSILON};

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

pub struct PreparedComputations<'a> {
    pub t: f64,
    pub object: &'a Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
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

    pub fn prepare_computations(&self, r: &Ray) -> PreparedComputations {
        let point = r.position(self.t);
        let eyev = -r.direction;
        let temp_normalv = self.object.normal_at(point);
        let (inside, normalv) = if temp_normalv.dot(&eyev) < 0. {
            (true, -temp_normalv)
        } else {
            (false, temp_normalv)
        };
        let over_point = point + normalv * EPSILON;
        let reflectv = r.direction.reflect(normalv);

        PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
            reflectv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix::Matrix, point, sphere, vector};
    use crate::{ray, shapes::Sphere};

    #[test]
    fn ctor() {
        let s = sphere!();
        let i = Intersection::new(3.5, &s);
        assert_eq!(3.5, i.t);
        assert!(&s == i.object);
    }

    #[test]
    fn hit_positive() {
        let s = sphere!();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(Some(Intersection::new(1., &s)), i);
    }

    #[test]
    fn hit_negative() {
        let s = sphere!();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(2., &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(Some(Intersection::new(2., &s)), i);
    }

    #[test]
    fn hit_all_negative() {
        let s = sphere!();
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(xs);
        assert_eq!(None, i);
    }
    #[test]
    fn hit_lowest_non_negative() {
        let s = sphere!();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(xs);
        assert_eq!(Some(Intersection::new(2., &s)), i);
    }

    #[test]
    fn precompute() {
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let s = sphere!();
        let i = Intersection::new(4., &s);
        let comps = i.prepare_computations(&r);
        assert_eq!(i.t, comps.t);
        assert!(i.object == comps.object);
        assert_eq!(point!(0., 0., -1.), comps.point);
        assert_eq!(vector!(0., 0., -1.), comps.eyev);
        assert_eq!(vector!(0., 0., -1.), comps.normalv);
    }

    #[test]
    fn outside() {
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let s = sphere!();
        let i = Intersection::new(4., &s);
        let comps = i.prepare_computations(&r);
        assert_eq!(i.t, comps.t);
        assert!(i.object == comps.object);
        assert!(!comps.inside);
    }

    #[test]
    fn inside() {
        let r = ray!(point!(0., 0., 0.), vector!(0., 0., 1.));
        let s = sphere!();
        let i = Intersection::new(1., &s);
        let comps = i.prepare_computations(&r);
        assert_eq!(i.t, comps.t);
        assert!(i.object == comps.object);
        assert_eq!(point!(0., 0., 1.), comps.point);
        assert_eq!(vector!(0., 0., -1.), comps.eyev);
        assert!(comps.inside);
        assert_eq!(vector!(0., 0., -1.), comps.normalv);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::translation(0., 0., 1.));
        let i = Intersection::new(5., &s);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
