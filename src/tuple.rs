use core::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt;

use crate::{approx_eq, matrix::Matrix};

#[macro_export]
macro_rules! point {
    () => {
        $crate::tuple::Tuple::point(0, 0, 0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::tuple::Tuple::point($x, $y, $z)
    };
}
#[macro_export]
macro_rules! vector {
    () => {
        $crate::tuple::Tuple::vector(0, 0, 0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::tuple::Tuple::vector($x, $y, $z)
    };
}

#[derive(Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, w: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
    pub fn point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), 1)
    }
    pub fn vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::new(x.into(), y.into(), z.into(), 0)
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }
    pub fn to_point(&self) -> Self {
        Self::point(self.x, self.y, self.z)
    }
    pub fn to_vector(&self) -> Self {
        Self::vector(self.x, self.y, self.z)
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
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * 2. * self.dot(&normal)
    }
}

impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let struct_name = if self.is_point() {
            "point"
        } else if self.is_vector() {
            "vector"
        } else {
            "tuple"
        };
        let mut ds = f.debug_tuple(struct_name);
        ds.field(&self.x).field(&self.y).field(&self.z);
        if !self.is_point() && !self.is_vector() {
            ds.field(&self.w);
        }
        ds.finish()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x)
            && approx_eq(self.y, other.y)
            && approx_eq(self.z, other.z)
            && self.w == other.w
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
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
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, other: Matrix) -> Tuple {
        other * self
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Self {
        Tuple::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point() {
        let p = Tuple::new(4.3, -4.2, 3.1, 1);

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
    fn is_vector() {
        let v = Tuple::new(4.3, -4.2, 3.1, 0);

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
    fn eq() {
        let p = point!(4.3, -4.2, 3.1);
        let p2 = point!(4.3, -4.2, 3.1);
        assert_eq!(p, p2);
    }
    #[test]
    fn ne() {
        let p = point!(4.3, -4.2, 3.1);
        let v = vector!(4.3, -4.2, 3.1);
        assert_ne!(p, v);
    }

    #[test]
    fn add() {
        let p = point!(1, 2, 3);
        let v = vector!(1, 2, 3);
        let result = p + v;
        assert_eq!(point!(2, 4, 6), result);
    }

    #[test]
    fn sub() {
        let p = point!(1, 2, 3);
        let p2 = point!(1, 2, 3);
        let result = p - p2;
        assert_eq!(vector!(0, 0, 0), result);
    }
    #[test]
    fn neg() {
        let t = point!(1, 2, 3);
        let result = -t;
        assert_eq!(Tuple::new(-1, -2, -3, -1), result);
    }

    #[test]
    fn mul() {
        let v = vector!(1, 2, 3);
        let result = v * 2.;
        assert_eq!(vector!(2, 4, 6), result);
    }
    #[test]
    fn div() {
        let v = vector!(1, 2, 3);
        let result = v / 2.;
        assert_eq!(vector!(0.5, 1, 1.5), result);
    }
    #[test]
    fn magnitude() {
        let v = vector!(1, 0, 0);
        assert_eq!(1., v.magnitude());
        let v = vector!(1, 2, 3);
        assert_eq!(14_f64.sqrt(), v.magnitude());
    }

    #[test]
    fn normalize() {
        let v = vector!(4, 0, 0);
        assert_eq!(vector!(1, 0, 0), v.normalize());
        let v = vector!(1, 2, 3);
        assert_eq!(vector!(0.26726, 0.53452, 0.80178), v.normalize());
        assert_eq!(1., v.normalize().magnitude());
    }

    #[test]
    fn dot_product() {
        let v = vector!(1, 2, 3);
        let v2 = vector!(2, 3, 4);
        assert_eq!(20., v.dot(&v2));
    }

    #[test]
    fn cross_product() {
        let v = vector!(1, 2, 3);
        let v2 = vector!(2, 3, 4);
        assert_eq!(vector!(-1, 2, -1), v.cross(&v2));
        assert_eq!(vector!(1, -2, 1), v2.cross(&v));
    }

    #[test]
    fn reflecting_vector_approaching_at_45_deg() {
        let v = vector!(1, -1, 0);
        let n = vector!(0, 1, 0);
        let r = v.reflect(n);
        assert_eq!(vector!(1, 1, 0), r);
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector!(0, -1, 0);
        let n = vector!(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0);
        let r = v.reflect(n);
        assert_eq!(vector!(1, 0, 0), r);
    }
}
