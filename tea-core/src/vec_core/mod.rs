mod iter;
mod iter_valid;
pub mod trusted;

use crate::prelude::IsNone;
use iter::{MutIter, OwnIter, ViewIter};
use std::iter::Iterator;
use std::marker::PhantomData;

pub use trusted::{CollectTrustedToVec, TrustedLen};

pub trait VecView1D<T> {
    fn len(&self) -> usize;

    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> &T;

    /// if the value is valid, return it, otherwise return None
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<&T>
    where
        T: IsNone,
    {
        let v = self.uget(index);
        if v.is_none() {
            None
        } else {
            Some(v)
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn get(&self, index: usize) -> &T {
        if index < self.len() {
            unsafe { self.uget(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn vget(&self, index: usize) -> Option<&T>
    where
        T: IsNone,
    {
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    #[inline]
    fn fold<U, F>(&self, init: U, mut f: F) -> U
    where
        F: FnMut(U, &T) -> U,
    {
        let mut acc = init;
        for i in 0..self.len() {
            let v = unsafe { self.uget(i) };
            acc = f(acc, v);
        }
        acc
    }

    #[inline]
    fn vfold<U, F>(&self, init: U, mut f: F) -> U
    where
        F: FnMut(U, &T) -> U,
        T: IsNone,
    {
        let mut acc = init;
        for i in 0..self.len() {
            let v = unsafe { self.uvget(i) };
            if let Some(v) = v {
                acc = f(acc, v);
            }
        }
        acc
    }

    #[inline]
    fn vfold_n<U, F>(&self, init: U, mut f: F) -> (usize, U)
    where
        F: FnMut(U, &T) -> U,
        T: IsNone,
    {
        let mut acc = init;
        let mut count = 0;
        for i in 0..self.len() {
            let v = unsafe { self.uvget(i) };
            if let Some(v) = v {
                acc = f(acc, v);
                count += 1;
            }
        }
        (count, acc)
    }

    #[inline]
    fn map<U, F, O: Vec1D<U>>(&self, mut f: F) -> O
    where
        F: FnMut(&T) -> U,
    {
        let iter = (0..self.len()).map(|i| {
            let v = unsafe { self.uget(i) };
            f(v)
        });
        Vec1D::collect_from_trusted(iter)
    }

    #[inline]
    fn iter_view(&self) -> ViewIter<T, Self> {
        ViewIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }

    fn rolling_apply<U, F, O>(&self, window: usize, mut f: F) -> O
    where
        F: FnMut(Option<&T>, &T) -> U,
        O: Vec1D<U>,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return O::empty();
        }
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len - window + 1).map(Some));
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v_remove = start.map(|v| unsafe { self.uget(v) });
                let v = unsafe { self.uget(end) };
                f(v_remove, v)
            })
            .collect_trusted()
    }

    // fn rolling_apply_opt<U, F, O>(&self, window: usize, mut f: F) -> O
    // where
    //     U: IsNone,
    //     F: FnMut(Option<&T>, &T) -> Option<U>,
    //     O: Vec1D<U>,
    // {
    //     let len = self.len();
    //     let window = window.min(len);
    //     if window == 0 {
    //         return O::empty();
    //     }
    //     let start_iter = std::iter::repeat(None).take(window-1)
    //         .chain((0..len - window + 1).map(Some));
    //     start_iter.zip(0..len).map(|(start, end)| {
    //         let v_remove = start.map(|v| unsafe{ self.uget(v) });
    //         let v = unsafe { self.uget(end) };
    //         f(v_remove, v)
    //     }).collect_vec1d_opt()
    // }

    /// Apply a rolling function to the array
    fn rolling_vapply_opt<U, F, O>(&self, window: usize, mut f: F) -> O
    where
        T: IsNone,
        U: IsNone,
        F: FnMut(Option<Option<&T>>, Option<&T>) -> Option<U>,
        O: Vec1D<U>,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return O::empty();
        }
        let start_iter = std::iter::repeat(None)
            .take(window - 1)
            .chain((0..len - window + 1).map(Some));
        start_iter
            .zip(0..len)
            .map(|(start, end)| {
                let v_remove = start.map(|v| unsafe { self.uvget(v) });
                let v = unsafe { self.uvget(end) };
                f(v_remove, v)
            })
            .collect_vec1d_opt()
    }
}

pub trait VecMut1D<T>: VecView1D<T> {
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&mut self, index: usize) -> &mut T;

    #[inline]
    fn get_mut(&mut self, index: usize) -> &mut T {
        if index < self.len() {
            unsafe { self.uget_mut(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn iter_mut(&mut self) -> MutIter<T, Self>
where {
        MutIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }

    fn map_inplace(&mut self, mut f: impl FnMut(&mut T)) {
        for i in 0..self.len() {
            let v = unsafe { self.uget_mut(i) };
            f(v);
        }
    }
}

pub trait Vec1D<T>: VecMut1D<T> {
    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self;

    #[inline]
    fn collect_from_iter_opt<I: Iterator<Item = Option<T>>>(iter: I) -> Self
    where
        T: IsNone,
        Self: Sized,
    {
        let iter = iter.map(|v| v.unwrap_or(T::none()));
        Vec1D::collect_from_iter(iter)
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = T> + TrustedLen>(iter: I) -> Self
    where
        Self: Sized,
    {
        Self::collect_from_iter(iter)
    }

    fn iter_own(self) -> OwnIter<T, Self>
    where
        Self: Sized,
    {
        OwnIter {
            idx: 0,
            len: self.len(),
            data: self,
            _element_dtype: PhantomData,
        }
    }

    #[inline]
    fn empty() -> Self
    where
        Self: Sized,
    {
        Self::collect_from_iter(std::iter::empty())
    }
}

pub trait Vec1DCollect: Iterator {
    #[inline]
    fn collect_vec1d<O: Vec1D<Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1D<Self::Item>>::collect_from_iter(self)
    }

    #[inline]
    fn collect_trusted<O: Vec1D<Self::Item>>(self) -> O
    where
        Self: Sized + TrustedLen,
    {
        <O as Vec1D<Self::Item>>::collect_from_trusted(self)
    }
}

pub trait Vec1DOptCollect<T: IsNone>: Iterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1d_opt<O: Vec1D<T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1D<T>>::collect_from_iter_opt(self)
    }
}

impl<T: Iterator + Sized> Vec1DCollect for T {}
impl<I: Iterator<Item = Option<T>>, T: IsNone> Vec1DOptCollect<T> for I {}
