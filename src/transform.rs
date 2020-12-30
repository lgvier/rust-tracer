use super::matrix::{Matrix, IDENTITY_MATRIX};
use super::tuple::Tuple;

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut m = IDENTITY_MATRIX;
        m[0][3] = x;
        m[1][3] = y;
        m[2][3] = z;
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_mul() {
        let t = Matrix::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);
        assert_eq!(Tuple::point(2., 1., 7.), t * p);

        // moves point in reverse
        let inv = t.inverse().unwrap();
        assert_eq!(Tuple::point(-8., 7., 3.), inv * p);

        // translation doesnt affect vectors
        let v = Tuple::vector(-3., 4., 5.);
        assert_eq!(v, t * v);
    }
}
