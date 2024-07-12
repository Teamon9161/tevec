use std::borrow::Cow;

use polars::prelude::*;
use polars_arrow::legacy::utils::CustomIterTools;
#[cfg(feature = "time")]
use tea_dtype::{unit, DateTime};

use crate::prelude::*;

macro_rules! impl_for_ca {
    (to_iter, $real: ty => $($ForType: ty),*) => {
        $(
            impl TIter<Option<$real>> for $ForType {
                #[inline]
                fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item=Option<$real>>>
                where Option<$real>: 'a
                {
                    TrustIter::new(self.into_iter(), self.len())
                }
            }
        )*
    };

    (view $type: ty, $real: ty => $($ForType: ty),*) => {
        $(
            impl Slice<Option<$real>> for $ForType {
                type Output<'a> = ChunkedArray<$type>
                where
                    Self: 'a,
                    Option<$real>: 'a;
                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<std::borrow::Cow<'a, Self::Output<'a>>>
                where Option<$real>: 'a,
                {
                    if end < start {
                        tbail!("end index: {} should be large than start index: {} in slice", end, start);
                    }
                    let len = end - start;
                    Ok(std::borrow::Cow::Owned((*self).slice(start as i64, len)))
                }
            }

            impl Vec1View<Option<$real>> for $ForType
            {
                #[inline]
                unsafe fn uget(&self, index: usize) -> Option<$real> {
                    self.get_unchecked(index)
                }
            }

        )*
    };

    (view_mut $real: ty => $($ForType: ty),*) => {
        $(impl<'a> Vec1Mut<'a, Option<$real>> for $ForType
        {
            #[inline]
            unsafe fn uget_mut(&mut self, _index: usize) -> &mut Option<$real> {
                unimplemented!("get mut is not supported in polars backend");
            }
        })*
    };

    (vec $real: ty => $($ForType: ty),*) => {
        $(impl Vec1<Option<$real>> for $ForType {
            type Uninit = $ForType;
            type UninitRefMut<'a> = &'a mut $ForType;

            #[inline]
            fn collect_from_iter<I: Iterator<Item = Option<$real>>>(iter: I) -> Self {
                iter.collect()
            }

            #[inline]
            fn try_collect_from_iter<I: Iterator<Item = TResult<Option<$real>>>>(iter: I) -> TResult<Self>
            {
                iter.collect()
            }

            #[inline]
            fn uninit(len: usize) -> Self::Uninit
            {
                ChunkedArray::full_null("", len)
            }

            #[inline]
            fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_> {
                uninit_vec
            }

            #[inline]
            fn collect_from_trusted<I: Iterator<Item = Option<$real>>+TrustedLen>(iter: I) -> Self {
                iter.collect_trusted()
            }

            #[inline]
            fn try_collect_from_trusted<I: Iterator<Item = TResult<Option<$real>>> + TrustedLen>(
                iter: I,
            ) -> TResult<Self>
            where
                Option<$real>: std::fmt::Debug,
            {
                iter.try_collect_ca_trusted("")
            }
        })*
    };

    ($($type:ty: $real: ty),*) => {
        $(
            impl_for_ca!(to_iter, $real=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view $type, $real=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view_mut $real=>ChunkedArray<$type>);
            impl_for_ca!(vec $real=>ChunkedArray<$type>);

            impl UninitVec<Option<$real>> for ChunkedArray<$type>
            {
                type Vec = ChunkedArray<$type>;

                #[inline(always)]
                unsafe fn assume_init(self) -> Self::Vec {
                    self
                }

                #[inline]
                unsafe fn uset(&mut self, _idx: usize, _v: Option<$real>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }


            impl UninitRefMut<Option<$real>> for &mut ChunkedArray<$type> {
                #[inline]
                unsafe fn uset(&mut self, _idx: usize, _v: Option<$real>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }

        )*
    };
}

impl<T: PolarsDataType> GetLen for ChunkedArray<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl_for_ca!(
    Float32Type: f32,
    Float64Type: f64,
    Int32Type: i32,
    Int64Type: i64,
    BooleanType: bool
);

impl<'a: 's, 's> TIter<Option<&'s str>> for &'a ChunkedArray<StringType> {
    #[inline]
    fn titer<'b>(&'b self) -> TrustIter<impl TIterator<Item = Option<&'s str>>>
    where
        Option<&'s str>: 'b,
    {
        TrustIter::new(self.into_iter(), self.len())
    }
}

impl<'b, 's> Slice<Option<&'b str>> for &'s ChunkedArray<StringType> {
    // type Element = Option<&str>;
    type Output<'a> = ChunkedArray<StringType>
    where
        Self: 'a,
        Option<&'b str>: 'a;

    #[inline]
    fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Cow<'a, Self::Output<'a>>>
    where
        Option<&'b str>: 'a,
    {
        if end < start {
            tbail!(
                "end index: {} should be large than start index: {} in slice",
                end,
                start
            );
        }
        let len = end - start;
        Ok(std::borrow::Cow::Owned((*self).slice(start as i64, len)))
    }
}

impl<'s: 'a, 'a> Vec1View<Option<&'a str>> for &'s ChunkedArray<StringType> {
    #[inline]
    unsafe fn uget(&self, index: usize) -> Option<&'a str> {
        self.get_unchecked(index)
    }
}

#[cfg(feature = "time")]
impl GetLen for DatetimeChunked {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

#[cfg(feature = "time")]
impl<'a> TIter<DateTime<unit::Nanosecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer<'b>(&'b self) -> TrustIter<impl TIterator<Item = DateTime<unit::Nanosecond>>>
    where
        DateTime<unit::Nanosecond>: 'b,
    {
        use polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Nanoseconds, _) => {
                // TODO(Teamon): support timezone in future
                TrustIter::new(self.into_iter().map(|v| v.cast()), self.len())
            },
            _ => unreachable!("datetime chunked should be nanoseconds unit"),
        }
    }
}

#[cfg(feature = "time")]
impl<'a> TIter<DateTime<unit::Millisecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer<'b>(&'b self) -> TrustIter<impl TIterator<Item = DateTime<unit::Millisecond>>>
    where
        DateTime<unit::Millisecond>: 'b,
    {
        use polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Microseconds, _) => {
                // TODO(Teamon): support timezone in future
                TrustIter::new(self.into_iter().map(|v| v.cast()), self.len())
            },
            _ => unreachable!("datetime chunked should be milliseconds unit"),
        }
    }
}

#[cfg(feature = "time")]
impl<'a> TIter<DateTime<unit::Microsecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer<'b>(&'b self) -> TrustIter<impl TIterator<Item = DateTime<unit::Microsecond>>>
    where
        DateTime<unit::Microsecond>: 'b,
    {
        use polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Microseconds, _) => {
                // TODO(Teamon): support timezone in future
                TrustIter::new(self.into_iter().map(|v| v.cast()), self.len())
            },
            _ => unreachable!("datetime chunked should be microseconds unit"),
        }
    }
}
