use crate::{ray::Ray, shapes::Shape, tuple::Tuple, EPSILON};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

#[derive(Debug)]
pub struct PreparedComputations<'a> {
    pub t: f64,
    pub object: &'a Shape,
    pub point: Tuple,
    pub under_point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl Intersection<'_> {
    pub fn new<'a>(t: f64, object: &'a Shape) -> Intersection<'a> {
        Intersection { t, object }
    }

    // pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    pub fn hit<'a>(xs: &[&'a Intersection<'a>]) -> Option<&'a Intersection<'a>> {
        let mut min = f64::MAX;
        let mut response: Option<&'a Intersection<'a>> = None;
        for i in xs {
            if i.t >= 0. && i.t < min {
                min = i.t;
                response = Some(i);
            }
        }
        response
    }

    pub fn prepare_computations(&self, r: &Ray, xs: &[&Intersection]) -> PreparedComputations {
        let point = r.position(self.t);
        let eyev = -r.direction;
        let temp_normalv = self.object.normal_at(point);
        let (inside, normalv) = if temp_normalv.dot(&eyev) < 0. {
            (true, -temp_normalv)
        } else {
            (false, temp_normalv)
        };
        let under_point = point - normalv * EPSILON;
        let over_point = point + normalv * EPSILON;
        let reflectv = r.direction.reflect(normalv);

        // compute n1 and n2
        let mut n1 = 1.;
        let mut n2 = 1.;
        let mut containers: Vec<&Shape> = vec![];
        for i in xs {
            if self == *i {
                if containers.is_empty() {
                    n1 = 1.;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index;
                }
            }
            match containers.iter().position(|&o| o == i.object) {
                Some(pos) => {
                    containers.remove(pos);
                }
                None => {
                    containers.push(i.object);
                }
            }
            if self == *i {
                if containers.is_empty() {
                    n2 = 1.;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
                }
                break;
            }
        }

        PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            under_point,
            over_point,
            eyev,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
        }
    }
}

impl PreparedComputations<'_> {
    pub fn schlick(&self) -> f64 {
        //  find the cosine of the angle between the eye and normal vectors​
        let mut cos = self.eyev.dot(&self.normalv);
        // total internal reflection can only occur if n1 > n2​
        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = (n_ratio * n_ratio) * (1. - (cos * cos));
            if sin2_t > 1. {
                return 1.;
            }
            // when n1 > n2, use cos(theta_t) instead​
            cos = (1. - sin2_t).sqrt();
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        approx_eq, color::BLACK, material::MaterialBuilder, matrix::Matrix, patterns::Pattern,
        point, ray, shapes::sphere::Sphere, solid, sphere, vector,
    };

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

        let i = Intersection::hit(&[&i2, &i1]);
        assert_eq!(Some(&i1), i);
    }

    #[test]
    fn hit_negative() {
        let s = sphere!();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(2., &s);

        let i = Intersection::hit(&[&i2, &i1]);
        assert_eq!(Some(&i2), i);
    }

    #[test]
    fn hit_all_negative() {
        let s = sphere!();
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);

        let i = Intersection::hit(&[&i2, &i1]);
        assert_eq!(None, i);
    }
    #[test]
    fn hit_lowest_non_negative() {
        let s = sphere!();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);

        let i = Intersection::hit(&[&i1, &i2, &i3, &i4]);
        assert_eq!(Some(&i4), i);
    }

    #[test]
    fn precompute() {
        let r = ray!(point!(0., 0., -5.), vector!(0., 0., 1.));
        let s = sphere!();
        let i = Intersection::new(4., &s);
        let comps = i.prepare_computations(&r, &[&i]);
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
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(i.t, comps.t);
        assert!(i.object == comps.object);
        assert!(!comps.inside);
    }

    #[test]
    fn inside() {
        let r = ray!(point!(0., 0., 0.), vector!(0., 0., 1.));
        let s = sphere!();
        let i = Intersection::new(1., &s);
        let comps = i.prepare_computations(&r, &[&i]);
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
        let comps = i.prepare_computations(&r, &[&i]);
        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn finding_n1_and_n2() {
        let mut a = sphere!();
        a.set_transform(Matrix::scaling(2., 2., 2.));
        a.set_material(
            MaterialBuilder::default()
                .transparency(1.)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        let mut b = sphere!();
        b.set_transform(Matrix::translation(0., 0., -0.25));
        b.set_material(
            MaterialBuilder::default()
                .transparency(1.)
                .refractive_index(2.)
                .build()
                .unwrap(),
        );
        let mut c = sphere!();
        c.set_transform(Matrix::translation(0., 0., 0.25));
        c.set_material(
            MaterialBuilder::default()
                .transparency(1.)
                .refractive_index(2.5)
                .build()
                .unwrap(),
        );
        let r = ray!(0., 0., -4.; 0., 0., 1.);
        let xs = vec![
            Intersection::new(2., &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6., &a),
        ];
        let xs_refs = xs.iter().collect::<Vec<&Intersection>>();

        let expected_n1_n2s = vec![
            (1., 1.5),
            (1.5, 2.),
            (2., 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (i, intersection) in xs.iter().enumerate() {
            let comps = intersection.prepare_computations(&r, &xs_refs[..]);
            let (expected_n1, expected_n2) = expected_n1_n2s[i];
            println!(
                "i: {}, t: {}, expected_n1: {}, n1: {}, expected_n2: {}, n2: {}",
                i, intersection.t, expected_n1, comps.n1, expected_n2, comps.n2
            );
            assert_eq!(expected_n1, comps.n1, "i = {}, t = {}", i, intersection.t);
            assert_eq!(expected_n2, comps.n2, "i = {}, t = {}", i, intersection.t);
        }
    }

    #[test]
    fn under_point_is_below_surface() {
        let r = ray!(0., 0., -5.; 0., 0., 1.);
        let mut s = sphere!();
        s.set_transform(Matrix::translation(0., 0., 1.));
        let i = Intersection::new(5., &s);
        let comps = i.prepare_computations(&r, &[&i]);
        assert!(comps.under_point.z > -EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let mut shape = sphere!();
        shape.set_material(
            MaterialBuilder::default()
                .pattern(solid!(BLACK))
                .transparency(1.)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        let r = ray!(0., 0., 2f64.sqrt() / 2.; 0., 1., 0.);
        let i1 = Intersection::new(-2f64.sqrt() / 2., &shape);
        let i2 = Intersection::new(2f64.sqrt() / 2., &shape);
        let comps = i2.prepare_computations(&r, &[&i1, &i2]);
        let reflectance = comps.schlick();
        assert_eq!(1., reflectance);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let mut shape = sphere!();
        shape.set_material(
            MaterialBuilder::default()
                .pattern(solid!(BLACK))
                .transparency(1.)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        let r = ray!(0., 0., 0.; 0., 1., 0.);
        let i1 = Intersection::new(-1., &shape);
        let i2 = Intersection::new(1., &shape);
        let comps = i2.prepare_computations(&r, &[&i1, &i2]);
        let reflectance = comps.schlick();
        assert!(approx_eq(0.04, reflectance));
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let mut shape = sphere!();
        shape.set_material(
            MaterialBuilder::default()
                .pattern(solid!(BLACK))
                .transparency(1.)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        let r = ray!(0., 0.99, -2.; 0., 0., 1.);
        let i = Intersection::new(1.8589, &shape);
        let comps = i.prepare_computations(&r, &[&i]);
        let reflectance = comps.schlick();
        assert!(approx_eq(0.48873, dbg!(reflectance)));
    }
}
