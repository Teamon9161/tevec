use crate::prelude::*;
use polars::prelude::*;
use polars_arrow::legacy::utils::CustomIterTools;

macro_rules! impl_for_primitive {
    ($($type:ty),*) => {
        $(
            impl ToIter for ChunkedArray<$type> {
                type Item = Option<<$type as PolarsNumericType>::Native>;
                #[inline]
                fn to_iterator<'a>(&'a self) -> impl Iterator<Item=Self::Item>
                where Self::Item: 'a
                {
                    self.into_iter()
                }
            }

            impl Vec1View for ChunkedArray<$type>
            {
                #[inline]
                fn len(&self) -> usize {
                    (*self).len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> Option<<$type as PolarsNumericType>::Native> {
                    self.get_unchecked(index)
                }

                #[inline]
                unsafe fn uvget(&self, index: usize) -> Option<Option<<$type as PolarsNumericType>::Native>>
                {
                    Some(self.uget(index))
                }
            }

            impl ToIter for &ChunkedArray<$type> {
                type Item = Option<<$type as PolarsNumericType>::Native>;
                #[inline]
                fn to_iterator<'a>(&'a self) -> impl Iterator<Item=Self::Item>
                where Self::Item: 'a
                {
                    self.into_iter()
                }
            }

            impl Vec1View for &ChunkedArray<$type>
            {
                #[inline]
                fn len(&self) -> usize {
                    (*self).len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> Option<<$type as PolarsNumericType>::Native> {
                    self.get_unchecked(index)
                }

                #[inline]
                unsafe fn uvget(&self, index: usize) -> Option<Option<<$type as PolarsNumericType>::Native>>
                {
                    Some(self.uget(index))
                }
            }

            impl<'a> Vec1Mut<'a> for ChunkedArray<$type>
            {
                #[inline]
                unsafe fn uget_mut(&'a mut self, _index: usize) -> &mut Option<<$type as PolarsNumericType>::Native> {
                    unimplemented!()
                }
            }


            impl Vec1 for ChunkedArray<$type> {
                #[inline]
                fn collect_from_iter<I: Iterator<Item = Option<<$type as PolarsNumericType>::Native>>>(iter: I) -> Self {
                    iter.collect()
                }

                #[inline]
                fn uninit<'a>(len: usize) -> impl UninitVec<'a, Option<<$type as PolarsNumericType>::Native>>
                where Option<<$type as PolarsNumericType>::Native>: Copy
                {
                    ChunkedArray::<$type>::full_null("", len)
                }

                #[inline]
                fn collect_from_trusted<I: Iterator<Item = Option<<$type as PolarsNumericType>::Native>>+TrustedLen>(iter: I) -> Self {
                    iter.collect_trusted()
                }
            }

            impl<'a> UninitVec<'a, Option<<$type as PolarsNumericType>::Native>> for ChunkedArray<$type>
            {
                type Vec = ChunkedArray<$type>;
                #[inline(always)]
                unsafe fn assume_init(self) -> Self::Vec {
                    self
                }

                #[inline]
                unsafe fn uset(&'a mut self, _idx: usize, _v: Option<<$type as PolarsNumericType>::Native>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }

        )*
    };

    ($($type:ty: $real: ty),*) => {
        $(
            impl ToIter for ChunkedArray<$type> {
                type Item = Option<$real>;
                #[inline]
                fn to_iterator<'a>(&'a self) -> impl Iterator<Item=Option<$real>> where Option<$real>: 'a{
                    self.into_iter()
                }
            }

            impl Vec1View for ChunkedArray<$type>
            {
                #[inline]
                fn len(&self) -> usize {
                    (*self).len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> Option<$real> {
                    self.get_unchecked(index)
                }

                #[inline]
                unsafe fn uvget(&self, index: usize) -> Option<Option<$real>>
                {
                    Some(self.uget(index))
                }
            }

            impl ToIter for &ChunkedArray<$type> {
                type Item = Option<$real>;
                #[inline]
                fn to_iterator<'a>(&'a self) -> impl Iterator<Item=Option<$real>> where Option<$real>: 'a{
                    self.into_iter()
                }
            }

            impl Vec1View for &ChunkedArray<$type>
            {
                #[inline]
                fn len(&self) -> usize {
                    (*self).len()
                }

                #[inline]
                unsafe fn uget(&self, index: usize) -> Option<$real> {
                    self.get_unchecked(index)
                }

                #[inline]
                unsafe fn uvget(&self, index: usize) -> Option<Option<$real>>
                {
                    Some(self.uget(index))
                }
            }

            impl<'a> Vec1Mut<'a> for ChunkedArray<$type>
            {
                #[inline]
                unsafe fn uget_mut(&'a mut self, _index: usize) -> &mut Option<$real> {
                    unimplemented!();
                }
            }

            impl Vec1 for ChunkedArray<$type> {
                #[inline]
                fn collect_from_iter<I: Iterator<Item = Option<$real>>>(iter: I) -> Self {
                    iter.collect()
                }

                #[inline]
                fn uninit<'a>(len: usize) -> impl UninitVec<'a, Option<$real>> where Option<$real>: Copy {
                    ChunkedArray::<$type>::full_null("", len)
                }
            }

            impl<'a> UninitVec<'a, Option<$real>> for ChunkedArray<$type>
            {
                type Vec = ChunkedArray<$type>;
                #[inline(always)]
                unsafe fn assume_init(self) -> Self::Vec {
                    self
                }

                #[inline]
                unsafe fn uset(&'a mut self, _idx: usize, _v: Option<$real>) {
                    unimplemented!("polars backend do not support set in given index");
                }
            }
        )*
    };
}

impl_for_primitive!(Float32Type, Float64Type, Int32Type, Int64Type);
impl_for_primitive!(BooleanType: bool);

// impl<'a> ToIter for ChunkedArray<StringType> {
//     type Item = Option<&'a str>;
//     #[inline]
//     fn to_iterator<'b>(&'b self) -> impl Iterator<Item=Option<&'a str>> where Self::Item: 'b {
//         self.into_iter()
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
