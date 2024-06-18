use crate::prelude::*;
use polars::prelude::*;
use polars_arrow::legacy::utils::CustomIterTools;

macro_rules! impl_for_ca {
    (to_iter, $real: ty => $($ForType: ty),*) => {
        $(
            impl TIter for $ForType {
                type Item = Option<$real>;

                #[inline]
                fn titer<'a>(&'a self) -> TrustIter<impl TIterator<Item=Self::Item>>
                where Self::Item: 'a
                {
                    TrustIter::new(self.into_iter(), self.len())
                }
            }
        )*
    };

    (view $type: ty => $($ForType: ty),*) => {
        $(
            impl Slice for $ForType {
                type Element = <Self as TIter>::Item;
                type Output<'a> = ChunkedArray<$type>
                where
                    Self: 'a,
                    Self::Element: 'a;
                #[inline]
                fn slice<'a>(&'a self, start: usize, end: usize) -> TResult<std::borrow::Cow<'a, Self::Output<'a>>> where <Self::Output<'a> as TIter>::Item: 'a {
                    if end < start {
                        tbail!("end index: {} should be large than start index: {} in slice", end, start);
                    }
                    let len = end - start;
                    Ok(std::borrow::Cow::Owned((*self).slice(start as i64, len)))
                }
            }

            impl Vec1View for $ForType
            {
                #[inline]
                unsafe fn uget(&self, index: usize) -> Self::Item {
                    self.get_unchecked(index)
                }

                #[inline]
                unsafe fn uvget(&self, index: usize) -> Self::Item
                {
                    self.uget(index).to_opt()
                }
            }

        )*
    };

    (view_mut $($ForType: ty),*) => {
        $(impl<'a> Vec1Mut<'a> for $ForType
        {
            #[inline]
            unsafe fn uget_mut(&mut self, _index: usize) -> &mut Self::Item {
                unimplemented!("get mut is not supported in polars backend");
            }
        })*
    };

    (vec $($ForType: ty),*) => {
        $(impl Vec1 for $ForType {
            type Uninit = $ForType;
            type UninitRefMut<'a> = &'a mut $ForType;

            #[inline]
            fn collect_from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> Self {
                iter.collect()
            }

            #[inline]
            fn try_collect_from_iter<I: Iterator<Item = TResult<Self::Item>>>(iter: I) -> TResult<Self>
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
            fn collect_from_trusted<I: Iterator<Item = Self::Item>+TrustedLen>(iter: I) -> Self {
                iter.collect_trusted()
            }

            #[inline]
            fn try_collect_from_trusted<I: Iterator<Item = TResult<Self::Item>> + TrustedLen>(
                iter: I,
            ) -> TResult<Self>
            where
                Self::Item: std::fmt::Debug,
            {
                iter.try_collect_ca_trusted("")
            }
        })*
    };

    ($($type:ty: $real: ty),*) => {
        $(
            impl_for_ca!(to_iter, $real=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view $type=>ChunkedArray<$type>, &ChunkedArray<$type>);
            impl_for_ca!(view_mut ChunkedArray<$type>);
            impl_for_ca!(vec ChunkedArray<$type>);

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

// impl ToIter for ChunkedArray<StringType> {
//     type Item = Option<&'a str>;

//     #[inline]
//     fn len(&self) -> usize {
//         self.len()
//     }

//     #[inline]
//     fn to_iterator<'b>(&'b self) -> TrustIter<impl Iterator<Item=Self::Item>> where Self::Item: 'b {
//         TrustIter::new(self.into_iter(), self.len())
//     }
// }

// impl<'a> Vec1View for ChunkedArray<StringType>
// {
//     #[inline]
//     fn len(&self) -> usize {
//         (*self).len()
//     }

//     #[inline]
//     unsafe fn uget(&self, index: usize) -> Option<&'a str> {
//         self.get_unchecked(index)
//     }

//     #[inline]
//     unsafe fn uvget(&self, index: usize) -> Option<Option<&'a str>>
//     {
//         Some(self.uget(index))
//     }
// }
