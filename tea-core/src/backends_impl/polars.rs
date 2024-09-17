use tea_deps::polars::export::arrow::legacy::utils::CustomIterTools;
use tea_deps::polars::prelude::*;
#[cfg(feature = "time")]
use tea_dtype::{unit, DateTime};

use crate::prelude::*;

macro_rules! impl_for_ca {
    (to_iter, $real: ty => $($ForType: ty),*) => {
        $(
            impl<'a> TIter<'a, Option<$real>> for $ForType {
                #[inline]
                fn titer(&'a self) -> impl TIterator<Item=Option<$real>>
                {
                    self.into_iter()
                }
            }
        )*
    };

    (view $type: ty, $real: ty => $($ForType: ty),*) => {
        $(
            impl<'a> Vec1View<'a, Option<$real>> for $ForType
            {
                type SliceOutput<'b> = ChunkedArray<$type> where Self: 'b;

                #[inline]
                fn slice(&self, start: usize, end: usize) -> TResult<Self::SliceOutput<'_>>
                {
                    if end < start {
                        tbail!("end index: {} should be large than start index: {} in slice", end, start);
                    }
                    let len = end - start;
                    Ok((*self).slice(start as i64, len))
                }

                #[inline]
                fn get_backend_name(&self) -> &'static str {
                    "polars"
                }

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
                ChunkedArray::full_null("".into(), len)
            }

            #[inline]
            fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_> {
                uninit_vec
            }

            #[inline]
            fn collect_from_trusted<I: Iterator<Item = Option<$real>>+TrustedLen>(iter: I) -> Self {
                let len = iter.len();
                unsafe{iter.trust_my_length(len)}.collect_trusted()
            }

            #[inline]
            fn try_collect_from_trusted<I: Iterator<Item = TResult<Option<$real>>> + TrustedLen>(
                iter: I,
            ) -> TResult<Self>
            {
                let len = iter.len();
                unsafe{iter.trust_my_length(len)}.try_collect_ca_trusted("".into())
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

impl<'a> TIter<'a, Option<&'a str>> for &'a ChunkedArray<StringType> {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = Option<&'a str>> {
        self.into_iter()
    }
}

impl<'a> TIter<'a, Option<&'a str>> for ChunkedArray<StringType> {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = Option<&'a str>> {
        self.into_iter()
    }
}

impl<'a> Vec1View<'a, Option<&'a str>> for &'a ChunkedArray<StringType> {
    type SliceOutput<'b> = ChunkedArray<StringType> where Self: 'b;

    #[inline]
    fn slice(&self, start: usize, end: usize) -> TResult<Self::SliceOutput<'_>> {
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
impl<'a> TIter<'a, DateTime<unit::Nanosecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = DateTime<unit::Nanosecond>> {
        use tea_deps::polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Nanoseconds, _) => {
                // TODO(Teamon): support timezone in future
                self.into_iter().map(|v| v.cast())
            },
            _ => unreachable!("datetime chunked should be nanoseconds unit"),
        }
    }
}

#[cfg(feature = "time")]
impl<'a> TIter<'a, DateTime<unit::Millisecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = DateTime<unit::Millisecond>> {
        use tea_deps::polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Microseconds, _) => {
                // TODO(Teamon): support timezone in future
                self.into_iter().map(|v| v.cast())
            },
            _ => unreachable!("datetime chunked should be milliseconds unit"),
        }
    }
}

#[cfg(feature = "time")]
impl<'a> TIter<'a, DateTime<unit::Microsecond>> for &'a DatetimeChunked {
    #[inline]
    fn titer(&'a self) -> impl TIterator<Item = DateTime<unit::Microsecond>> {
        use tea_deps::polars::prelude::{DataType, TimeUnit};
        match self.dtype() {
            DataType::Datetime(TimeUnit::Microseconds, _) => {
                // TODO(Teamon): support timezone in future
                self.into_iter().map(|v| v.cast())
            },
            _ => unreachable!("datetime chunked should be microseconds unit"),
        }
    }
}
