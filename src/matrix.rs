use core::ops::{Index, IndexMut};
use core::slice::Iter;

pub mod matrix_fixed;

pub trait Matrix<T>: Index<(usize, usize)> + IndexMut<(usize, usize)>
{
    fn col_count(&self) -> usize;
    fn row_count(&self) -> usize;

    fn get(&self, c: usize, r: usize) -> Option<&T>;
    fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T>;

    fn get_unchecked(&self, c: usize, r: usize) -> &T
    {
        self.get(c, r).unwrap()
    }

    fn get_mut_unchecked(&mut self, c: usize, r: usize) -> &mut T
    {
        self.get_mut(c, r).unwrap()
    }

    //fn submatrix(&self) -> dyn Matrix<T, Output = Self::Output>;

    fn iter(&self) -> Iter<T>;
}