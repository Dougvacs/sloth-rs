use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};
use nalgebra as na;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(na::SVector<f32, N>);
impl<const N: usize> Vector<N> {
    pub fn new(data: &[f32]) -> Self {
        Self(na::SVector::from_column_slice(data))
    }
}

macro_rules! impl_op_variants {
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

impl_op_variants!(Add, add, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0 + b.0));
impl_op_variants!(Sub, sub, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0 - b.0));
impl_op_variants!(Div, div, Vector<N>, f32, Vector<N>, |a: Vector<N>, b: f32| Vector(a.0 / b));
impl_op_variants!(Mul, mul, Vector<N>, f32, Vector<N>, |a: Vector<N>, b: f32| Vector(a.0 * b));
impl_op_variants!(BitOr, bitor, Vector<N>, Vector<N>, f32, |a: Vector<N>, b: Vector<N>| a.0.dot(&b.0));
impl_op_variants!(BitAnd, bitand, Vector<N>, Vector<N>, Vector<N>, |a: Vector<N>, b: Vector<N>| Vector(a.0.cross(&b.0)));
