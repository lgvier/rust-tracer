use super::matrix;
use super::matrix::Matrix;
use super::tuple::Tuple;

// Fluent API
impl Tuple {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z) * self
    }
    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z) * self
    }
    pub fn rotate_x(self, r: f64) -> Self {
        Matrix::rotation_x(r) * self
    }
    pub fn rotate_y(self, r: f64) -> Self {
        Matrix::rotation_y(r) * self
    }
    pub fn rotate_z(self, r: f64) -> Self {
        Matrix::rotation_z(r) * self
    }
    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

impl Matrix {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self::translation(x, y, z) * self
    }
    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Self::scaling(x, y, z) * self
    }
    pub fn rotate_x(self, r: f64) -> Self {
        Self::rotation_x(r) * self
    }
    pub fn rotate_y(self, r: f64) -> Self {
        Self::rotation_y(r) * self
    }
    pub fn rotate_z(self, r: f64) -> Self {
        Self::rotation_z(r) * self
    }
    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

// Static methods
impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        matrix![
            1., 0., 0., x;
            0., 1., 0., y;
            0., 0., 1., z;
            0., 0., 0., 1.]
    }
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        matrix![
            x, 0., 0., 0.;
            0., y, 0., 0.;
            0., 0., z, 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_x(r: f64) -> Self {
        matrix![
            1., 0., 0., 0.;
            0., r.cos(), -r.sin(), 0.;
            0., r.sin(), r.cos(), 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_y(r: f64) -> Self {
        matrix![
            r.cos(), 0., r.sin(), 0.;
            0., 1., 0., 0.;
            -r.sin(), 0., r.cos(), 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_z(r: f64) -> Self {
        matrix![
            r.cos(), -r.sin(), 0., 0.;
            r.sin(), r.cos(), 0., 0.;
            0., 0., 1., 0.;
            0., 0., 0., 1.]
    }
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        matrix![
            1., xy, xz, 0.;
            yx, 1., yz, 0.;
            zx, zy, 1., 0.;
            0., 0., 0., 1.]
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use matrix::IDENTITY_MATRIX;

    use crate::{point, vector};

    use super::*;

    #[test]
    fn transform_translation() {
        let t = Matrix::translation(5., -3., 2.);
        let p = point!(-3., 4., 5.);
        assert_eq!(point!(2., 1., 7.), t * p);

        // moves point in reverse
        let inv = t.inverse().unwrap();
        assert_eq!(point!(-8., 7., 3.), inv * p);

        // translation doesnt affect vectors
        let v = vector!(-3., 4., 5.);
        assert_eq!(v, t * v);
    }

    #[test]
    fn transform_scaling() {
        let t = Matrix::scaling(2., 3., 4.);
        let p = point!(-4., 6., 8.);
        assert_eq!(point!(-8., 18., 32.), t * p);

        let v = vector!(-4., 6., 8.);
        assert_eq!(vector!(-8., 18., 32.), t * v);

        let inv = t.inverse().unwrap();
        assert_eq!(vector!(-2., 2., 2.), inv * v);
    }

    #[test]
    fn transform_rotation_x() {
        let p = point!(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);
        assert_eq!(
            point!(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(point!(0., 0., 1.), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.), inv * p);
    }

    #[test]
    fn transform_rotation_y() {
        let p = point!(0., 0., 1.);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);
        assert_eq!(
            point!(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(point!(1., 0., 0.), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(-2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.), inv * p);
    }

    #[test]
    fn transform_rotation_z() {
        let p = point!(0., 1., 0.);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);
        assert_eq!(
            point!(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.),
            half_quarter * p
        );
        assert_eq!(point!(-1., 0., 0.), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.), inv * p);
    }

    #[test]
    fn transform_shearing() {
        let p = point!(2., 3., 4.);
        {
            // x in proportion to y
            let t = Matrix::shearing(1., 0., 0., 0., 0., 0.);
            assert_eq!(point!(5., 3., 4.), t * p);
        }
        {
            // x in proportion to z
            let t = Matrix::shearing(0., 1., 0., 0., 0., 0.);
            assert_eq!(point!(6., 3., 4.), t * p);
        }
        {
            // y in proportion to x
            let t = Matrix::shearing(0., 0., 1., 0., 0., 0.);
            assert_eq!(point!(2., 5., 4.), t * p);
        }
        {
            // y in proportion to z
            let t = Matrix::shearing(0., 0., 0., 1., 0., 0.);
            assert_eq!(point!(2., 7., 4.), t * p);
        }
        {
            // z in proportion to x
            let t = Matrix::shearing(0., 0., 0., 0., 1., 0.);
            assert_eq!(point!(2., 3., 6.), t * p);
        }
        {
            // z in proportion to y
            let t = Matrix::shearing(0., 0., 0., 0., 0., 1.);
            assert_eq!(point!(2., 3., 7.), t * p);
        }
    }

    #[test]
    fn transform_chaining() {
        let p = point!(1., 0., 1.);
        let a = Matrix::rotation_x(PI / 2.);
        let b = Matrix::scaling(5., 5., 5.);
        let c = Matrix::translation(10., 5., 7.);
        let expected = point!(15., 0., 7.);

        // individual transformations
        let p2 = a * p;
        assert_eq!(point!(1., -1., 0.), p2);
        let p3 = b * p2;
        assert_eq!(point!(5., -5., 0.), p3);
        let p4 = c * p3;
        assert_eq!(expected, p4);

        // combined transformations
        let t = c * b * a;
        assert_eq!(expected, t * p);

        // fluent API
        let t_fluent = IDENTITY_MATRIX
            .rotate_x(PI / 2.)
            .scale(5., 5., 5.)
            .translate(10., 5., 7.);
        assert_eq!(t, t_fluent);
        assert_eq!(expected, t_fluent * p);
    }
}
