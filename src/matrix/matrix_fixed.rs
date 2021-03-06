use core::slice::Iter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixFixed<T, const ROWS: usize, const COLUMNS: usize>
where
    [T; ROWS*COLUMNS]: Sized
{
    data: [T; ROWS*COLUMNS]
}

impl<T, const ROWS: usize, const COLUMNS: usize> MatrixFixed<T, ROWS, COLUMNS>
where
    [T; ROWS*COLUMNS]: Sized
{
    pub fn new() -> Self
    where T: Copy + Default
    {
        Self::repeat(Default::default())
    }

    pub const fn repeat(repeat: T) -> Self
    where T: Copy
    {
        Self
        {
            data: [repeat; ROWS*COLUMNS]
        }
    }

    pub fn fill(&mut self, fill: T) -> ()
    where T: Clone
    {
        self.data.fill(fill)
    }

    pub fn rows(&self) -> &[[T; COLUMNS]; ROWS]
    {
        unsafe
        {
            self.data.as_chunks_unchecked::<COLUMNS>().split_array_ref::<ROWS>().0
        }
    }
    
    pub fn rows_mut(&mut self) -> &mut [[T; COLUMNS]; ROWS]
    {
        unsafe
        {
            self.data.as_chunks_unchecked_mut::<COLUMNS>().split_array_mut::<ROWS>().0
        }
    }

    pub const fn serial(&self) -> &[T; ROWS*COLUMNS]
    {
        &self.data
    }

    pub fn serial_mut(&mut self) -> &mut [T; ROWS*COLUMNS]
    {
        &mut self.data
    }

    pub fn as_op(&self) -> MatrixFixed<Option<T>, ROWS, COLUMNS>
    where T: Copy
    {
        let mut data: [Option<T>; ROWS*COLUMNS] = [None; ROWS*COLUMNS];

        for (i, x) in self.data.iter().map(|x| Some(*x)).enumerate()
        {
            data[i] = x;
        }
        
        MatrixFixed
        {
            data
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T> for MatrixFixed<T, ROWS, COLUMNS>
where
    [T; ROWS*COLUMNS]: Sized
{
    fn row_count(&self) -> usize
    {
        ROWS
    }
    fn col_count(&self) -> usize
    {
        COLUMNS
    }

    fn get(&self, r: usize, c: usize) -> Option<&T>
    {
        self.data.get(r*COLUMNS + c)
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T>
    {
        self.data.get_mut(r*COLUMNS + c)
    }

    fn iter(&self) -> Iter<T>
    {
        self.data.iter()
    }

    /*fn submatrix<const R: usize, const C: usize>(&self, r0: usize, c0: usize) -> MatrixFixed<T, R, C>
    where
        [T; R*C]: Sized
    {
        assert!(R + r0 <= ROWS, "Sub-matrix rows out of bounds");
        assert!(C + c0 <= COLUMNS, "Sub-matrix columns out of bounds");

        self.rows()[]
    }*/
}

impl<T, const ROWS: usize, const COLUMNS: usize> Index<(usize, usize)> for MatrixFixed<T, ROWS, COLUMNS>
where
    [T; ROWS*COLUMNS]: Sized
{
    type Output = T;

    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let (r, c) = indices;
        self.get_unchecked(r, c)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> IndexMut<(usize, usize)> for MatrixFixed<T, ROWS, COLUMNS>
where
    [T; ROWS*COLUMNS]: Sized
{
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        let (r, c) = indices;
        self.get_mut_unchecked(r, c)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Default for MatrixFixed<T, ROWS, COLUMNS>
where
    [T; ROWS*COLUMNS]: Sized,
    T: Copy + Default
{
    fn default() -> Self
    {
        Self::repeat(Default::default())
    }
}
