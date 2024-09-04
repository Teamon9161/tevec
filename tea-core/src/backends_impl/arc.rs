use crate::prelude::*;

impl<I: TIter<T>, T> TIter<T> for std::sync::Arc<I> {
    #[inline]
    fn titer<'a>(&'a self) -> impl TIterator<Item = T>
    where
        T: 'a,
    {
        (**self).titer()
    }
}

impl<V: Vec1View<T>, T> Vec1View<T> for std::sync::Arc<V> {
    type SliceOutput<'a> = V::SliceOutput<'a>
    where
        Self: 'a,
        T: 'a;

    #[inline]
    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        (**self).slice(start, end)
    }

    #[inline]
    unsafe fn uslice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
    where
        T: 'a,
    {
        (**self).uslice(start, end)
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        (**self).get_backend_name()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        (**self).uget(index)
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[T]> {
        (**self).try_as_slice()
    }

    #[inline]
    fn rolling_custom<'a, O: Vec1<OT>, OT: Clone, F>(
        &'a self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(Self::SliceOutput<'a>) -> OT,
        T: 'a,
    {
        (**self).rolling_custom(window, f, out)
    }

    #[inline]
    fn rolling_apply<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        F: FnMut(Option<T>, T) -> OT,
    {
        (**self).rolling_apply(window, f, out)
    }

    #[inline]
    fn rolling2_apply<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        T: Clone,
        T2: Clone,
        F: FnMut(Option<(T, T2)>, (T, T2)) -> OT,
    {
        (**self).rolling2_apply(other, window, f, out)
    }
    #[inline]
    fn rolling_apply_idx<O: Vec1<OT>, OT, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, T) -> OT,
    {
        (**self).rolling_apply_idx(window, f, out)
    }

    #[inline]
    fn rolling2_apply_idx<O: Vec1<OT>, OT, V2: Vec1View<T2>, T2, F>(
        &self,
        other: &V2,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        // start, end, value
        F: FnMut(Option<usize>, usize, (T, T2)) -> OT,
    {
        (**self).rolling2_apply_idx(other, window, f, out)
    }
}
