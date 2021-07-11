use crate::math::tuple::Tuple;
use ray_tracer::approx_eq;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Row {
    inner: [f64; 4],
    size: usize,
}

impl ops::Index<usize> for Row {
    type Output = f64;
    fn index(&self, col: usize) -> &Self::Output {
        if col >= self.size {
            panic!("Index out of bounds!")
        }

        &self.inner[col]
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        (0..self.size).all(|col| approx_eq(self[col], other[col]))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    inner: [Row; 4],
    size: usize,
}

pub const IDENTITY_MATRIX: Matrix = Matrix {
    inner: [
        Row {
            inner: [1., 0., 0., 0.],
            size: 4,
        },
        Row {
            inner: [0., 1., 0., 0.],
            size: 4,
        },
        Row {
            inner: [0., 0., 1., 0.],
            size: 4,
        },
        Row {
            inner: [0., 0., 0., 1.],
            size: 4,
        },
    ],
    size: 4,
};

impl Matrix {
    pub fn create(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Self {
        Self {
            inner: [
                Row {
                    inner: row0,
                    size: 4,
                },
                Row {
                    inner: row1,
                    size: 4,
                },
                Row {
                    inner: row2,
                    size: 4,
                },
                Row {
                    inner: row3,
                    size: 4,
                },
            ],
            size: 4,
        }
    }

    pub fn create2(row0: [f64; 2], row1: [f64; 2]) -> Self {
        Self {
            inner: [
                Matrix::force_array2(row0),
                Matrix::force_array2(row1),
                Matrix::force_array2([0., 0.]),
                Matrix::force_array2([0., 0.]),
            ],
            size: 2,
        }
    }

    pub fn create3(row0: [f64; 3], row1: [f64; 3], row2: [f64; 3]) -> Self {
        Self {
            inner: [
                Matrix::force_array3(row0),
                Matrix::force_array3(row1),
                Matrix::force_array3(row2),
                Matrix::force_array3([0., 0., 0.]),
            ],
            size: 3,
        }
    }

    fn force_array2(array: [f64; 2]) -> Row {
        Row {
            inner: [array[0], array[1], 0., 0.],
            size: 2,
        }
    }

    fn force_array3(array: [f64; 3]) -> Row {
        Row {
            inner: [array[0], array[1], array[2], 0.],
            size: 3,
        }
    }

    fn tuple(&self, row: usize) -> Tuple {
        let r = &self[row];
        Tuple::create(r[0], r[1], r[2], r[3])
    }

    fn empty(&self) -> Self {
        match self.size {
            2 => Matrix::create2([0., 0.], [0., 0.]),
            3 => Matrix::create3([0., 0., 0.], [0., 0., 0.], [0., 0., 0.]),
            4 => Matrix::create(
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
            ),
            _ => {
                panic!("Unknown dimension {}", self.size)
            }
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.inner[row].inner[col] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = self.empty();
        let size = self.size;
        for row in 0..size {
            for col in 0..size {
                matrix.set(col, row, self[row][col]);
            }
        }
        matrix
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, row: usize) -> &Self::Output {
        if row >= self.size {
            panic!("Index out of bounds!")
        }
        &self.inner[row]
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut matrix = self.empty();
        let size = self.size;
        for row in 0..size {
            for col in 0..size {
                let a = (0..size).map(|i| self[row][i] * rhs[i][col]).sum();
                matrix.set(row, col, a);
            }
        }
        matrix
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::create(
            self.tuple(0).dot(rhs),
            self.tuple(1).dot(rhs),
            self.tuple(2).dot(rhs),
            self.tuple(3).dot(rhs),
        )
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && (0..self.size).all(|row| self[row] == other[row])
    }
}

#[test]
fn construct_4x4_matrix() {
    let m = Matrix::create(
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
}

#[test]
fn construct_2x2_matrix() {
    let m = Matrix::create2([-3., 5.], [1., -2.]);

    assert_eq!(-3., m[0][0]);
    assert_eq!(5., m[0][1]);
    assert_eq!(1., m[1][0]);
    assert_eq!(-2., m[1][1]);
}

#[test]
fn construct_3x3_matrix() {
    let m = Matrix::create3([-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]);

    assert_eq!(-3., m[0][0]);
    assert_eq!(-2., m[1][1]);
    assert_eq!(1., m[2][2]);
}

#[test]
fn identical_matrices() {
    let a = Matrix::create(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    );
    let b = Matrix::create(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    );

    assert_eq!(a, b);
}

#[test]
fn not_identical_matrices() {
    let a = Matrix::create(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    );
    let b = Matrix::create(
        [2., 3., 4., 5.],
        [6., 7., 8., 9.],
        [8., 7., 6., 5.],
        [4., 3., 2., 1.],
    );

    assert_ne!(a, b);
}

#[test]
fn multiplying_two_matrices() {
    let a = Matrix::create(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    );
    let b = Matrix::create(
        [-2., 1., 2., 3.],
        [3., 2., 1., -1.],
        [4., 3., 6., 5.],
        [1., 2., 7., 8.],
    );
    let expected = Matrix::create(
        [20., 22., 50., 48.],
        [44., 54., 114., 108.],
        [40., 58., 110., 102.],
        [16., 26., 46., 42.],
    );

    let result = a * b;
    assert_eq!(expected, result);
}

#[test]
fn tuple_from_matrix_row() {
    let a = Matrix::create(
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    );
    let b = Tuple::create(5., 4., 3., 2.);

    assert_eq!(b, a.tuple(3));
}

#[test]
fn multiply_matrix_by_tuple() {
    let a = Matrix::create(
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    );
    let b = Tuple::create(1., 2., 3., 1.);
    let expected = Tuple::create(18., 24., 33., 1.);

    assert_eq!(expected, a * b);
}

#[test]
fn multiply_matrix_with_identity_matrix() {
    let m = Matrix::create(
        [0., 1., 2., 4.],
        [1., 2., 4., 8.],
        [2., 4., 8., 16.],
        [4., 8., 16., 32.],
    );

    assert_eq!(m, m * IDENTITY_MATRIX);
}

#[test]
fn multiply_identity_matrix_by_a_tuple() {
    let a = Tuple::create(1., 2., 3., 4.);
    assert_eq!(IDENTITY_MATRIX * a, a);
}

#[test]
fn transpose_a_matrix() {
    let m = Matrix::create(
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    );

    let expect = Matrix::create(
        [0., 9., 1., 0.],
        [9., 8., 8., 0.],
        [3., 0., 5., 5.],
        [0., 8., 3., 8.],
    );

    assert_eq!(expect, m.transpose());
}

#[test]
fn transpose_identity_matrix() {
    assert_eq!(IDENTITY_MATRIX, IDENTITY_MATRIX.transpose());
}
