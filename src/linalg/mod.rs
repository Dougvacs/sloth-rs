use std::ops::{Add, Div, Mul, Sub, BitAnd, BitOr};
use nalgebra as na;

macro_rules! impl_op_gen_1 {
    ($trait:ident, $method:ident, $type:ty, $rhs:ty, $output:ty, $body:expr) => {
        impl<const N: usize> $trait<$rhs> for &$type {
            type Output = $output;
            fn $method(self, rhs: $rhs) -> Self::Output {
                $body(*self, rhs)
            }
        }
        impl<const N: usize> $trait<&$rhs> for &$type {
            type Output = $output;
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $body(*self, *rhs)
            }
        }
        impl<const N: usize> $trait<$rhs> for $type {
            type Output = $output;
            fn $method(self, rhs: $rhs) -> Self::Output {
                $body(self, rhs)
            }
        }
        impl<const N: usize> $trait<&$rhs> for $type {
            type Output = $output;
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $body(self, *rhs)
            }
        }
    };
}
macro_rules! impl_op_gen_2 {
    ($trait:ident, $method:ident, $type:ty, $rhs:ty, $output:ty, $body:expr) => {
        impl<const N: usize, const M: usize> $trait<$rhs> for &$type {
            type Output = $output;
            fn $method(self, rhs: $rhs) -> Self::Output {
                $body(*self, rhs)
            }
        }
        impl<const N: usize, const M: usize> $trait<&$rhs> for &$type {
            type Output = $output;
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $body(*self, *rhs)
            }
        }
        impl<const N: usize, const M: usize> $trait<$rhs> for $type {
            type Output = $output;
            fn $method(self, rhs: $rhs) -> Self::Output {
                $body(self, rhs)
            }
        }
        impl<const N: usize, const M: usize> $trait<&$rhs> for $type {
            type Output = $output;
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $body(self, *rhs)
            }
        }
    };
}

// Vector
#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(na::SVector<f32, N>);
impl<const N: usize> Vector<N> {
    pub fn new(data: &[f32]) -> Self {
        Self(na::SVector::from_column_slice(data))
    }
    pub fn trans(&self) -> Matrix<1, N> {
        Matrix(self.0.transpose())
    }
}

impl_op_gen_1!(Add, add, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0 + b.0));
impl_op_gen_1!(Sub, sub, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0 - b.0));
impl_op_gen_1!(Div, div, Vector<N>, f32, Vector<N>, |a: Vector<N>, b: f32| Vector(a.0 / b));
impl_op_gen_1!(Mul, mul, Vector<N>, f32, Vector<N>, |a: Vector<N>, b: f32| Vector(a.0 * b));
impl_op_gen_1!(BitOr, bitor, Vector<N>, Vector<N>, f32, |a: Vector<N>, b: Vector<N>| a.0.dot(&b.0));
impl_op_gen_1!(BitAnd, bitand, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0.cross(&b.0)));

// Matrix
#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize>(na::SMatrix<f32, N, M>);
impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn new(data: &[f32]) -> Self {
        Self(na::SMatrix::<f32, N, M>::from_column_slice(data))
    }
    pub fn trans(&self) -> Matrix<M, N> {
        Matrix(self.0.transpose())
    }
}

impl_op_gen_2!(Add, add, Matrix<N, M>, Matrix<N, M>, Matrix<N, M>, |a: Matrix<N, M>, b: Matrix<N, M>| Matrix(a.0 + b.0));
impl_op_gen_2!(Sub, sub, Matrix<N, M>, Matrix<N, M>, Matrix<N, M>, |a: Matrix<N, M>, b: Matrix<N, M>| Matrix(a.0 - b.0));
impl_op_gen_2!(Div, div, Matrix<N, M>, f32, Matrix<N, M>, |a: Matrix<N, M>, b: f32| Matrix(a.0 / b));
impl_op_gen_2!(Mul, mul, Matrix<N, M>, f32, Matrix<N, M>, |a: Matrix<N, M>, b: f32| Matrix(a.0 * b));
impl_op_gen_2!(Mul, mul, Matrix<N, M>, Vector<M>, Vector<N>, |a: Matrix<N, M>, b: Vector<M>| Vector(a.0 * b.0));
impl_op_gen_2!(Mul, mul, Matrix<N, M>, Matrix<M, N>, Matrix<N, N>, |a: Matrix<N, M>, b: Matrix<M, N>| Matrix(a.0 * b.0));
