mod iter;
mod iter_traits;
mod trusted;
mod uninit;

pub use uninit::{UninitRefMut, UninitVec};

use crate::prelude::IsNone;
pub use iter::{IntoIter, OptIter, ToIter};
pub use iter_traits::IterBasic;
use tea_dtype::Cast;
pub use trusted::{CollectTrustedToVec, ToTrustIter, TrustIter, TrustedLen};

pub trait Vec1View: ToIter {
    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> Self::Item;

    #[inline]
    fn to_iter<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self::Item: 'a,
    {
        self.to_iterator()
    }

    #[inline]
    fn iter_cast<U>(&self) -> TrustIter<impl Iterator<Item = U>>
    where
        Self::Item: Cast<U>,
    {
        TrustIter::new(self.to_iterator().map(|v| v.cast()), self.len())
    }

    #[inline]
    fn opt_iter_cast<U>(&self) -> TrustIter<impl Iterator<Item = Option<U>>>
    where
        Self::Item: IsNone,
        <Self::Item as IsNone>::Inner: Cast<U>,
    {
        TrustIter::new(
            self.to_iterator().map(|v| v.to_opt().map(Cast::<U>::cast)),
            self.len(),
        )
    }

    #[inline]
    fn to_opt(&self) -> OptIter<Self>
    where
        Self::Item: IsNone,
        Self: Sized,
    {
        OptIter { view: self }
    }

    #[inline]
    fn to_opt_iter<'a>(
        &'a self,
    ) -> TrustIter<impl Iterator<Item = Option<<Self::Item as IsNone>::Inner>>>
    where
        Self::Item: IsNone + 'a,
    {
        TrustIter::new(self.to_iterator().map(|v| v.to_opt()), self.len())
    }

    /// if the value is valid, return it, otherwise return None
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<<Self::Item as IsNone>::Inner>
    where
        Self::Item: IsNone,
    {
        let v = self.uget(index);
        if v.is_none() {
            None
        } else {
            v.to_opt()
        }
    }

    #[inline]
    fn get(&self, index: usize) -> Self::Item {
        if index < self.len() {
            unsafe { self.uget(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn vget(&self, index: usize) -> Option<<Self::Item as IsNone>::Inner>
    where
        Self::Item: IsNone,
    {
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    #[inline]
    fn rolling_apply<O: Vec1, F>(
        &self,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        Self::Item: Clone,
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        if let Some(out) = out {
            self.rolling_apply_to::<O, _>(window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let remove_value_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain(self.to_iterator().map(Some));
            Some(
                self.to_iter()
                    .zip(remove_value_iter)
                    .map(move |(v, v_remove)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling_apply instead
    fn rolling_apply_to<O: Vec1, F>(&self, window: usize, mut f: F, mut out: O::UninitRefMut<'_>)
    where
        Self::Item: Clone,
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, self.uget(i)))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe {
                // new valid value
                let (v_rm, v) = (self.uget(start), self.uget(end));
                out.uset(end, f(Some(v_rm), v))
            }
        }
    }

    #[inline]
    fn rolling_apply_idx<O: Vec1, F>(
        &self,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        if let Some(out) = out {
            self.rolling_apply_idx_to::<O, _>(window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let start_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain((0..self.len()).map(Some)); // this is longer than expect, but start_iter will stop earlier
            Some(
                self.to_iter()
                    .zip(start_iter)
                    .enumerate()
                    .map(move |(end, (v, start))| f(start, end, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling_apply_idx instead
    fn rolling_apply_idx_to<O: Vec1, F>(
        &self,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        // start, end, value
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        let len = self.len();
        let window = window.min(len);
        if window == 0 {
            return;
        }
        // within the first window
        for i in 0..window - 1 {
            unsafe {
                // no value should be removed in the first window
                out.uset(i, f(None, i, self.uget(i)))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe { out.uset(end, f(Some(start), end, self.uget(end))) }
        }
    }
}

pub trait Vec1Mut<'a>: Vec1View {
    // type OutType;
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut Self::Item;

    #[inline]
    fn get_mut(&'a mut self, index: usize) -> Option<&'a mut Self::Item> {
        if index < self.len() {
            Some(unsafe { self.uget_mut(index) })
        } else {
            None
        }
    }
}

/// a vector owns its data is not necessarily mutable
pub trait Vec1: Vec1View + Sized {
    type Uninit: UninitVec<Self::Item, Vec = Self>;
    type UninitRefMut<'a>: UninitRefMut<Self::Item>
    where
        Self::Item: 'a;

    fn collect_from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> Self;

    fn uninit(len: usize) -> Self::Uninit;

    fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_>;

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = Self::Item> + TrustedLen>(iter: I) -> Self {
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn collect_with_len<I: Iterator<Item = Self::Item>>(iter: I, len: usize) -> Self {
        Self::collect_from_trusted(iter.to_trust(len))
    }

    #[inline]
    fn collect_from_opt_iter<I: Iterator<Item = Option<Self::Item>>>(iter: I) -> Self
    where
        Self::Item: IsNone,
    {
        let iter = iter.map(|v| v.unwrap_or(Self::Item::none()));
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn empty() -> Self {
        Self::collect_from_iter(std::iter::empty())
    }
}

pub trait Vec1Collect: IntoIterator {
    #[inline]
    fn collect_vec1<O: Vec1<Item = Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        O::collect_from_iter(self.into_iter())
    }

    #[inline]
    fn collect_trusted_vec1<O: Vec1<Item = Self::Item>>(self) -> O
    where
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1>::collect_from_trusted(self.into_iter())
    }

    #[inline]
    fn collect_vec1_with_len<O: Vec1<Item = Self::Item>>(self, len: usize) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_with_len(self.into_iter(), len)
    }
}

pub trait Vec1DOptCollect<T: IsNone>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1_opt<O: Vec1<Item = T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_from_opt_iter(self.into_iter())
    }
}

impl<T: IntoIterator + Sized> Vec1Collect for T {}
impl<I: IntoIterator<Item = Option<T>>, T: IsNone> Vec1DOptCollect<T> for I {}
