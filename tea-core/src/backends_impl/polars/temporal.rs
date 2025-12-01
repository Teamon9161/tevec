use tea_deps::chrono::{NaiveDate, NaiveDateTime};
use tea_deps::polars::prelude::*;
use tea_deps::polars_arrow::legacy::utils::CustomIterTools;
use tea_deps::polars_arrow::temporal_conversions::{
    date32_to_date, timestamp_ms_to_datetime, timestamp_ns_to_datetime, timestamp_us_to_datetime,
};

use crate::prelude::*;

macro_rules! impl_for_ca {
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
                    use tea_deps::polars::prelude::TimeUnit;
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

    (date_view => $($ForType: ty),*) => {
        $(
            impl Vec1View<Option<NaiveDate>> for $ForType
            {
                type SliceOutput<'a>
                    = DateChunked
                where
                    Self: 'a,
                    Option<NaiveDate>: 'a;

                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
                where
                    Self: 'a,
                    Option<NaiveDate>: 'a,
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
                unsafe fn uget(&self, index: usize) -> Option<NaiveDate> {
                    let anyvalue = unsafe { self.get_any_value_unchecked(index) };
                    match anyvalue {
                        AnyValue::Date(v) => Some(date32_to_date(v)),
                        AnyValue::Null => None,
                        _ => panic!("can not cast {anyvalue:?} to date"),
                    }
                }
            }
        )*
    };

    (datetime_view => $($ForType: ty),*) => {
        $(
            impl Vec1View<Option<NaiveDateTime>> for $ForType
            {
                type SliceOutput<'a>
                    = DatetimeChunked
                where
                    Self: 'a,
                    Option<NaiveDateTime>: 'a;

                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
                where
                    Self: 'a,
                    Option<NaiveDateTime>: 'a,
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
                unsafe fn uget(&self, index: usize) -> Option<NaiveDateTime> {
                    use tea_deps::polars::prelude::TimeUnit;
                    let anyvalue = unsafe { self.get_any_value_unchecked(index) };
                    match anyvalue {
                        AnyValue::Datetime(v, unit, _timezone) => match unit {
                            TimeUnit::Nanoseconds => Some(timestamp_ns_to_datetime(v)),
                            TimeUnit::Microseconds => Some(timestamp_us_to_datetime(v)),
                            TimeUnit::Milliseconds => Some(timestamp_ms_to_datetime(v)),
                        },
                        AnyValue::Null => None,
                        _ => panic!("can not cast {anyvalue:?} to datetime"),
                    }
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
}

impl GetLen for DatetimeChunked {
    #[inline]
    fn len(&self) -> usize {
        Logical::len(self)
    }
}

impl_for_ca!(datetime_to_iter => &DatetimeChunked, DatetimeChunked);

impl GetLen for DateChunked {
    #[inline]
    fn len(&self) -> usize {
        Logical::len(self)
    }
}

impl_for_ca!(date_to_iter => &DateChunked, DateChunked);
impl_for_ca!(date_view => &DateChunked, DateChunked);
impl_for_ca!(datetime_view => &DatetimeChunked, DatetimeChunked);
impl_for_ca!(view_mut NaiveDateTime => DatetimeChunked);
impl_for_ca!(view_mut NaiveDate => DateChunked);

impl UninitVec<Option<NaiveDate>> for DateChunked {
    type Vec = DateChunked;

    #[inline(always)]
    unsafe fn assume_init(self) -> Self::Vec {
        self
    }

    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<NaiveDate>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl UninitRefMut<Option<NaiveDate>> for &mut DateChunked {
    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<NaiveDate>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl Vec1<Option<NaiveDate>> for DateChunked {
    type Uninit = DateChunked;
    type UninitRefMut<'a> = &'a mut DateChunked;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = Option<NaiveDate>>>(iter: I) -> Self {
        DateChunked::from_naive_date_options("".into(), iter)
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        DateChunked::new("".into(), Vec::<NaiveDate>::with_capacity(len))
    }

    #[inline]
    fn uninit_ref_mut<'a>(uninit_vec: &'a mut Self::Uninit) -> Self::UninitRefMut<'a>
    where
        Option<NaiveDate>: 'a,
    {
        uninit_vec
    }
}

impl UninitVec<Option<NaiveDateTime>> for DatetimeChunked {
    type Vec = DatetimeChunked;

    #[inline(always)]
    unsafe fn assume_init(self) -> Self::Vec {
        self
    }

    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<NaiveDateTime>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl UninitRefMut<Option<NaiveDateTime>> for &mut DatetimeChunked {
    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<NaiveDateTime>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl Vec1<Option<NaiveDateTime>> for DatetimeChunked {
    type Uninit = DatetimeChunked;
    type UninitRefMut<'a> = &'a mut DatetimeChunked;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = Option<NaiveDateTime>>>(iter: I) -> Self {
        use tea_deps::polars::prelude::TimeUnit;
        let iter = iter
            .into_iter()
            .map(|opt| opt.and_then(|dt| dt.and_utc().timestamp_nanos_opt()));
        Int64Chunked::from_iter_options("".into(), iter).into_datetime(TimeUnit::Nanoseconds, None)
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        DatetimeChunked::new("".into(), Vec::<NaiveDateTime>::with_capacity(len))
    }

    #[inline]
    fn uninit_ref_mut<'a>(uninit_vec: &'a mut Self::Uninit) -> Self::UninitRefMut<'a>
    where
        Option<NaiveDateTime>: 'a,
    {
        uninit_vec
    }
}
