mod element;
mod iter;
mod trusted;

use crate::prelude::IsNone;
pub use element::Element;
pub use iter::{IntoIter, OptIter, ToIter};
pub use trusted::{CollectTrustedToVec, ToTrustIter, TrustIter, TrustedLen};

pub trait Vec1View: ToIter {
    type Vec<U: Element>;

    fn len(&self) -> usize;

    /// Get the value at the index
    ///
    /// # Safety
    ///
    /// The index should be less than the length of the array
    unsafe fn uget(&self, index: usize) -> Self::Item;

    #[inline]
    fn to_iter<'a>(&'a self) -> TrustIter<impl Iterator<Item = Self::Item>, Self::Item>
    //impl Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
    {
        TrustIter::new(self.to_iterator(), self.len())
    }

    fn to_opt(&self) -> OptIter<Self>
    where
        Self::Item: IsNone,
        Self: Sized,
    {
        OptIter {
            view: self,
            // type_: std::marker::PhantomData,
        }
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
    fn is_empty(&self) -> bool {
        self.len() == 0
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

    #[inline]
    fn vfold<U, F>(&self, init: U, mut f: F) -> U
    where
        F: FnMut(U, Self::Item) -> U,
        Self::Item: IsNone,
    {
        self.to_iterator()
            .fold(init, |acc, v| if v.not_none() { f(acc, v) } else { acc })
    }

    #[inline]
    fn vfold_n<U, F>(&self, init: U, mut f: F) -> (usize, U)
    where
        F: FnMut(U, Self::Item) -> U,
        Self::Item: IsNone,
    {
        let mut n = 0;
        let acc = self.to_iterator().fold(init, |acc, v| {
            if v.not_none() {
                n += 1;
                f(acc, v)
            } else {
                acc
            }
        });
        (n, acc)
    }
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
            // panic!("Index out of bounds")
        }
    }
}

/// a vector owns its data is not necessarily mutable
pub trait Vec1: Vec1View + Sized
where
    Self::Item: Element,
{
    fn collect_from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> Self;

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

    #[inline]
    fn collect_opt_from_trusted<I: Iterator<Item = Option<Self::Item>> + TrustedLen>(
        iter: I,
    ) -> Self
    where
        Self::Item: IsNone,
    {
        Self::collect_from_opt_iter(iter)
    }

    #[inline]
    fn collect_opt_with_len<I: Iterator<Item = Option<Self::Item>>>(iter: I, len: usize) -> Self
    where
        Self::Item: IsNone,
    {
        Self::collect_opt_from_trusted(iter.to_trust(len))
    }

    #[inline]
    fn empty() -> Self {
        Self::collect_from_iter(std::iter::empty())
    }
}

pub trait Vec1Collect: IntoIterator
where
    Self::Item: Element,
{
    #[inline]
    fn collect_vec1<O: Vec1<Item = Self::Item>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_from_iter(self.into_iter())
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

pub trait Vec1DOptCollect<T: IsNone + Element>: IntoIterator<Item = Option<T>> {
    #[inline]
    fn collect_vec1_opt<O: Vec1<Item = T>>(self) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_from_opt_iter(self.into_iter())
    }

    #[inline]
    fn collect_trusted_vec1_opt<O: Vec1<Item = T>>(self) -> O
    where
        Self: Sized,
        Self::IntoIter: TrustedLen,
    {
        <O as Vec1>::collect_opt_from_trusted(self.into_iter())
    }

    #[inline]
    fn collect_vec1_opt_with_len<O: Vec1<Item = T>>(self, len: usize) -> O
    where
        Self: Sized,
    {
        <O as Vec1>::collect_opt_with_len(self.into_iter(), len)
    }
}

impl<T: IntoIterator + Sized> Vec1Collect for T where T::Item: Element {}
impl<I: IntoIterator<Item = Option<T>>, T: IsNone + Element> Vec1DOptCollect<T> for I {}
