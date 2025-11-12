#[cfg(feature = "time")]
use tea_deps::chrono::NaiveDateTime;
use tea_deps::polars::prelude::*;
use tea_deps::polars_arrow::legacy::utils::CustomIterTools;

// #[cfg(feature = "time")]
// use tea_dtype::{DateTime, unit};
use crate::prelude::*;

macro_rules! impl_for_ca {
    (to_iter, $real: ty => $($ForType: ty),*) => {
        $(
            impl TIter<Option<$real>> for $ForType {
                #[inline]
                fn titer(&self) -> impl TIterator<Item=Option<$real>>
                {
                    self.into_iter()
                }


                #[inline]
                fn tditer(&self) -> impl TDoubleIterator<Item=Option<$real>>{
                    self.into_iter()
                }
            }
        )*
    };

    (view $type: ty, $real: ty => $($ForType: ty),*) => {
        $(
            impl Vec1View<Option<$real>> for $ForType
            {
                type SliceOutput<'a> = ChunkedArray<$type>
                where
                    Self: 'a,
                    Option<$real>: 'a;

                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
                where
                    Self: 'a,
                    Option<$real>: 'a,
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
                unsafe fn uget(&self, index: usize) -> Option<$real> { unsafe {
                    self.get_unchecked(index)
                }}
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

// impl<'s> TIter<Option<&'s str>> for ChunkedArray<StringType> {
//     #[inline]
//     fn titer<'a>(&'a self) -> impl TIterator<Item = Self::Item> + 'a {
//         // let iter = self.into_iter();
//         // let i: Box<dyn PolarsIterator<Item = Option<&'s str>> + 'a> =
//         //     unsafe { std::mem::transmute(iter) };
//         // unsafe { std::mem::transmute(iter) }
//         self.into_iter().cloned()
//     }
// }

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

#[cfg(feature = "time")]
impl GetLen for DatetimeChunked {
    #[inline]
    fn len(&self) -> usize {
        Logical::len(self)
    }
}

#[cfg(feature = "time")]
impl TIter<Option<NaiveDateTime>> for &DatetimeChunked {
    #[inline]
    fn titer(&self) -> impl TIterator<Item = Option<NaiveDateTime>> {
        use tea_deps::polars::prelude::*;
        use tea_deps::polars_arrow::temporal_conversions::{
            timestamp_ms_to_datetime, timestamp_ns_to_datetime, timestamp_us_to_datetime,
        };
        let func = match self.time_unit() {
            TimeUnit::Nanoseconds => timestamp_ns_to_datetime,
            TimeUnit::Microseconds => timestamp_us_to_datetime,
            TimeUnit::Milliseconds => timestamp_ms_to_datetime,
        };
        // we know the iterators len
        unsafe {
            self.physical()
                .downcast_iter()
                .flat_map(move |iter| iter.into_iter().map(move |opt_v| opt_v.copied().map(func)))
                .trust_my_length(self.len())
        }
    }
}

// impl TIter<DateTime<unit::Nanosecond>> for &DatetimeChunked {
//     #[inline]
//     fn titer(&self) -> impl TIterator<Item = DateTime<unit::Nanosecond>> {
//         use tea_deps::polars::prelude::{DataType, TimeUnit};
//         match self.dtype() {
//             DataType::Datetime(TimeUnit::Nanoseconds, _) => {
//                 // TODO(Teamon): support timezone in future
//                 self.iter().map(|v| v.cast())
//             },
//             _ => unreachable!("datetime chunked should be nanoseconds unit"),
//         }
//     }
// }

// #[cfg(feature = "time")]
// impl TIter<DateTime<unit::Millisecond>> for &DatetimeChunked {
//     #[inline]
//     fn titer(&self) -> impl TIterator<Item = DateTime<unit::Millisecond>> {
//         use tea_deps::polars::prelude::{DataType, TimeUnit};
//         match self.dtype() {
//             DataType::Datetime(TimeUnit::Microseconds, _) => {
//                 // TODO(Teamon): support timezone in future
//                 self.into_iter().map(|v| v.cast())
//             },
//             _ => unreachable!("datetime chunked should be milliseconds unit"),
//         }
//     }
// }

// #[cfg(feature = "time")]
// impl TIter<DateTime<unit::Microsecond>> for &DatetimeChunked {
//     #[inline]
//     fn titer(&self) -> impl TIterator<Item = DateTime<unit::Microsecond>> {
//         use tea_deps::polars::prelude::{DataType, TimeUnit};
//         match self.dtype() {
//             DataType::Datetime(TimeUnit::Microseconds, _) => {
//                 // TODO(Teamon): support timezone in future
//                 self.into_iter().map(|v| v.cast())
//             },
//             _ => unreachable!("datetime chunked should be microseconds unit"),
//         }
//     }
// }
