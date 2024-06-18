use std::borrow::Cow;

use super::super::{
    iter::{OptIter, TIter},
    iter_traits::TIterator,
    trusted::TrustIter,
    uninit::UninitRefMut,
};
use super::own::{Vec1, Vec1Collect};
use tea_dtype::{Cast, IsNone};
use tea_error::{tbail, TResult};

pub trait Slice {
    type Element;
    // lifetime 'a is needed for ndarray backend, ArrayView has lifetime 'a
    type Output<'a>: Vec1View<Item = Self::Element> + ToOwned + ?Sized
    where
        Self: 'a,
        Self::Element: 'a;

    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Cow<'a, Self::Output<'a>>>
    where
        <Self::Output<'a> as TIter>::Item: 'a;
}

pub trait Vec1View: TIter + Slice<Element = Self::Item> {
    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> Self::Item;

    #[inline(always)]
    fn try_as_slice(&self) -> Option<&[Self::Item]> {
        None
    }

    #[inline]
    fn iter_cast<U>(&self) -> TrustIter<impl TIterator<Item = U>>
    where
        Self::Item: Cast<U>,
    {
        TrustIter::new(self.titer().map(|v| v.cast()), self.len())
    }

    #[inline]
    fn opt_iter_cast<U>(&self) -> TrustIter<impl TIterator<Item = Option<U>>>
    where
        Self::Item: IsNone,
        <Self::Item as IsNone>::Inner: Cast<U>,
    {
        TrustIter::new(
            self.titer().map(|v| v.to_opt().map(Cast::<U>::cast)),
            self.len(),
        )
    }

    #[inline]
    fn opt(&self) -> OptIter<Self>
    where
        Self::Item: IsNone,
        Self: Sized,
    {
        OptIter { view: self }
    }

