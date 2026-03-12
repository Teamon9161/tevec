use tea_deps::polars::prelude::*;
use tea_deps::polars_arrow::legacy::utils::CustomIterTools;
use tea_deps::rust_decimal::Decimal;

use crate::prelude::*;

macro_rules! impl_for_decimal_ca {
    (to_iter => $($ForType: ty),*) => {
        $(
            impl TIter<Option<Decimal>> for $ForType {
                #[inline]
                fn titer(&self) -> impl TIterator<Item = Option<Decimal>> {
                    self.tditer()
                }

                #[inline]
                fn tditer(&self) -> impl TDoubleIterator<Item = Option<Decimal>> {
                    let scale = match self.dtype() {
                        tea_deps::polars::prelude::DataType::Decimal(_, s) => *s as u32,
                        _ => 0,
                    };
                    unsafe {
                        self.physical()
                            .downcast_iter()
                            .flat_map(move |iter| {
                                iter.into_iter()
                                    .map(move |opt_v| opt_v.copied().map(|v| Decimal::from_i128_with_scale(v, scale)))
                            })
                            .trust_my_length(self.len())
                    }
                }
            }
        )*
    };

    (view => $($ForType: ty),*) => {
        $(
            impl Vec1View<Option<Decimal>> for $ForType {
                type SliceOutput<'a>
                    = DecimalChunked
                where
                    Self: 'a,
                    Option<Decimal>: 'a;

                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<Self::SliceOutput<'a>>
                where
                    Self: 'a,
                    Option<Decimal>: 'a,
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
                unsafe fn uget(&self, index: usize) -> Option<Decimal> {
                    let anyvalue = unsafe { self.get_any_value_unchecked(index) };
                    match anyvalue {
                        AnyValue::Decimal(v, _precision, scale) => {
                            Some(Decimal::from_i128_with_scale(v, scale as u32))
                        }
                        AnyValue::Null => None,
                        _ => panic!("can not cast {anyvalue:?} to decimal"),
                    }
                }
            }
        )*
    };

    (view_mut => $($ForType: ty),*) => {
        $(
            impl<'a> Vec1Mut<'a, Option<Decimal>> for $ForType {
                #[inline]
                unsafe fn uget_mut(&mut self, _index: usize) -> &mut Option<Decimal> {
                    unimplemented!("get mut is not supported in polars backend");
                }
            }
        )*
    };
}

impl GetLen for DecimalChunked {
    #[inline]
    fn len(&self) -> usize {
        Logical::len(self)
    }
}

impl_for_decimal_ca!(to_iter => DecimalChunked, &DecimalChunked);
impl_for_decimal_ca!(view => DecimalChunked, &DecimalChunked);
impl_for_decimal_ca!(view_mut => DecimalChunked);

impl UninitVec<Option<Decimal>> for DecimalChunked {
    type Vec = DecimalChunked;

    #[inline(always)]
    unsafe fn assume_init(self) -> Self::Vec {
        self
    }

    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<Decimal>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl UninitRefMut<Option<Decimal>> for &mut DecimalChunked {
    #[inline]
    unsafe fn uset(&mut self, _idx: usize, _v: Option<Decimal>) {
        unimplemented!("polars backend do not support set in given index");
    }
}

impl Vec1<Option<Decimal>> for DecimalChunked {
    type Uninit = DecimalChunked;
    type UninitRefMut<'a> = &'a mut DecimalChunked;

    #[inline]
    fn collect_from_iter<I: Iterator<Item = Option<Decimal>>>(iter: I) -> Self {
        let items: Vec<Option<Decimal>> = iter.collect();
        let scale = items
            .iter()
            .filter_map(|opt| *opt)
            .map(|d| d.scale())
            .max_by_key(|&s| s)
            .unwrap_or(0);
        let physical: Int128Chunked = items
            .into_iter()
            .map(|opt| {
                opt.map(|mut d| {
                    d.rescale(scale);
                    d.mantissa()
                })
            })
            .collect();
        physical.into_decimal_unchecked(0, scale as usize)
    }

    #[inline]
    fn uninit(len: usize) -> Self::Uninit {
        Int128Chunked::full_null("".into(), len).into_decimal_unchecked(0, 0)
    }

    #[inline]
    fn uninit_ref_mut<'a>(uninit_vec: &'a mut Self::Uninit) -> Self::UninitRefMut<'a>
    where
        Option<Decimal>: 'a,
    {
        uninit_vec
    }
}
