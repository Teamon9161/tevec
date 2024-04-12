use polars::prelude::*;
use crate::prelude::*;


impl<P, T: IsNone+Clone> VecView1D<T> for &ChunkedArray<P>
where
    P: PolarsNumericType<Native = T>,
{
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> &T {
        &self.get_unchecked(index).unwrap()
    }

    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<&T>
    where
        T: IsNone,
    {
        self.get_unchecked(index).as_ref()
    }
}

// impl<P, T: IsNone+Clone> VecMut1D<T> for &mut ChunkedArray<P>
// where
//     P: PolarsNumericType<Native = T>,
// {
//     #[inline]
//     unsafe fn uget_mut(&mut self, index: usize) -> &mut T {
//         // self.get_unchecked_mut()
//         unimplemented!()
//     }
// }