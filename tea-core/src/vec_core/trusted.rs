// pub use std::iter::TrustedLen;

#[cfg(feature = "pl")]
use polars_arrow::trusted_len::TrustedLen as PlTrustedLen;
use std::iter::Scan;
use std::slice::Iter;

/// An iterator of known, fixed size.
/// A trait denoting Rusts' unstable [TrustedLen](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html).
/// This is re-defined here and implemented for some iterators until `std::iter::TrustedLen`
/// is stabilized.
///
/// # Safety
/// This trait must only be implemented when the contract is upheld.
/// Consumers of this trait must inspect Iterator::size_hint()’s upper bound.
#[cfg(not(feature = "pl"))]
pub unsafe trait TrustedLen: Iterator {}

/// An iterator of known, fixed size.
/// A trait denoting Rusts' unstable [TrustedLen](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html).
/// This is re-defined here and implemented for some iterators until `std::iter::TrustedLen`
/// is stabilized.
///
/// # Safety
/// This trait must only be implemented when the contract is upheld.
/// Consumers of this trait must inspect Iterator::size_hint()’s upper bound.
#[cfg(feature = "pl")]
pub unsafe trait TrustedLen: Iterator + PlTrustedLen {}

unsafe impl<T> TrustedLen for Iter<'_, T> {}

unsafe impl<'a, I, T: 'a> TrustedLen for std::iter::Copied<I>
where
    I: TrustedLen<Item = &'a T>,
    T: Copy,
{
}
unsafe impl<'a, I, T: 'a> TrustedLen for std::iter::Cloned<I>
where
    I: TrustedLen<Item = &'a T>,
    T: Clone,
{
}

unsafe impl<I> TrustedLen for std::iter::Enumerate<I> where I: TrustedLen {}

unsafe impl<A, B> TrustedLen for std::iter::Zip<A, B>
where
    A: TrustedLen,
    B: TrustedLen,
{
}

unsafe impl<T> TrustedLen for std::slice::ChunksExact<'_, T> {}

unsafe impl<T> TrustedLen for std::slice::Windows<'_, T> {}

unsafe impl<A, B> TrustedLen for std::iter::Chain<A, B>
where
    A: TrustedLen,
    B: TrustedLen<Item = A::Item>,
{
}

unsafe impl<T> TrustedLen for std::iter::Once<T> {}

unsafe impl<T> TrustedLen for std::vec::IntoIter<T> {}

unsafe impl<A: Clone> TrustedLen for std::iter::Repeat<A> {}
unsafe impl<A, F: FnMut() -> A> TrustedLen for std::iter::RepeatWith<F> {}
unsafe impl<A: TrustedLen> TrustedLen for std::iter::Take<A> {}

#[cfg(feature = "pl")]
unsafe impl<T> PlTrustedLen for &mut dyn TrustedLen<Item = T> {}
#[cfg(feature = "pl")]
unsafe impl<T> PlTrustedLen for Box<dyn TrustedLen<Item = T> + '_> {}
unsafe impl<T> TrustedLen for &mut dyn TrustedLen<Item = T> {}
unsafe impl<T> TrustedLen for Box<dyn TrustedLen<Item = T> + '_> {}

unsafe impl<B, I: TrustedLen, T: FnMut(I::Item) -> B> TrustedLen for std::iter::Map<I, T> {}

unsafe impl<I: TrustedLen + DoubleEndedIterator> TrustedLen for std::iter::Rev<I> {}

unsafe impl<T> TrustedLen for std::ops::Range<T> where std::ops::Range<T>: Iterator {}
unsafe impl<T> TrustedLen for std::ops::RangeInclusive<T> where std::ops::RangeInclusive<T>: Iterator
{}
unsafe impl<A: TrustedLen> TrustedLen for std::iter::StepBy<A> {}

unsafe impl<I, St, F, B> TrustedLen for Scan<I, St, F>
where
    F: FnMut(&mut St, I::Item) -> Option<B>,
    I: TrustedLen + Iterator<Item = B>,
{
}

// unsafe impl<K, V> TrustedLen for std::collections::hash_map::IntoIter<K, V> {}
// unsafe impl<K, V> TrustedLen for std::collections::hash_map::IntoValues<K, V> {}

#[derive(Clone)]
pub struct TrustIter<I: Iterator<Item = J>, J> {
    iter: I,
    len: usize,
}

impl<I, J> TrustIter<I, J>
where
    I: Iterator<Item = J>,
{
    #[inline]
    pub fn new(iter: I, len: usize) -> Self {
        Self { iter, len }
    }
}

impl<I, J> Iterator for TrustIter<I, J>
where
    I: Iterator<Item = J>,
{
    type Item = J;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I, J> ExactSizeIterator for TrustIter<I, J> where I: Iterator<Item = J> {}

impl<I, J> DoubleEndedIterator for TrustIter<I, J>
where
    I: Iterator<Item = J> + DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

unsafe impl<I: Iterator<Item = J>, J> PlTrustedLen for TrustIter<I, J> {}
unsafe impl<I: Iterator<Item = J>, J> TrustedLen for TrustIter<I, J> {}

pub trait ToTrustIter: IntoIterator {
    fn to_trust(self, len: usize) -> TrustIter<Self::IntoIter, Self::Item>;
}

impl<I: IntoIterator> ToTrustIter for I {
    fn to_trust(self, len: usize) -> TrustIter<Self::IntoIter, Self::Item> {
        TrustIter::new(self.into_iter(), len)
    }
}

// /// The remain length of the iterator can be trusted
// ///
// /// # Safety
// ///
// /// the size hint of the iterator should be correct
// pub unsafe trait TrustedLen {}

pub trait CollectTrusted<T> {
    fn collect_from_trusted<I>(i: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: TrustedLen;
}

impl<T> CollectTrusted<T> for Vec<T> {
    /// safety: upper bound on the remaining length of the iterator must be correct.
    fn collect_from_trusted<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: TrustedLen,
    {
        let iter = iter.into_iter();
        let len = iter
            .size_hint()
            .1
            .expect("The iterator must have an upper bound");
        let mut vec = Vec::<T>::with_capacity(len);
        let mut ptr = vec.as_mut_ptr();
        unsafe {
            for v in iter {
                std::ptr::write(ptr, v);
                ptr = ptr.add(1);
            }
            vec.set_len(len);
        }
        vec
    }
}

pub trait CollectTrustedToVec: Iterator + TrustedLen {
    #[inline(always)]
    fn collect_trusted_to_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        CollectTrusted::<Self::Item>::collect_from_trusted(self)
    }
}

impl<T: Iterator + TrustedLen + Sized> CollectTrustedToVec for T {}
