use tea_dtype::IsNone;
use tea_error::*;

use super::super::trusted::{ToTrustIter, TrustedLen};
use super::super::uninit::{UninitRefMut, UninitVec};
use super::{Vec1Mut, Vec1View};

/// a vector owns its data is not necessarily mutable
pub trait Vec1<T>: Vec1View<T> + Sized {
    type Uninit: UninitVec<T, Vec = Self>;
    type UninitRefMut<'a>: UninitRefMut<T>
    where
        T: 'a;

    fn collect_from_iter<I: Iterator<Item = T>>(iter: I) -> Self;

    fn uninit(len: usize) -> Self::Uninit;

    fn uninit_ref_mut(uninit_vec: &mut Self::Uninit) -> Self::UninitRefMut<'_>;

    #[inline]
    fn try_collect_from_iter<I: Iterator<Item = TResult<T>>>(iter: I) -> TResult<Self> {
        Ok(Self::collect_from_iter(iter.map(|v| v.unwrap())))
    }

    #[inline]
    fn collect_from_trusted<I: TrustedLen<Item = T>>(iter: I) -> Self {
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn try_collect_from_trusted<I: TrustedLen<Item = TResult<T>>>(iter: I) -> TResult<Self>
    where
        T: std::fmt::Debug,
    {
        Self::try_collect_from_iter(iter)
    }

    #[inline]
    fn collect_with_len<I: Iterator<Item = T>>(iter: I, len: usize) -> Self {
        Self::collect_from_trusted(iter.to_trust(len))
    }

    #[inline]
    fn collect_from_opt_iter<I: Iterator<Item = Option<T>>>(iter: I) -> Self
    where
        T: IsNone,
    {
        let iter = iter.map(|v| v.unwrap_or_else(T::none));
        Self::collect_from_iter(iter)
    }

    #[inline]
    fn empty() -> Self {
        Self::collect_from_iter(std::iter::empty())
    }

    #[inline]
    fn full(len: usize, v: T) -> Self
    where
        T: Clone,
    {
        let iter = std::iter::repeat(v).take(len);
        Self::collect_from_trusted(iter)
    }

    /// sort 1d array using a compare function, but might not preserve the order of equal elements.
    fn sort_unstable_by<'a, F>(&'a mut self, compare: F) -> TResult<()>
    where
        Self: Vec1Mut<'a, T>,
        T: Clone,
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        if let Some(slc) = self.try_as_slice_mut() {
            slc.sort_unstable_by(compare);
            Ok(())
        } else {
            let mut out_c: Vec<_> = self.titer().collect_trusted_vec1();
            let slc = out_c.try_as_slice_mut().ok_or_else(|| {
                terr!("This type of 1d vector can not be sorted by the given compare function")
            })?;
            slc.sort_unstable_by(compare);
            self.apply_mut_with(&out_c, |v, vo| *v = vo)
        }
    }
}

pub trait Vec1Collect: IntoIterator {
    #[inline]
    fn collect_vec1<O: Vec1<Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        O::collect_from_iter(self.into_iter())
    }

    #[inline]
    fn collect_trusted_vec1<O: Vec1<Self::Item>>(self) -> O
    where
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1<Self::Item>>::collect_from_trusted(self.into_iter())
    }

    #[inline]
    fn collect_vec1_with_len<O: Vec1<Self::Item>>(self, len: usize) -> O
    where
        Self: Sized,
    {
        <O as Vec1<Self::Item>>::collect_with_len(self.into_iter(), len)
    }
}

pub trait Vec1OptCollect<T: IsNone>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1_opt<O: Vec1<T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1<T>>::collect_from_opt_iter(self.into_iter())
    }
}

pub trait Vec1TryCollect<T: IsNone>: IntoIterator<Item = TResult<T>> {
    #[inline]
    fn try_collect_vec1<O: Vec1<T>>(self) -> TResult<O>
    where
        Self: Sized,
    {
        <O as Vec1<T>>::try_collect_from_iter(self.into_iter())
    }

    #[inline]
    fn try_collect_trusted_vec1<O: Vec1<T>>(self) -> TResult<O>
    where
        T: std::fmt::Debug,
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1<T>>::try_collect_from_trusted(self.into_iter())
    }
}

impl<T: IntoIterator + Sized> Vec1Collect for T {}
impl<I: IntoIterator<Item = Option<T>>, T: IsNone> Vec1OptCollect<T> for I {}
impl<I: IntoIterator<Item = TResult<T>>, T: IsNone + std::fmt::Debug> Vec1TryCollect<T> for I {}
