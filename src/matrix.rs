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

const EMPTY_ROW: [f64; 4] = [0., 0., 0., 0.];

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
}
