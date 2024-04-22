mod iter;
mod iter_traits;
mod trusted;
mod uninit;

pub use uninit::UninitVec;

use crate::prelude::IsNone;
pub use iter::{IntoIter, OptIter, ToIter};
pub use iter_traits::IterBasic;
use tea_dtype::{Cast, Opt};
pub use trusted::{CollectTrustedToVec, ToTrustIter, TrustIter, TrustedLen};

pub trait Vec1View: ToIter {
    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> Self::Item;

    #[inline]
    fn to_iter<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>>
    where
        Self::Item: 'a,
    {
        self.to_iterator()
    }

    #[inline]
    fn iter_cast<U>(&self) -> TrustIter<impl Iterator<Item = U>>
    where
        Self::Item: Cast<U>,
    {
        TrustIter::new(self.to_iterator().map(|v| v.cast()), self.len())
    }

    #[inline]
    fn opt_iter_cast<U>(&self) -> TrustIter<impl Iterator<Item = Option<U>>>
    where
        Self::Item: Opt,
        <Self::Item as Opt>::Value: Cast<U>,
    {
        TrustIter::new(
            self.to_iterator().map(|v| v.map_to(Cast::<U>::cast)),
            self.len(),
        )
    }

    #[inline]
    fn to_opt(&self) -> OptIter<Self>
    where
        Self::Item: IsNone,
        Self: Sized,
    {
        OptIter { view: self }
    }

    #[inline]
    fn to_opt_iter<'a>(
        &'a self,
    ) -> TrustIter<impl Iterator<Item = Option<<Self::Item as IsNone>::Inner>>>
    where
        Self::Item: IsNone + 'a,
    {
        TrustIter::new(self.to_iterator().map(|v| v.to_opt()), self.len())
    }

    /// if the value is valid, return it, otherwise return None
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    #[inline]
    unsafe fn uvget(&self, index: usize) -> Option<Self::Item>
    where
        Self::Item: IsNone,
    {
        let v = self.uget(index);
        if v.is_none() {
            None
        } else {
            Some(v)
        }
    }

    #[inline]
    fn get(&self, index: usize) -> Self::Item {
        if index < self.len() {
            unsafe { self.uget(index) }
        } else {
            panic!("Index out of bounds")
        }
    }

    #[inline]
    fn vget(&self, index: usize) -> Option<Self::Item>
    where
        Self::Item: IsNone,
    {
        if index < self.len() {
            unsafe { self.uvget(index) }
        } else {
            None
        }
    }

    // #[inline]
    // fn vfold<U, F>(&self, init: U, mut f: F) -> U
    // where
    //     F: FnMut(U, Self::Item) -> U,
    //     Self::Item: IsNone,
    // {
    //     self.to_iter()
    //         .fold(init, |acc, v| if v.not_none() { f(acc, v) } else { acc })
    // }

    // #[inline]
    // fn vfold_n<U, F>(&self, init: U, mut f: F) -> (usize, U)
    // where
    //     F: FnMut(U, Self::Item) -> U,
    //     Self::Item: IsNone,
    // {
    //     let mut n = 0;
    //     let acc = self.to_iter().fold(init, |acc, v| {
    //         if v.not_none() {
    //             n += 1;
    //             f(acc, v)
    //         } else {
    //             acc
    //         }
    //     });
    //     (n, acc)
    // }
}

pub trait Vec1Mut<'a>: Vec1View {
    // type OutType;
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget_mut(&'a mut self, index: usize) -> &'a mut Self::Item;

    #[inline]
    fn get_mut(&'a mut self, index: usize) -> Option<&'a mut Self::Item> {
        if index < self.len() {
            Some(unsafe { self.uget_mut(index) })
        } else {
            None
        }
    }
}

/// a vector owns its data is not necessarily mutable
pub trait Vec1: Vec1View + Sized {
    fn collect_from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> Self;

    fn uninit<'a>(len: usize) -> impl UninitVec<'a, Self::Item, Vec = Self>
    where
        Self::Item: Copy + 'a;

    #[inline]
    fn collect_from_trusted<I: Iterator<Item = Self::Item> + TrustedLen>(iter: I) -> Self {
        Self::collect_from_iter(iter)
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
        let iter = iter.map(|v| v.unwrap_or(Self::Item::none()));
        Self::collect_from_iter(iter)
    }

    // #[inline]
    // fn collect_opt_from_trusted<I: Iterator<Item = Option<Self::Item>> + TrustedLen>(
    //     iter: I,
    // ) -> Self
    // where
    //     Self::Item: IsNone,
    // {
    //     Self::collect_from_opt_iter(iter)
    // }

    // #[inline]
    // fn collect_opt_with_len<I: Iterator<Item = Option<Self::Item>>>(iter: I, len: usize) -> Self
    // where
    //     Self::Item: IsNone
    // {
    //     Self::collect_opt_from_trusted(TrustIter::new(iter, len))
    // }

    #[inline]
    fn empty() -> Self {
        Self::collect_from_iter(std::iter::empty())
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

pub trait Vec1DOptCollect<T: IsNone>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1_opt<O: Vec1<Item = T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_from_opt_iter(self.into_iter())
    }

    // #[inline]
    // fn collect_trusted_vec1_opt<O: Vec1<Item = T>>(self) -> O
    // where
    //     Self: Sized,
    //     Self::IntoIter: TrustedLen,
    // {
    //     <O as Vec1>::collect_opt_from_trusted(self.into_iter())
    // }

    // #[inline]
    // fn collect_vec1_opt_with_len<O: Vec1<Item = T>>(self, len: usize) -> O
    // where
    //     Self: Sized,
    // {
    //     <O as Vec1>::collect_opt_with_len(self.into_iter(), len)
    // }
}

impl<T: IntoIterator + Sized> Vec1Collect for T {}
impl<I: IntoIterator<Item = Option<T>>, T: IsNone> Vec1DOptCollect<T> for I {}
