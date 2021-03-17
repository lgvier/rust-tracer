use crate::{matrix, matrix::Matrix, ray::Ray, tuple::Tuple};

// Fluent API
impl Tuple {
    pub fn translated(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Matrix::translation(x, y, z) * self
    }
    pub fn scaled(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Matrix::scaling(x, y, z) * self
    }
    pub fn rotated_x(self, r: impl Into<f64>) -> Self {
        Matrix::rotation_x(r) * self
    }
    pub fn rotated_y(self, r: impl Into<f64>) -> Self {
        Matrix::rotation_y(r) * self
    }
    pub fn rotated_z(self, r: impl Into<f64>) -> Self {
        Matrix::rotation_z(r) * self
    }
    pub fn sheared(
        self,
        xy: impl Into<f64>,
        xz: impl Into<f64>,
        yx: impl Into<f64>,
        yz: impl Into<f64>,
        zx: impl Into<f64>,
        zy: impl Into<f64>,
    ) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

impl Ray {
    pub fn translated(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Matrix::translation(x, y, z) * self
    }
    pub fn scaled(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Matrix::scaling(x, y, z) * self
    }
}

impl Matrix {
    pub fn translated(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::translation(x, y, z) * self
    }
    pub fn scaled(self, x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::scaling(x, y, z) * self
    }
    pub fn rotated_x(self, r: impl Into<f64>) -> Self {
        Self::rotation_x(r) * self
    }
    pub fn rotated_y(self, r: impl Into<f64>) -> Self {
        Self::rotation_y(r) * self
    }
    pub fn rotated_z(self, r: impl Into<f64>) -> Self {
        Self::rotation_z(r) * self
    }
    pub fn sheared(
        self,
        xy: impl Into<f64>,
        xz: impl Into<f64>,
        yx: impl Into<f64>,
        yz: impl Into<f64>,
        zx: impl Into<f64>,
        zy: impl Into<f64>,
    ) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

// Static methods
impl Matrix {
    pub fn translation(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        matrix![
            1., 0., 0., x.into();
            0., 1., 0., y.into();
            0., 0., 1., z.into();
            0., 0., 0., 1.]
    }
    pub fn scaling(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        matrix![
            x.into(), 0., 0., 0.;
            0., y.into(), 0., 0.;
            0., 0., z.into(), 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_x(r: impl Into<f64>) -> Self {
        let r = r.into();
        matrix![
            1., 0., 0., 0.;
            0., r.cos(), -r.sin(), 0.;
            0., r.sin(), r.cos(), 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_y(r: impl Into<f64>) -> Self {
        let r = r.into();
        matrix![
            r.cos(), 0., r.sin(), 0.;
            0., 1., 0., 0.;
            -r.sin(), 0., r.cos(), 0.;
            0., 0., 0., 1.]
    }
    pub fn rotation_z(r: impl Into<f64>) -> Self {
        let r = r.into();
        matrix![
            r.cos(), -r.sin(), 0., 0.;
            r.sin(), r.cos(), 0., 0.;
            0., 0., 1., 0.;
            0., 0., 0., 1.]
    }
    pub fn shearing(
        xy: impl Into<f64>,
        xz: impl Into<f64>,
        yx: impl Into<f64>,
        yz: impl Into<f64>,
        zx: impl Into<f64>,
        zy: impl Into<f64>,
    ) -> Self {
        matrix![
            1., xy.into(), xz.into(), 0.;
            yx.into(), 1., yz.into(), 0.;
            zx.into(), zy.into(), 1., 0.;
            0., 0., 0., 1.]
    }
    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(&upn);
        let true_up = left.cross(&forward);
        let orientation = matrix![
            left.x, left.y, left.z, 0.;
            true_up.x, true_up.y, true_up.z, 0.;
            -forward.x, -forward.y, -forward.z, 0.;
            0., 0., 0., 1.
        ];
        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::PI;

    use crate::{matrix::IDENTITY_MATRIX, point, ray, vector};

    #[test]
    fn translation() {
        let t = Matrix::translation(5, -3, 2);
        let p = point!(-3, 4, 5);
        assert_eq!(point!(2, 1, 7), t * p);

        // moves point in reverse
        let inv = t.inverse().unwrap();
        assert_eq!(point!(-8, 7, 3), inv * p);

        // translation doesnt affect vectors
        let v = vector!(-3, 4, 5);
        assert_eq!(v, t * v);
    }

    #[test]
    fn scaling() {
        let t = Matrix::scaling(2, 3, 4);
        let p = point!(-4, 6, 8);
        assert_eq!(point!(-8, 18, 32), t * p);

        let v = vector!(-4, 6, 8);
        assert_eq!(vector!(-8, 18, 32), t * v);

        let inv = t.inverse().unwrap();
        assert_eq!(vector!(-2, 2, 2), inv * v);
    }

    #[test]
    fn rotation_x() {
        let p = point!(0, 1, 0);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);
        assert_eq!(
            point!(0, 2f64.sqrt() / 2., 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(point!(0, 0, 1), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(0, 2f64.sqrt() / 2., -2f64.sqrt() / 2.), inv * p);
    }

    #[test]
    fn rotation_y() {
        let p = point!(0, 0, 1);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);
        assert_eq!(
            point!(2f64.sqrt() / 2., 0, 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(point!(1, 0, 0), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(-2f64.sqrt() / 2., 0, 2f64.sqrt() / 2.), inv * p);
    }

    #[test]
    fn rotation_z() {
        let p = point!(0, 1, 0);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);
        assert_eq!(
            point!(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0),
            half_quarter * p
        );
        assert_eq!(point!(-1, 0, 0), full_quarter * p);

        let inv = half_quarter.inverse().unwrap();
        assert_eq!(point!(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0), inv * p);
    }

    #[test]
    fn shearing() {
        let p = point!(2, 3, 4);
        {
            // x in proportion to y
            let t = Matrix::shearing(1, 0, 0, 0, 0, 0);
            assert_eq!(point!(5, 3, 4), t * p);
        }
        {
            // x in proportion to z
            let t = Matrix::shearing(0, 1, 0, 0, 0, 0);
            assert_eq!(point!(6, 3, 4), t * p);
        }
        {
            // y in proportion to x
            let t = Matrix::shearing(0, 0, 1, 0, 0, 0);
            assert_eq!(point!(2, 5, 4), t * p);
        }
        {
            // y in proportion to z
            let t = Matrix::shearing(0, 0, 0, 1, 0, 0);
            assert_eq!(point!(2, 7, 4), t * p);
        }
        {
            // z in proportion to x
            let t = Matrix::shearing(0, 0, 0, 0, 1, 0);
            assert_eq!(point!(2, 3, 6), t * p);
        }
        {
            // z in proportion to y
            let t = Matrix::shearing(0, 0, 0, 0, 0, 1);
            assert_eq!(point!(2, 3, 7), t * p);
        }
    }

    #[test]
    fn chaining() {
        let p = point!(1, 0, 1);
        let a = Matrix::rotation_x(PI / 2.);
        let b = Matrix::scaling(5, 5, 5);
        let c = Matrix::translation(10, 5, 7);
        let expected = point!(15, 0, 7);

        // individual transformations
        let p2 = a * p;
        assert_eq!(point!(1, -1, 0), p2);
        let p3 = b * p2;
        assert_eq!(point!(5, -5, 0), p3);
        let p4 = c * p3;
        assert_eq!(expected, p4);

        // combined transformations
        let t = c * b * a;
        assert_eq!(expected, t * p);

        // fluent API
        let t_fluent = IDENTITY_MATRIX
            .rotated_x(PI / 2.)
            .scaled(5, 5, 5)
            .translated(10, 5, 7);
        assert_eq!(t, t_fluent);
        assert_eq!(expected, t_fluent * p);
    }

    #[test]
    fn ray_translation() {
        let r = ray!(1, 2, 3; 0, 1, 0);
        let r2 = r.translated(3, 4, 5);

        assert_eq!(point!(4, 6, 8), r2.origin);
        assert_eq!(vector!(0, 1, 0), r2.direction);
    }

    #[test]
    fn ray_scaling() {
        let r = ray!(1, 2, 3; 0, 1, 0);
        let r2 = r.scaled(2, 3, 4);

        assert_eq!(point!(2, 6, 12), r2.origin);
        assert_eq!(vector!(0, 3, 0), r2.direction);
    }

    #[test]
    fn matrix_for_default_orientation() {
        let from = point!(0, 0, 0);
        let to = point!(0, 0, -1);
        let up = point!(0, 1, 0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(IDENTITY_MATRIX, t);
    }

    #[test]
    fn view_transformation_looking_positive_z_direction() {
        let from = point!(0, 0, 0);
        let to = point!(0, 0, 1);
        let up = point!(0, 1, 0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(Matrix::scaling(-1, 1, -1), t);
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point!(0, 0, 8);
        let to = point!(0, 0, 0);
        let up = point!(0, 1, 0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(Matrix::translation(0, 0, -8), t);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point!(1, 3, 2);
        let to = point!(4, -2, 8);
        let up = point!(1, 1, 0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(
            matrix![-0.50709, 0.50709, 0.67612, -2.36643;
                    0.76772, 0.60609,  0.12122, -2.82843;
                    -0.35857, 0.59761, -0.71714,  0.00000;
                    0.00000, 0.00000,  0.00000,  1.00000],
            t
        );
    }
}
