use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::marker::PhantomData;

use super::{VecView1D, VecMut1D};

pub struct OwnIter<T, D: VecView1D<T>> {
    pub idx: usize,
    pub len: usize,
    pub data: D,
    pub _element_dtype: PhantomData<T>,
}

impl <T, D: VecView1D<T>> Iterator for OwnIter<T, D> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            // safety: data is owned by the iterator, so it is safe to read from it
            let item = unsafe { std::ptr::read(self.data.uget(self.idx) as *const T) };
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

pub struct ViewIter<'a, T, D: VecView1D<T> + ?Sized> {
    pub idx: usize,
    pub len: usize,
    pub data: &'a D,
    pub _element_dtype: PhantomData<T>,
}

impl <'a, T: 'a, D: VecView1D<T>> Iterator for ViewIter<'a, T, D> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let item = unsafe { self.data.uget(self.idx) };
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

pub struct MutIter<'a, T, D: VecMut1D<T> + ?Sized> {
    pub idx: usize,
    pub len: usize,
    pub data: &'a mut D,
    pub _element_dtype: PhantomData<T>,
}

impl <'a, T: 'a, D: VecMut1D<T>> Iterator for MutIter<'a, T, D> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let item = unsafe { &mut *(self.data.uget_mut(self.idx) as *mut T)};
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

impl <T, D: VecView1D<T>> ExactSizeIterator for OwnIter<T, D> {
    #[inline]
    fn len(&self) -> usize {
        self.len - self.idx
    }
}

impl<'a, T: 'a, D: VecView1D<T>> ExactSizeIterator for ViewIter<'a, T, D> {
    #[inline]
    fn len(&self) -> usize {
        self.len - self.idx
    }
}

impl<'a, T: 'a, D: VecMut1D<T>> ExactSizeIterator for MutIter<'a, T, D> {
    #[inline]
    fn len(&self) -> usize {
        self.len - self.idx
    }
}

impl<T, D: VecView1D<T>> FusedIterator for OwnIter<T, D> {}
impl<'a, T: 'a, D: VecView1D<T>> FusedIterator for ViewIter<'a, T, D> {}
impl<'a, T: 'a, D: VecMut1D<T>> FusedIterator for MutIter<'a, T, D> {}

impl <T, D: VecView1D<T>> DoubleEndedIterator for OwnIter<T, D> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { std::ptr::read(self.data.uget(self.len) as *const T) })
        } else {
            None
        }
    }
}

impl <'a, T: 'a, D: VecView1D<T>> DoubleEndedIterator for ViewIter<'a, T, D> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { self.data.uget(self.len) })
        } else {
            None
        }
    }
}

impl <'a, T: 'a, D: VecMut1D<T>> DoubleEndedIterator for MutIter<'a, T, D> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { &mut *(self.data.uget_mut(self.len) as *mut T) })
        } else {
            None
        }
    }
}