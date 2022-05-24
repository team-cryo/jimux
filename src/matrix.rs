use core::ops::{Index, IndexMut};
use core::slice::Iter;

pub mod matrix_fixed;

pub trait Matrix<T>: Index<(usize, usize), Output = T> + IndexMut<(usize, usize)> + Sized
{
    fn col_count(&self) -> usize;
    fn row_count(&self) -> usize;

    fn get(&self, r: usize, c: usize) -> Option<&T>;
    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T>;

    fn get_unchecked(&self, r: usize, c: usize) -> &T
    {
        self.get(r, c).unwrap()
    }

    fn get_mut_unchecked(&mut self, r: usize, c: usize) -> &mut T
    {
        self.get_mut(r, c).unwrap()
    }

    //fn submatrix(&self) -> dyn Matrix<T, Output = Self::Output>;

    fn iter(&self) -> Iter<T>;
}