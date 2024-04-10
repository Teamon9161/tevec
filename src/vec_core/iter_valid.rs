use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::marker::PhantomData;
use crate::prelude::IsNone;

use super::VecView1D;

pub struct ViewIter<'a, T, D: VecView1D<T> + ?Sized> {
    pub idx: usize,
    pub len: usize,
    pub data: &'a D,
    pub _element_dtype: PhantomData<T>,
}

impl <'a, T: IsNone + 'a, D: VecView1D<T>> Iterator for ViewIter<'a, T, D> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let item = unsafe { self.data.uvget(self.idx) };
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }


    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remain = self.len - self.idx;
        (remain, Some(remain))
    }

    #[inline]
    fn count(self) -> usize {
        self.len - self.idx
    }
}

impl<'a, T: IsNone + 'a, D: VecView1D<T>> ExactSizeIterator for ViewIter<'a, T, D> {
    fn len(&self) -> usize {
        self.len - self.idx
    }

}

impl<'a, T: IsNone + 'a, D: VecView1D<T>> FusedIterator for ViewIter<'a, T, D> {}

impl <'a, T: IsNone + 'a, D: VecView1D<T>> DoubleEndedIterator for ViewIter<'a, T, D> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { self.data.uvget(self.len) })
        } else {
            None
        }
    }
}