    #[inline]
    fn to_opt_iter<'a>(
        &'a self,
    ) -> TrustIter<impl TIterator<Item = Option<<Self::Item as IsNone>::Inner>>>
    where
        Self::Item: IsNone + 'a,
    {
        TrustIter::new(self.titer().map(|v| v.to_opt()), self.len())
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
    fn get(&self, index: usize) -> TResult<Self::Item> {
        if index < self.len() {
            Ok(unsafe { self.uget(index) })
        } else {
            tbail!(io(index, self.len()))
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

    /// Rolling and apply a custom funtion to each window
    #[inline]
    fn rolling_custom<O: Vec1, F>(
        &self,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(&<Self as Slice>::Output<'_>) -> O::Item,
        O::Item: Clone,
    {
        if let Some(mut out) = out {
            let iter = (1..self.len() + 1)
                .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
                .map(|(end, start)| f(self.slice(start, end).unwrap().as_ref()));
            // TODO: maybe we should return a result here?
            out.write_trust_iter(iter).unwrap();
            None
        } else {
            let res = (1..self.len() + 1)
                .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
                .map(|(end, start)| f(self.slice(start, end).unwrap().as_ref()))
                .collect_trusted_vec1();
            Some(res)
        }
    }

    /// Rolling and apply a custom funtion to each window of two vecs
    #[inline]
    fn rolling2_custom<O: Vec1, V2, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        V2: Vec1View,
        F: FnMut(&<Self as Slice>::Output<'_>, &<V2 as Slice>::Output<'_>) -> O::Item,
        O::Item: Clone,
    {
        if let Some(mut out) = out {
            let iter = (1..self.len() + 1)
                .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
                .map(|(end, start)| {
                    f(
                        self.slice(start, end).unwrap().as_ref(),
                        other.slice(start, end).unwrap().as_ref(),
                    )
                });
            // TODO: maybe we should return a result here?
            out.write_trust_iter(iter).unwrap();
            None
        } else {
            let res = (1..self.len() + 1)
                .zip(std::iter::repeat(0).take(window - 1).chain(0..self.len()))
                .map(|(end, start)| {
                    f(
                        self.slice(start, end).unwrap().as_ref(),
                        other.slice(start, end).unwrap().as_ref(),
                    )
                })
                .collect_trusted_vec1();
            Some(res)
        }
    }

    /// Rolling and apply a function, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window
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
                .chain(self.titer().map(Some));
            Some(
                remove_value_iter
                    .zip(self.titer())
                    .map(move |(v_remove, v)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    /// Rolling and apply a function, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window.
    ///
    /// Different with `rolling_apply`, the caller should pass a mut reference
    /// of uninit vec.
    /// Be careful to use this function as it will panic in polars backend.
    /// use `rolling_apply` instead
    #[inline]
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

    /// Rolling and apply a function to both vecs, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window
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
                .chain(self.titer().zip(other.titer()).map(Some));
            Some(
                remove_value_iter
                    .zip(self.titer().zip(other.titer()))
                    .map(move |(v_remove, v)| f(v_remove, v))
                    .collect_trusted_vec1(),
            )
        }
    }

    #[inline]
    /// Rolling and apply a function to both vecs, the function accept whether to
    /// move element from the window and a value to be added to
    /// the window.
    ///
    /// Different with `rolling_apply`, the caller should pass a mut reference
    /// of uninit vec.
    /// Be careful to use this function as it will panic in polars backend.
    /// use `rolling_apply` instead
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
                self.titer()
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

    #[inline]
    fn rolling2_apply_idx<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, (Self::Item, V2::Item)) -> O::Item,
    {
        if let Some(out) = out {
            self.rolling2_apply_idx_to::<O, _, _>(other, window, f, out);
            None
        } else {
            assert!(window > 0, "window must be greater than 0");
            let start_iter = std::iter::repeat(None)
                .take(window - 1)
                .chain((0..self.len()).map(Some)); // this is longer than expect, but start_iter will stop earlier
            Some(
                self.titer()
                    .zip(other.titer())
                    .zip(start_iter)
                    .enumerate()
                    .map(move |(end, ((v, v2), start))| f(start, end, (v, v2)))
                    .collect_trusted_vec1(),
            )
        }
    }

    #[inline]
    /// be careful to use this function as it will panic in polars backend.
    /// use rolling2_apply_idx instead
    fn rolling2_apply_idx_to<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        mut f: F,
        mut out: O::UninitRefMut<'_>,
    ) where
        // start, end, value
        F: FnMut(Option<usize>, usize, (Self::Item, V2::Item)) -> O::Item,
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
                out.uset(i, f(None, i, (self.uget(i), other.uget(i))))
            }
        }
        // other windows
        for (start, end) in (window - 1..len).enumerate() {
            unsafe { out.uset(end, f(Some(start), end, (self.uget(end), other.uget(end)))) }
        }
    }
}

impl<I: TIter> TIter for std::sync::Arc<I> {
    type Item = I::Item;

    #[inline]
    fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item = I::Item>>
    where
        I::Item: 'a,
    {
        (**self).titer()
    }
}

impl<S: Slice> Slice for std::sync::Arc<S> {
    type Element = S::Element;
    type Output<'a> = S::Output<'a>
    where
        Self: 'a,
        Self::Element: 'a;

    #[inline]
    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Cow<'a, Self::Output<'a>>>
    where
        <Self::Output<'a> as TIter>::Item: 'a,
    {
        (**self).slice(start, end)
    }
}

impl<V: Vec1View> Vec1View for std::sync::Arc<V> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> Self::Item {
        (**self).uget(index)
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[Self::Item]> {
        (**self).try_as_slice()
    }

    #[inline]
    fn rolling_apply<O: Vec1, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        Self::Item: Clone,
        F: FnMut(Option<Self::Item>, Self::Item) -> O::Item,
    {
        (**self).rolling_apply(window, f, out)
    }

    #[inline]
    fn rolling2_apply<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        Self::Item: Clone,
        V2::Item: Clone,
        F: FnMut(Option<(Self::Item, V2::Item)>, (Self::Item, V2::Item)) -> O::Item,
    {
        (**self).rolling2_apply(other, window, f, out)
    }

    #[inline]
    fn rolling_apply_idx<O: Vec1, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, Self::Item) -> O::Item,
    {
        (**self).rolling_apply_idx(window, f, out)
    }

    #[inline]
    fn rolling2_apply_idx<O: Vec1, V2: Vec1View, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, (Self::Item, V2::Item)) -> O::Item,
    {
        (**self).rolling2_apply_idx(other, window, f, out)
    }
}
