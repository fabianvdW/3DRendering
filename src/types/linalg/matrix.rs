use crate::types::linalg::dimension::Dimension;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct Matrix<T> {
    pub dimension: Dimension,
    pub data: Vec<T>,
}
impl<T: Copy> Matrix<T> {
    pub fn from_data(data: Vec<T>, dimension: Dimension) -> Self {
        debug_assert!(!data.is_empty());
        Matrix { dimension, data }
    }
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }
    pub fn apply_closure<F>(&mut self, closure: F)
    where
        F: Fn(T, usize) -> T,
    {
        for index in self.dimension.iter() {
            self.data[index] = closure(self.data[index], index);
        }
    }
    pub fn from_closure<F>(closure: F, dimension: Dimension) -> Matrix<T>
    where
        F: Fn(usize) -> T,
    {
        let mut data = Vec::with_capacity(dimension.rows * dimension.columns);
        for index in dimension.iter() {
            data.push(closure(index));
        }
        Matrix { dimension, data }
    }
    pub fn closure_into_buffer<F>(&mut self, closure: F)
    where
        F: Fn(usize) -> T,
    {
        for index in self.dimension.iter() {
            self.data[index] = closure(index);
        }
    }
}
//Matrix Matrix Addition and Subtraction
impl<T: Add<T, Output = T> + Copy> Add for Matrix<T> {
    type Output = Matrix<T>;
    fn add(mut self, other: Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension == other.dimension);
        self.apply_closure(|t, index| t + other.data[index]);
        self
    }
}
impl<'a, 'b, T: Add<T, Output = T> + Copy> Add<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, other: &'b Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension == other.dimension);
        Matrix::from_closure(|index| self.data[index] + other.data[index], self.dimension)
    }
}
impl<T: Sub<T, Output = T> + Copy> Sub for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(mut self, other: Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension == other.dimension);
        self.apply_closure(|t, index| t - other.data[index]);
        self
    }
}
impl<'a, 'b, T: Sub<T, Output = T> + Copy> Sub<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, other: &'b Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension == other.dimension);
        Matrix::from_closure(|index| self.data[index] - other.data[index], self.dimension)
    }
}
//Matrix Scalar Addition and Subtraction and Multiplication
impl<T: Add<T, Output = T> + Copy> Add<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(mut self, other: T) -> Self::Output {
        self.apply_closure(|t, _| t + other);
        self
    }
}
impl<'a, T: Add<T, Output = T> + Copy> Add<T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, other: T) -> Self::Output {
        Matrix::from_closure(|index| self.data[index] + other, self.dimension)
    }
}
impl<T: Sub<T, Output = T> + Copy> Sub<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(mut self, other: T) -> Self::Output {
        self.apply_closure(|t, _| t - other);
        self
    }
}
impl<'a, T: Sub<T, Output = T> + Copy> Sub<T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, other: T) -> Self::Output {
        Matrix::from_closure(|index| self.data[index] - other, self.dimension)
    }
}
impl<T: Mul<T, Output = T> + Copy> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(mut self, other: T) -> Self::Output {
        self.apply_closure(|t, _| t * other);
        self
    }
}
impl<'a, T: Mul<T, Output = T> + Copy> Mul<T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: T) -> Self::Output {
        Matrix::from_closure(|index| self.data[index] * other, self.dimension)
    }
}
//Matrix Matrix Multiplication
pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
}
impl Zero for i32 {
    #[inline(always)]
    fn zero() -> Self {
        0
    }
}
impl Zero for f32 {
    #[inline(always)]
    fn zero() -> Self {
        0.
    }
}
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension.columns == other.dimension.rows);
        let output_dimension = Dimension::new(self.dimension.rows, other.dimension.columns);
        Matrix::from_closure(
            |index| {
                let (row, colm) = output_dimension.to_xy(index);
                (0..self.dimension.columns).fold(Zero::zero(), |curr, i| {
                    curr + self.data[self.dimension.to_index(row, i)]
                        * other.data[other.dimension.to_index(i, colm)]
                })
            },
            output_dimension,
        )
    }
}
impl<'a, 'b, T: Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy> Mul<&'a Matrix<T>>
    for &'b Matrix<T>
{
    type Output = Matrix<T>;
    fn mul(self, other: &'a Matrix<T>) -> Self::Output {
        debug_assert!(self.dimension.columns == other.dimension.rows);
        let output_dimension = Dimension::new(self.dimension.rows, other.dimension.columns);
        Matrix::from_closure(
            |index| {
                let (row, colm) = output_dimension.to_xy(index);
                (0..self.dimension.columns).fold(Zero::zero(), |curr, i| {
                    curr + self.data[self.dimension.to_index(row, i)]
                        * other.data[other.dimension.to_index(i, colm)]
                })
            },
            output_dimension,
        )
    }
}
//Matrix Matrix Buffered Multiplication
impl<T: AddAssign + Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy> Matrix<T> {
    pub fn buffered_mul(&mut self, m1: &Matrix<T>, m2: &Matrix<T>) {
        assert_eq!(m1.dimension.columns, m2.dimension.rows);
        assert!(
            self.dimension.rows == m1.dimension.rows
                && self.dimension.columns == m2.dimension.columns
        );
        assert_eq!(
            self.data.len(),
            self.dimension.columns * self.dimension.rows
        );
        assert_eq!(m1.data.len(), m1.dimension.columns * m1.dimension.rows);
        assert_eq!(m2.data.len(), m2.dimension.columns * m2.dimension.rows);
        let output_dimension = self.dimension;
        self.closure_into_buffer(|index| {
            let (row, colm) = output_dimension.to_xy(index);
            (0..m1.dimension.columns).fold(Zero::zero(), |curr, i| {
                curr + m1.data[m1.dimension.to_index(row, i)]
                    * m2.data[m2.dimension.to_index(i, colm)]
            })
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_matmul() {
        let matrix1 = Matrix::from_data(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Dimension::new(3, 3));
        let res = &matrix1 * &matrix1;
        debug_assert!(vec![30, 36, 42, 66, 81, 96, 102, 126, 150] == res.data);
    }
}
