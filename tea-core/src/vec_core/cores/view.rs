use super::super::{
    iter::{OptIter, ToIter},
    trusted::TrustIter,
    uninit::UninitRefMut,
};
use super::own::{Vec1, Vec1Collect};
use tea_dtype::{Cast, IsNone};

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
    fn rolling_custom<O: Vec1, U: ?Sized, F>(&self, window: usize, mut f: F) -> O
    where
        Self: std::ops::Index<std::ops::Range<usize>, Output = U>,
        F: FnMut(&U) -> O::Item,
    {
        (1..self.len() + 1)
            .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
            .map(|(end, start)| f(&self[start..end]))
            .collect_trusted_vec1()
    }

    #[inline]
    fn rolling2_custom<O: Vec1, V2, U1: ?Sized, U2: ?Sized, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
    ) -> O
    where
        Self: std::ops::Index<std::ops::Range<usize>, Output = U1>,
        V2: Vec1 + std::ops::Index<std::ops::Range<usize>, Output = U2>,
        F: FnMut(&U1, &U2) -> O::Item,
    {
        (1..self.len() + 1)
            .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
            .map(|(end, start)| f(&self[start..end], &other[start..end]))
            .collect_trusted_vec1()
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
                remove_value_iter
                    .zip(self.to_iter())
                    .map(move |(v_remove, v)| f(v_remove, v))
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
    fn rolling2_apply<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        Self::Item: Clone,
        V2::Item: Clone,
        F: FnMut(Option<(Self::Item, V2::Item)>, (Self::Item, V2::Item)) -> O::Item,
    {
        if let Some(out) = out {
            self.rolling2_apply_to::<O, _, _>(other, window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let remove_value_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain(self.to_iter().zip(other.to_iter()).map(Some));
            Some(
                remove_value_iter
                    .zip(self.to_iter().zip(other.to_iter()))
                    .map(move |(v_remove, v)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling_apply instead
    fn rolling2_apply_to<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        F: FnMut(Option<(Self::Item, V2::Item)>, (Self::Item, V2::Item)) -> O::Item,
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
                out.uset(i, f(None, (self.uget(i), other.uget(i))))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe {
                // new valid value
                let (v1_rm, v1) = (self.uget(start), self.uget(end));
                let (v2_rm, v2) = (other.uget(start), other.uget(end));
                out.uset(end, f(Some((v1_rm, v2_rm)), (v1, v2)))
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
