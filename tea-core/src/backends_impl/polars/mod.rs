mod string;
#[cfg(feature = "time")]
mod temporal;

use tea_deps::polars::prelude::*;
use tea_deps::polars_arrow::legacy::utils::CustomIterTools;

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

    (datetime_to_iter => $($ForType: ty),*) => {
        $(
            impl TIter<Option<NaiveDateTime>> for $ForType {
                #[inline]
                fn titer(&self) -> impl TIterator<Item=Option<NaiveDateTime>>
                {
                    self.tditer()
                }


                #[inline]
                fn tditer(&self) -> impl TDoubleIterator<Item=Option<NaiveDateTime>>{
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
        )*
    };

    (date_to_iter => $($ForType: ty),*) => {
        $(
            impl TIter<Option<NaiveDate>> for $ForType {
                #[inline]
                fn titer(&self) -> impl TIterator<Item=Option<NaiveDate>>
                {
                    self.tditer()
                }


                #[inline]
                fn tditer(&self) -> impl TDoubleIterator<Item=Option<NaiveDate>>{
                    use tea_deps::polars_arrow::temporal_conversions::date32_to_date;
                    unsafe {
                        self.physical()
                            .downcast_iter()
                            .flat_map(|iter| {
                                iter.into_iter()
                                    .map(|opt_v| opt_v.copied().map(date32_to_date))
                            })
                            .trust_my_length(self.len())
                    }
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
            fn uninit_ref_mut<'a>(uninit_vec: &'a mut Self::Uninit) -> Self::UninitRefMut<'a>
            where
                Option<$real>: 'a
            {
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

    (unnit $real: ty => $($ForType: ty),*) => {
        $(
            impl UninitVec<Option<$real>> for $ForType
            {
                type Vec = $ForType;

                #[inline(always)]
                unsafe fn assume_init(self) -> Self::Vec {
                    self
                }

                #[inline]
                unsafe fn uset(&mut self, _idx: usize, _v: Option<$real>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }


            impl UninitRefMut<Option<$real>> for &mut $ForType {
                #[inline]
                unsafe fn uset(&mut self, _idx: usize, _v: Option<$real>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }

        )*
    };

    ($($type:ty: $real: ty),*) => {
        $(
            impl_for_ca!(to_iter, $real=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view $type, $real=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view_mut $real=>ChunkedArray<$type>);
            impl_for_ca!(vec $real=>ChunkedArray<$type>);
            impl_for_ca!(unnit $real=>ChunkedArray<$type>);
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
