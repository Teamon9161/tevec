use super::super::{
    trusted::{ToTrustIter, TrustedLen},
    uninit::{UninitRefMut, UninitVec},
};
use super::Vec1View;
use tea_dtype::IsNone;
use tea_error::*;

/// a vector owns its data is not necessarily mutable
pub trait Vec1: Vec1View + Sized {
    type Uninit: UninitVec<Self::Item, Vec = Self>;
    type UninitRefMut<'a>: UninitRefMut<Self::Item>
    where
        Self::Item: 'a;

    fn collect_from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> Self;

    fn uninit(len: usize) -> Self::Uninit;

    fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_>;

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<Self::Item>>>(iter: I) -> TResult<Self> {
        Ok(Self::collect_from_iter(iter.map(|v| v.unwrap())))
    }

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = Self::Item> + TrustedLen>(iter: I) -> Self {
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn try_collect_from_trusted<I: Iterator<Item = TResult<Self::Item>> + TrustedLen>(
        iter: I,
    ) -> TResult<Self>
    where
        Self::Item: std::fmt::Debug,
    {
        Self::try_collect_from_iter(iter)
    }

    #[inline]
    fn collect_with_len<I: Iterator<Item = Self::Item>>(iter: I, len: usize) -> Self {
        Self::collect_from_trusted(iter.to_trust(len))
    }

    #[inline]
    fn collect_from_opt_iter<I: Iterator<Item = Option<Self::Item>>>(iter: I) -> Self
    where
        Self::Item: IsNone,
    {
        let iter = iter.map(|v| v.unwrap_or_else(Self::Item::none));
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn empty() -> Self {
        Self::collect_from_iter(std::iter::empty())
    }

    #[inline]
    fn full(len: usize, v: Self::Item) -> Self
    where
        Self::Item: Clone,
    {
        let iter = std::iter::repeat(v).take(len);
        Self::collect_from_trusted(iter)
    }
}

pub trait Vec1Collect: IntoIterator {
    #[inline]
    fn collect_vec1<O: Vec1<Item = Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        O::collect_from_iter(self.into_iter())
    }

    #[inline]
    fn collect_trusted_vec1<O: Vec1<Item = Self::Item>>(self) -> O
    where
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1>::collect_from_trusted(self.into_iter())
    }

    #[inline]
    fn collect_vec1_with_len<O: Vec1<Item = Self::Item>>(self, len: usize) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_with_len(self.into_iter(), len)
    }
}

pub trait Vec1OptCollect<T: IsNone>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1_opt<O: Vec1<Item = T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_from_opt_iter(self.into_iter())
    }
}

pub trait Vec1TryCollect<T: IsNone>: IntoIterator<Item = TResult<T>> {
    #[inline]
    fn try_collect_vec1<O: Vec1<Item = T>>(self) -> TResult<O>
    where
        Self: Sized,
    {
        <O as Vec1>::try_collect_from_iter(self.into_iter())
    }

    #[inline]
    fn try_collect_trusted_vec1<O: Vec1<Item = T>>(self) -> TResult<O>
    where
        T: std::fmt::Debug,
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1>::try_collect_from_trusted(self.into_iter())
    }
}

impl<T: IntoIterator + Sized> Vec1Collect for T {}
impl<I: IntoIterator<Item = Option<T>>, T: IsNone> Vec1OptCollect<T> for I {}
impl<I: IntoIterator<Item = TResult<T>>, T: IsNone + std::fmt::Debug> Vec1TryCollect<T> for I {}
