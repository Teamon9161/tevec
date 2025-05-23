use crate::prelude::*;

impl<'a, I: TIter<'a, T>, T> TIter<'a, T> for std::sync::Arc<I> {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = T> + 'a {
        (**self).titer()
    }
}

impl<'a, V: Vec1View<'a, T>, T> Vec1View<'a, T> for std::sync::Arc<V> {
    type SliceOutput<'b>
        = V::SliceOutput<'b>
    where
        Self: 'b,
        T: 'b;

    #[inline]
    fn slice(&self, start: usize, end: usize) -> TResult<Self::SliceOutput<'_>> {
        (**self).slice(start, end)
    }

    #[inline]
    unsafe fn uslice(&self, start: usize, end: usize) -> TResult<Self::SliceOutput<'_>> {
        (**self).uslice(start, end)
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        (**self).get_backend_name()
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> T {
        unsafe { (**self).uget(index) }
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[T]> {
        (**self).try_as_slice()
    }

    #[inline]
    fn rolling_custom<O: Vec1<OT>, OT: Clone, F>(
        &self,
        window: usize,
        f: F,
        out: Option<O::UninitRefMut<'_>>,
    ) -> Option<O>
    where
        F: FnMut(V::SliceOutput<'_>) -> OT,
    {
        (**self).rolling_custom(window, f, out)
    }

    #[inline]
    fn rolling_apply<O: Vec1<OT>, OT, F>(
        &'a self,
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
    fn rolling2_apply<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
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
        &'a self,
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
    fn rolling2_apply_idx<'b, O: Vec1<OT>, OT, V2: Vec1View<'b, T2>, T2, F>(
        &'a self,
        other: &'b V2,
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
