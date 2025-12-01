use tea_deps::polars::prelude::*;
use tea_deps::polars_arrow::legacy::utils::CustomIterTools;

use crate::prelude::*;

impl<'a> TIter<Option<&'a str>> for &'a ChunkedArray<StringType> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = Option<&'a str>> {
        self.into_iter()
    }
    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = Option<&'a str>> {
        self.into_iter()
    }
}

impl<'a> TIter<Option<&'a str>> for ChunkedArray<StringType> {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = Option<&'a str>> {
        unsafe {
            std::mem::transmute::<_, Box<dyn PolarsIterator<Item = Option<&'a str>> + 'a>>(
                self.into_iter(),
            )
        }
    }
    #[inline]
    fn tditer(&self) -> impl TDoubleIterator<Item = Option<&'a str>> {
        unsafe {
            std::mem::transmute::<_, Box<dyn PolarsIterator<Item = Option<&'a str>> + 'a>>(
                self.into_iter(),
            )
        }
    }
}

impl<'a> Vec1View<Option<&'a str>> for &'a ChunkedArray<StringType> {
    type SliceOutput<'b>
        = ChunkedArray<StringType>
    where
        Self: 'b,
        Option<&'a str>: 'b;

    #[inline]
    fn slice<'b>(&'b self, start: usize, end: usize) -> TResult<Self::SliceOutput<'b>>
    where
        Self: 'b,
        Option<&'a str>: 'b,
    {
        if end < start {
            tbail!(
                "end index: {} should be large than start index: {} in slice",
                end,
                start
            );
        }
        let len = end - start;
        Ok((*self).slice(start as i64, len))
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        "polars"
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<&'a str> {
        unsafe { self.get_unchecked(index) }
    }
}

impl<'a> Vec1View<Option<&'a str>> for ChunkedArray<StringType> {
    type SliceOutput<'b>
        = ChunkedArray<StringType>
    where
        Self: 'b,
        Option<&'a str>: 'b;

    #[inline]
    fn slice<'b>(&'b self, start: usize, end: usize) -> TResult<Self::SliceOutput<'b>>
    where
        Self: 'b,
        Option<&'a str>: 'b,
    {
        if end < start {
            tbail!(
                "end index: {} should be large than start index: {} in slice",
                end,
                start
            );
        }
        let len = end - start;
        Ok((*self).slice(start as i64, len))
    }

    #[inline]
    fn get_backend_name(&self) -> &'static str {
        "polars"
    }

    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<&'a str> {
        unsafe { std::mem::transmute(self.get_unchecked(index)) }
    }
}

impl<'a, 'b> Vec1Mut<'a, Option<&'b str>> for ChunkedArray<StringType> {
    #[inline]
    unsafe fn uget_mut(&mut self, _index: usize) -> &mut Option<&'b str> {
        unimplemented!("get mut is not supported in polars backend");
    }
}

impl<'a> Vec1<Option<&'a str>> for ChunkedArray<StringType> {
    type Uninit = ChunkedArray<StringType>;
    type UninitRefMut<'b>
        = &'b mut ChunkedArray<StringType>
    where
        Option<&'a str>: 'b;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = Option<&'a str>>>(iter: I) -> Self {
        iter.collect()
    }

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<Option<&'a str>>>>(
        iter: I,
    ) -> TResult<Self> {
        iter.collect()
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        ChunkedArray::full_null("".into(), len)
    }

    #[inline]
    fn uninit_ref_mut<'b>(uninit_vec: &'b mut Self::Uninit) -> Self::UninitRefMut<'b>
    where
        Option<&'a str>: 'b,
    {
        uninit_vec
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = Option<&'a str>> + TrustedLen>(iter: I) -> Self {
        let len = iter.len();
        unsafe { iter.trust_my_length(len) }.collect_trusted()
    }

    #[inline]
    fn try_collect_from_trusted<I: Iterator<Item = TResult<Option<&'a str>>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self> {
        let len = iter.len();
        unsafe { iter.trust_my_length(len) }.try_collect_ca_trusted("".into())
    }
}

impl<'a> UninitVec<Option<&'a str>> for ChunkedArray<StringType> {
    type Vec = ChunkedArray<StringType>;

    #[inline(always)]
    unsafe fn assume_init(self) -> Self::Vec {
        self
    }

    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<&'a str>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl<'a> UninitRefMut<Option<&'a str>> for &mut ChunkedArray<StringType> {
    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<&'a str>) {
        unimplemented!("polars backend do not support set in given index");
    }
}
