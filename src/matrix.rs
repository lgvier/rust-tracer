use super::tuple::*;
use crate::utils::approx_eq;
use core::ops::{Index, Mul};

pub const IDENTITY_MATRIX: Matrix = Matrix {
    data: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ],
    size: 4,
};

const EMPTY_ROW: [f64; 4] = [0.; 4];

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    data: [[f64; 4]; 4],
    size: usize,
}

impl Matrix {
    pub fn empty(size: usize) -> Self {
        Self {
            data: [EMPTY_ROW, EMPTY_ROW, EMPTY_ROW, EMPTY_ROW],
            size,
        }
    }
    pub fn new(r0: [f64; 4], r1: [f64; 4], r2: [f64; 4], r3: [f64; 4]) -> Self {
        Self {
            data: [r0, r1, r2, r3],
            size: 4,
        }
    }
    pub fn new3(r0: [f64; 3], r1: [f64; 3], r2: [f64; 3]) -> Self {
        Self {
            data: [
                [r0[0], r0[1], r0[2], 0.],
                [r1[0], r1[1], r1[2], 0.],
                [r2[0], r2[1], r2[2], 0.],
                EMPTY_ROW,
            ],
            size: 3,
        }
    }
    pub fn new2(r0: [f64; 2], r1: [f64; 2]) -> Self {
        Self {
            data: [
                [r0[0], r0[1], 0., 0.],
                [r1[0], r1[1], 0., 0.],
                EMPTY_ROW,
                EMPTY_ROW,
            ],
            size: 2,
        }
    }
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::empty(self.size);
        for r in 0..self.size {
            for c in 0..self.size {
                m.data[c][r] = self.data[r][c];
            }
        }
        m
    }
    pub fn determinant(&self) -> f64 {
        if self.size > 2 {
            panic!("not supported yet");
        }
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Self {
        let size = self.size - 1;
        let mut m = Matrix::empty(size);
        for r in 0..size {
            for c in 0..size {
                let src_row = if r < remove_row { r } else { r + 1 };
                let src_col = if c < remove_col { c } else { c + 1 };
                m.data[r][c] = self.data[src_row][src_col];
            }
        }
        m
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, r: usize) -> &[f64] {
        &self.data[0..self.size][r][0..self.size]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && (0..self.size)
                .all(|r| (0..self.size).all(|c| approx_eq(self.data[r][c], other.data[r][c])))
    }
}

impl Default for Matrix {
    fn default() -> Self {
        IDENTITY_MATRIX
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self {
        let size = self.size;
        let mut m = Matrix::empty(size);
        for r in 0..size {
            for c in 0..size {
                m.data[r][c] = (0..size).map(|i| self.data[r][i] * other.data[i][c]).sum();
            }
        }
        m
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let dot = |i| {
            self[i][0] * other.x
                + self[i][1] * other.y
                + self[i][2] * other.z
                + self[i][3] * other.w
        };
        Tuple::new(dot(0), dot(1), dot(2), dot(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_ctor() {
        let m = Matrix::new(
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        );
        assert_eq!(1., m[0][0]);
        assert_eq!(4., m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11., m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);

        let m3 = Matrix::new3([1., 2., 3.], [5.5, 6.5, 7.5], [9., 10., 11.]);
        assert_eq!(11., m3[2][2]);
    }

    #[test]
    fn matrix_eq() {
        let m = Matrix::new(
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        );
        let m2 = Matrix::new(
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        );
        let m3 = Matrix::new(
            [0., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        );
        assert_eq!(m, m2);
        assert_ne!(m, m3);
    }

    #[test]
    fn matrix_mul() {
        let m = Matrix::new(
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        );
        let m2 = Matrix::new(
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        );
        let expected = Matrix::new(
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        );
        assert_eq!(expected, m * m2);
    }

    #[test]
    fn matrix_tuple_mul() {
        let m = Matrix::new(
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        );
        let t = Tuple::new(1., 2., 3., 1.);
        assert_eq!(Tuple::new(18., 24., 33., 1.), m * t);
    }

    #[test]
    fn matrix_identity() {
        let m = Matrix::new(
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        );
        assert_eq!(m, m * IDENTITY_MATRIX);
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix::new(
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        );
        let expected = Matrix::new(
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        );
        assert_eq!(expected, m.transpose());

        assert_eq!(IDENTITY_MATRIX, IDENTITY_MATRIX.transpose());
    }

    #[test]
    fn matrix_determinant() {
        let m = Matrix::new2([1., 5.], [-3., 2.]);
        assert_eq!(17., m.determinant());
    }

    #[test]
    fn matrix_submatrix() {
        let m = Matrix::new3([1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]);
        let expected = Matrix::new2([-3., 2.], [0., 6.]);
        assert_eq!(expected, m.submatrix(0, 2));
    }
}
