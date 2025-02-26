use std::error::Error;
use std::iter::Scan;
use std::slice::Iter;

#[cfg(feature = "polars")]
pub(crate) use tea_deps::polars_arrow::trusted_len::TrustedLen as PlTrustedLen;
#[cfg(feature = "polars")]
use tea_deps::polars::prelude::PolarsIterator;

/// An iterator of known, fixed size.
///
/// A trait denoting Rusts' unstable [TrustedLen](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html).
/// This is re-defined here and implemented for some iterators until `std::iter::TrustedLen`
/// is stabilized.
///
/// # Safety
/// This trait must only be implemented when the contract is upheld.
/// Consumers of this trait must inspect Iterator::size_hint()’s upper bound.
// #[cfg(not(feature = "polars"))]
pub unsafe trait TrustedLen: Iterator {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().1.unwrap()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

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

unsafe impl<I> TrustedLen for std::iter::Empty<I> {}
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

#[cfg(feature = "polars")]
unsafe impl<T> PlTrustedLen for &mut dyn TrustedLen<Item = T> {}
#[cfg(feature = "polars")]
unsafe impl<T> PlTrustedLen for Box<dyn TrustedLen<Item = T> + '_> {}
#[cfg(feature = "polars")]
unsafe impl<T> TrustedLen for &mut dyn PlTrustedLen<Item = T> {}
#[cfg(feature = "polars")]
unsafe impl<T> TrustedLen for Box<dyn PlTrustedLen<Item = T> + '_> {}
#[cfg(feature = "polars")]
unsafe impl<T> TrustedLen for dyn PolarsIterator<Item = T> {}
#[cfg(feature = "polars")]
unsafe impl<T> TrustedLen for Box<dyn PolarsIterator<Item = T> + '_> {}

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

#[cfg(feature = "ndarray")]
unsafe impl<A, D: tea_deps::ndarray::Dimension> TrustedLen
    for tea_deps::ndarray::iter::Iter<'_, A, D>
{
}
#[cfg(feature = "ndarray")]
unsafe impl<A, D: tea_deps::ndarray::Dimension> TrustedLen
    for tea_deps::ndarray::iter::IterMut<'_, A, D>
{
}

// unsafe impl<K, V> TrustedLen for std::collections::hash_map::IntoIter<K, V> {}
// unsafe impl<K, V> TrustedLen for std::collections::hash_map::IntoValues<K, V> {}

#[cfg(feature = "vecdeque")]
unsafe impl<T> TrustedLen for std::collections::vec_deque::IntoIter<T> {}
#[cfg(feature = "vecdeque")]
unsafe impl<T> TrustedLen for std::collections::vec_deque::Iter<'_, T> {}

/// A wrapper struct for an iterator with a known length.
///
/// `TrustIter` wraps an iterator and stores its length, allowing it to implement
/// `TrustedLen` and provide more efficient size hints.
///
/// # Type Parameters
///
/// * `I`: The type of the wrapped iterator, which must implement `Iterator`.
///
/// # Fields
///
/// * `iter`: The wrapped iterator.
/// * `len`: The known length of the iterator.
#[derive(Clone)]
pub struct TrustIter<I: Iterator> {
    iter: I,
    len: usize,
}

impl<I> TrustIter<I>
where
    I: Iterator,
{
    #[inline]
    pub fn new(iter: I, len: usize) -> Self {
        Self { iter, len }
    }
}

impl<I> Iterator for TrustIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I> ExactSizeIterator for TrustIter<I> where I: Iterator {}

impl<I> DoubleEndedIterator for TrustIter<I>
where
    I: Iterator + DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

#[cfg(feature = "polars")]
unsafe impl<I: Iterator> PlTrustedLen for TrustIter<I> {}
unsafe impl<I: Iterator> TrustedLen for TrustIter<I> {}

/// A trait for converting an iterator into a `TrustIter`.
///
/// This trait provides a method to wrap an iterator with a known length
/// into a `TrustIter`, which implements `TrustedLen`.
pub trait ToTrustIter: IntoIterator {
    /// Converts the iterator into a `TrustIter` with a known length.
    ///
    /// # Arguments
    ///
    /// * `self` - The iterator to be converted.
    /// * `len` - The known length of the iterator.
    ///
    /// # Returns
    ///
    /// A `TrustIter` wrapping the original iterator with the specified length.
    fn to_trust(self, len: usize) -> TrustIter<Self::IntoIter>;
}

impl<I: IntoIterator> ToTrustIter for I {
    fn to_trust(self, len: usize) -> TrustIter<Self::IntoIter> {
        TrustIter::new(self.into_iter(), len)
    }
}
/// A trait for collecting items from a trusted iterator into a collection.
///
/// This trait provides methods to efficiently collect items from iterators
/// that implement `TrustedLen`, allowing for optimized memory allocation
/// and item placement.
pub trait CollectTrusted<T> {
    /// Collects items from a trusted iterator into the implementing collection.
    ///
    /// This method assumes that the iterator's length is known and trusted,
    /// allowing for more efficient collection of items.
    ///
    /// # Arguments
    ///
    /// * `i` - An iterator with items of type `T` and implementing `TrustedLen`.
    ///
    /// # Returns
    ///
    /// The collection containing all items from the iterator.
    fn collect_from_trusted<I>(i: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: TrustedLen;

    /// Attempts to collect items from a trusted iterator that may produce errors.
    ///
    /// This method is similar to `collect_from_trusted`, but handles iterators
    /// that may produce `TResult<T>` items, allowing for error propagation.
    ///
    /// # Arguments
    ///
    /// * `i` - An iterator with items of type `TResult<T>` and implementing `TrustedLen`.
    ///
    /// # Returns
    ///
    /// A `TResult` containing either the successfully collected items or an error.
    fn try_collect_from_trusted<I, E: Error>(iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<T, E>>,
        I::IntoIter: TrustedLen,
        Self: Sized;
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

    /// safety: upper bound on the remaining length of the iterator must be correct.
    fn try_collect_from_trusted<I, E: Error>(iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<T, E>>,
        I::IntoIter: TrustedLen,
        Self: Sized,
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
                let v = v?;
                std::ptr::write(ptr, v);
                ptr = ptr.add(1);
            }
            vec.set_len(len);
        }
        Ok(vec)
    }
}

/// A trait for iterators that can be collected into a `Vec` with a trusted length.
///
/// This trait is implemented for all iterators that implement `TrustedLen`,
/// allowing for efficient collection into a `Vec` without unnecessary reallocations.
pub trait CollectTrustedToVec: Iterator + TrustedLen + Sized {
    /// Collects the iterator into a `Vec` using the trusted length information.
    ///
    /// This method is more efficient than the standard `collect()` method for
    /// iterators with a known length, as it can allocate the exact amount of
    /// memory needed upfront.
    ///
    /// # Returns
    ///
    /// A `Vec` containing all the items from the iterator.
    #[inline(always)]
    fn collect_trusted_to_vec(self) -> Vec<Self::Item> {
        CollectTrusted::<Self::Item>::collect_from_trusted(self)
    }
}

/// A trait for iterators that can be collected into a `Vec` with a trusted length,
/// where each item is a `Result`.
///
/// This trait is implemented for all iterators that implement `TrustedLen` and
/// yield `Result` items, allowing for efficient collection into a `Vec` while
/// propagating any errors encountered during iteration.
pub trait TryCollectTrustedToVec<T, E: Error>:
    Iterator<Item = Result<T, E>> + TrustedLen + Sized
{
    /// Attempts to collect the iterator into a `Vec` using the trusted length information.
    ///
    /// This method is more efficient than the standard `collect()` method for
    /// iterators with a known length, as it can allocate the exact amount of
    /// memory needed upfront. If any item in the iterator is an `Err`, the
    /// collection process is short-circuited and the error is returned.
    ///
    /// # Returns
    ///
    /// A `TResult` containing either:
    /// - `Ok(Vec<T>)`: A `Vec` containing all the successfully collected items.
    /// - `Err(E)`: The first error encountered during iteration.
    #[inline(always)]
    fn try_collect_trusted_to_vec(self) -> Result<Vec<T>, E> {
        CollectTrusted::<T>::try_collect_from_trusted(self)
    }
}

impl<T: TrustedLen> CollectTrustedToVec for T {}
impl<I: TrustedLen<Item = Result<T, E>> + Sized, T, E: Error> TryCollectTrustedToVec<T, E> for I {}
