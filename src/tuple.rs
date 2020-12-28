use crate::utils::approx_eq;
use core::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.)
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) &&
        approx_eq(self.y, other.y) &&
        approx_eq(self.z, other.z) &&
        self.w == other.w
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Self {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Self {
        Tuple::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Self {
        Tuple::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let p = Tuple::new(4.3, -4.2, 3.1, 1.);

        assert_eq!(4.3, p.x);
        assert_eq!(-4.2, p.y);
        assert_eq!(3.1, p.z);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn point_ctor() {
        let p = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(4.3, p.x);
        assert_eq!(-4.2, p.y);
        assert_eq!(3.1, p.z);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let v = Tuple::new(4.3, -4.2, 3.1, 0.);

        assert_eq!(4.3, v.x);
        assert_eq!(-4.2, v.y);
        assert_eq!(3.1, v.z);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }
    #[test]
    fn vector_ctor() {
        let v = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(4.3, v.x);
        assert_eq!(-4.2, v.y);
        assert_eq!(3.1, v.z);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn tuple_eq() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        let p2 = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(p, p2);
    }
    
    #[test]
    fn tuple_ne() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        let v = Tuple::vector(4.3, -4.2, 3.1);
        assert_ne!(p, v);
    }

    #[test]
    fn tuple_add() {
        let p = Tuple::point(1., 2., 3.);
        let v = Tuple::vector(1., 2., 3.);
        let result = p + v;
        assert_eq!(Tuple::point(2., 4., 6.), result);
    }

    #[test]
    fn tuple_sub() {
        let p = Tuple::point(1., 2., 3.);
        let p2 = Tuple::point(1., 2., 3.);
        let result = p - p2;
        assert_eq!(Tuple::vector(0., 0., 0.), result);
    }
    
    #[test]
    fn tuple_neg() {
        let t = Tuple::point(1., 2., 3.);
        let result = -t;
        assert_eq!(Tuple::new(-1., -2., -3., -1.), result);
    }

    #[test]
    fn tuple_mul() {
        let v = Tuple::vector(1., 2., 3.);
        let result = v * 2.;
        assert_eq!(Tuple::vector(2., 4., 6.), result);
    }
    
    #[test]
    fn tuple_div() {
        let v = Tuple::vector(1., 2., 3.);
        let result = v / 2.;
        assert_eq!(Tuple::vector(0.5, 1., 1.5), result);
    }
    
    #[test]
    fn tuple_mag() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(1., v.magnitude());
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(14_f64.sqrt(), v.magnitude());
    }

    #[test]
    fn tuple_norm() {
        let v = Tuple::vector(4., 0., 0.);
        assert_eq!(Tuple::vector(1., 0., 0.), v.normalize());
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(Tuple::vector(0.26726, 0.53452, 0.80178), v.normalize());
        assert_eq!(1., v.normalize().magnitude());
    }

    #[test]
    fn tuple_dot_product() {
        let v = Tuple::vector(1., 2., 3.);
        let v2 = Tuple::vector(2., 3., 4.);
        assert_eq!(20., v.dot(&v2));
    }

    #[test]
    fn tuple_cross_product() {
        let v = Tuple::vector(1., 2., 3.);
        let v2 = Tuple::vector(2., 3., 4.);
        assert_eq!(Tuple::vector(-1., 2., -1.), v.cross(&v2));
        assert_eq!(Tuple::vector(1., -2., 1.), v2.cross(&v));
    }


    #[derive(Debug)]
    struct Projectile {
        position: Tuple,
        velocity: Tuple
    }
    struct Environment {
        gravity: Tuple,
        wind: Tuple
    }

    fn tick(e: &Environment, p: &Projectile) -> Projectile {
        let position = p.position + p.velocity;
        let velocity = p.velocity + e.gravity + e.wind;
        Projectile { position, velocity }
    }

    #[test]
    fn proj() {
        let mut p = Projectile { 
            position: Tuple::point(0., 1., 0.),
            velocity: Tuple::vector(1., 1., 0.).normalize()
        };
        let e = Environment {
            gravity: Tuple::vector(0., -0.1, 0.),
            wind: Tuple::vector(0.01, 0., 0.)
        };
        let mut count = 0;
        while p.position.y > 0. {
            println!("p: {:#?}", p);
            p = tick(&e, &p);
            count = count + 1;
        }
        println!("end p: {:#?}, count: {}", p, count);
    }

}
