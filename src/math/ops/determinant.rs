use crate::matrix::matrix_fixed::*;
use crate::number::float::Float;

pub trait Determinant
{
    type Output;

    fn det(&self) -> Self::Output;
}

/*impl<T, const SIZE: usize> Determinant for MatrixFixed<T, SIZE, SIZE>
where
    [T; SIZE*SIZE]: Sized,
    T: Float
{
    type Output = T;

    fn det(&self) -> T
    {
    }
}*/